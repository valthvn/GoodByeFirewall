const { test } = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const vm = require('node:vm');

function loadTray(invoke) {
  const elements = new Map();
  const alerts = [];

  const context = vm.createContext({
    console, alert: message => alerts.push(message),
    document: {
      querySelector: () => null,
      body: { classList: { toggle() {} } },
      getElementById(id) {
        const element = { classList: { add() {}, remove() {}, toggle() {} },
          addEventListener(type, handler) { this[type] = handler; } };
        elements.set(id, element);
        return element;
      }
    },
    window: {
      addEventListener() {},
      __TAURI__: { core: { invoke } }
    }
  });
  vm.runInContext(fs.readFileSync('renderer/tray.js', 'utf8'), context);
  return { elements, alerts };
}

test('tray activation uses the latest saved configuration', async () => {
  let saved = { preset: '-5', dns: 'cloudflare' };
  let started;
  const { elements } = loadTray(async (command, args) => {
    if (command === 'load_config') return saved;
    if (command === 'check_status') return false;
    if (command === 'start_bypass') { started = args.config; return { success: true }; }
  });
  await new Promise(resolve => setImmediate(resolve));
  saved = { preset: '-9', dns: 'quad9' };
  const toggle = elements.get('trayToggleSwitch');
  toggle.checked = true;
  await toggle.change();
  assert.deepEqual(started, saved);
  assert.equal(toggle.disabled, false);
});

for (const starting of [true, false]) {
  test('tray failed ' + (starting ? 'start' : 'stop') + ' preserves actual active status', async () => {
    const { elements, alerts } = loadTray(async command => {
      if (command === 'load_config') return { preset: '-5', dns: 'cloudflare' };
      if (command === 'check_status') return true;
      if (command === 'start_bypass' || command === 'stop_bypass') return { success: false, message: 'Access denied' };
    });
    await new Promise(resolve => setImmediate(resolve));
    const toggle = elements.get('trayToggleSwitch');
    toggle.checked = starting;
    await toggle.change();
    assert.equal(toggle.checked, true);
    assert.equal(toggle.disabled, false);
    assert.deepEqual(alerts, ['Access denied']);
  });
}
