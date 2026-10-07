const { test } = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const vm = require('node:vm');

function loadRenderer() {
  const elements = new Map();
  let writes = 0;
  const timers = new Map();
  let timerId = 0;
  const saved = [];
  const alerts = [];
  const context = vm.createContext({
    console, Date, alert: message => alerts.push(message),
    setTimeout(fn) { timers.set(++timerId, fn); return timerId; },
    clearTimeout(id) { timers.delete(id); },
    requestAnimationFrame(fn) { timers.set(++timerId, fn); return timerId; },
    document: {
      body: { classList: { contains: () => false, add() {}, remove() {}, toggle() {} } },
      addEventListener() {},
      querySelectorAll: () => [],
      getElementById(id) {
        if (!elements.has(id)) {
          let content = '';
          elements.set(id, {
            value: '', scrollHeight: 100, scrollTop: 0,
            classList: { add() {}, remove() {}, toggle() {}, contains: () => false },
            style: {},
            setAttribute() {}, querySelectorAll: () => [], querySelector: () => null,
            get textContent() { return content; },
            set textContent(value) { content = value; if (id === 'terminalLogs') writes++; }
          });
        }
        return elements.get(id);
      }
    },
    window: { api: { saveConfig: async cfg => { saved.push(cfg); return true; } } }
  });
  vm.runInContext(fs.readFileSync('renderer/app.js', 'utf8'), context);
  return {
    context, elements, saved, alerts, writes: () => writes,
    async flush() {
      const pending = [...timers.values()]; timers.clear();
      for (const fn of pending) fn();
      await new Promise(resolve => setImmediate(resolve));
    }
  };
}

test('a burst of logs renders once and retains only the last 500 lines', async () => {
  const app = loadRenderer();
  vm.runInContext('for (let i = 0; i < 1000; i++) appendLog("line-" + i)', app.context);
  await app.flush();
  assert.equal(app.writes(), 1);
  const lines = app.elements.get('terminalLogs').textContent.split('\n');
  assert.equal(lines.length, 500);
  assert.match(lines[0], /line-500$/);
  assert.match(lines[499], /line-999$/);
});

test('failed stop preserves active status when the backend is still running', async () => {
  const app = loadRenderer();
  app.context.window.api.stopBypass = async () => ({ success: false, message: 'Access denied' });
  app.context.window.api.checkStatus = async () => true;
  await vm.runInContext('updateStatusUI(true); handlePowerToggle()', app.context);
  assert.equal(vm.runInContext('isRunning', app.context), true);
  assert.deepEqual(app.alerts, ['Access denied']);
});

test('failed start checks real status instead of hiding a still-active service', async () => {
  const app = loadRenderer();
  app.context.window.api.startBypass = async () => ({ success: false, message: 'Invalid DNS' });
  app.context.window.api.checkStatus = async () => true;
  await vm.runInContext('handlePowerToggle()', app.context);
  assert.equal(vm.runInContext('isRunning', app.context), true);
  assert.match(app.alerts[0], /Invalid DNS/);
});

test('rapid settings edits persist only the latest value', async () => {
  const app = loadRenderer();
  for (let i = 0; i < 20; i++) {
    app.elements.get('inputExtraArgs').value = `--value ${i}`;
    vm.runInContext('saveCurrentConfig()', app.context);
  }
  await app.flush();
  assert.equal(app.saved.length, 1);
  assert.equal(app.saved[0].extraArgs, '--value 19');
});

test('flushing with no pending edit does not block future saves', async () => {
  const app = loadRenderer();
  await vm.runInContext('flushConfigSave()', app.context);
  vm.runInContext('saveCurrentConfig()', app.context);
  await app.flush();
  assert.equal(app.saved.length, 1);
});

test('loading saved service mode never overwrites it while applying the language', async () => {
  const app = loadRenderer();
  vm.runInContext("applyConfigToUI({language: 'fr', isServiceMode: true})", app.context);
  await app.flush();
  assert.equal(app.saved.length, 0);
  assert.equal(vm.runInContext('getConfigFromUI().isServiceMode', app.context), true);
});

test('a failed save does not lose a newer pending edit', async () => {
  const app = loadRenderer();
  let rejectFirst;
  app.context.window.api.saveConfig = cfg => {
    app.saved.push(cfg);
    if (app.saved.length === 1) return new Promise((resolve, reject) => { rejectFirst = reject; });
    return Promise.resolve(true);
  };
  vm.runInContext('saveCurrentConfig()', app.context);
  await app.flush();
  app.elements.get('inputExtraArgs').value = '--after-failure';
  vm.runInContext('saveCurrentConfig()', app.context);
  await app.flush();
  rejectFirst(new Error('disk unavailable'));
  await new Promise(resolve => setImmediate(resolve));
  assert.equal(app.saved.length, 2);
  assert.equal(app.saved[1].extraArgs, '--after-failure');
});

test('settings writes never overlap and the latest edit survives an in-flight save', async () => {
  const app = loadRenderer();
  let release;
  app.context.window.api.saveConfig = cfg => {
    app.saved.push(cfg);
    return new Promise(resolve => { release = resolve; });
  };
  vm.runInContext('saveCurrentConfig()', app.context);
  // Start the first write without awaiting its unresolved promise.
  const first = app.flush();
  await new Promise(resolve => setImmediate(resolve));
  app.elements.get('inputExtraArgs').value = '--latest';
  vm.runInContext('saveCurrentConfig()', app.context);
  const second = app.flush();
  await new Promise(resolve => setImmediate(resolve));
  assert.equal(app.saved.length, 1);
  const finishFirst = release;
  finishFirst(true);
  await first;
  await second;
  await new Promise(resolve => setImmediate(resolve));
  assert.equal(app.saved.length, 2);
  assert.equal(app.saved[1].extraArgs, '--latest');
  release(true);
});
