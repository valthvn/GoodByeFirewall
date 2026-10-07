const { test } = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const vm = require('node:vm');

test('tray activation uses the latest saved configuration', async () => {
  const elements = new Map();
  let saved = { preset: '-5', dns: 'cloudflare' };
  let started;
  const context = vm.createContext({
    console,
    document: {
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
      __TAURI__: { core: { async invoke(command, args) {
        if (command === 'load_config') return saved;
        if (command === 'check_status') return false;
        if (command === 'start_bypass') { started = args.config; return { success: true }; }
      } } }
    }
  });
  vm.runInContext(fs.readFileSync('renderer/tray.js', 'utf8'), context);
  await new Promise(resolve => setImmediate(resolve));
  saved = { preset: '-9', dns: 'quad9' };
  const toggle = elements.get('trayToggleSwitch');
  toggle.checked = true;
  await toggle.change();
  assert.deepEqual(started, saved);
  assert.equal(toggle.disabled, false);
});
