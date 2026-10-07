// All native commands are simulated. This preview never launches a process.
(() => {
  let config = { preset: '-5', ttl: 'none', dns: 'cloudflare', customDnsIp: '1.1.1.1', customDnsPort: '53',
    extraArgs: '', language: 'fr', theme: 'light', isServiceMode: false };
  let running = false;
  const handlers = new Map();
  const emit = (name, payload) => { for (const handler of handlers.get(name) || []) handler({ payload }); };
  window.__TAURI__ = {
    event: {
      emit: async (name, payload) => emit(name, payload),
      listen: async (name, handler) => {
        if (!handlers.has(name)) handlers.set(name, new Set());
        handlers.get(name).add(handler);
        return () => handlers.get(name).delete(handler);
      }
    },
    core: { async invoke(command, args) {
      switch (command) {
        case 'load_config': return config;
        case 'save_config': config = args.config; return true;
        case 'check_status': return running;
        case 'check_is_admin': return true;
        case 'measure_ping': return 12;
        case 'start_bypass': running = true; emit('status-changed', running); return { success: true };
        case 'stop_bypass': running = false; emit('status-changed', running); return { success: true };
        case 'uninstall_service': running = false; emit('status-changed', false); return { success: true };
        case 'migrate_legacy_service': return { success: true, message: 'Simulation : ancien service retiré.' };
        case 'open_main_window': case 'close_window': case 'minimize_window': case 'quit_app': return { success: true };
        default: throw new Error(`Unknown preview command: ${command}`);
      }
    } }
  };
})();
