(function () {
  if (typeof window !== 'undefined' && window.__TAURI__) {
    const core = window.__TAURI__.core;
    const event = window.__TAURI__.event;

    window.api = {
      startBypass: (config) => core.invoke('start_bypass', { config }),
      stopBypass: () => core.invoke('stop_bypass'),
      checkStatus: () => core.invoke('check_status'),
      measurePing: (targetIp) => core.invoke('measure_ping', { targetIp: targetIp || '1.1.1.1' }),
      saveConfig: (config) => core.invoke('save_config', { config }),
      loadConfig: () => core.invoke('load_config'),
      uninstallService: () => core.invoke('uninstall_service'),
      migrateLegacyService: () => core.invoke('migrate_legacy_service'),
      minimizeWindow: () => core.invoke('minimize_window'),
      closeWindow: () => core.invoke('close_window'),
      checkIsAdmin: () => core.invoke('check_is_admin'),
      requestAdminElevation: () => core.invoke('request_admin_elevation'),
      quitApp: () => core.invoke('quit_app'),

      onStatusChange: (callback) => {
        if (event && event.listen) {
          return event.listen('status-changed', (evt) => callback(evt.payload));
        }
      },
      onLog: (callback) => {
        if (event && event.listen) {
          return event.listen('log-message', (evt) => callback(evt.payload));
        }
      }
    };
  }
})();
