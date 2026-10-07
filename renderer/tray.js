
(async function () {
  const core = window.__TAURI__?.core;
  const eventApi = window.__TAURI__?.event;

  const trayToggleSwitch = document.getElementById('trayToggleSwitch');
  const trayStatusBadge = document.getElementById('trayStatusBadge');
  const trayStatusText = document.getElementById('trayStatusText');
  const cardIconWrap = document.getElementById('cardIconWrap');
  const cardSubtitle = document.getElementById('cardSubtitle');
  const trayModeBadge = document.getElementById('trayModeBadge');
  const btnOpenApp = document.getElementById('btnOpenApp');
  const btnQuitApp = document.getElementById('btnQuitApp');

  let currentConfig = null;
  let isToggling = false;

  function applyTheme(theme) {
    const isDark = theme === 'dark';
    document.body.classList.toggle('dark-theme', isDark);
    document.body.classList.toggle('light-theme', !isDark);
  }

  function updateStatus(isRunning, config) {
    if (trayToggleSwitch) {
      trayToggleSwitch.checked = !!isRunning;
    }

    if (trayStatusBadge && trayStatusText) {
      if (isRunning) {
        trayStatusBadge.classList.add('active');
        trayStatusText.textContent = 'Protégé';
      } else {
        trayStatusBadge.classList.remove('active');
        trayStatusText.textContent = 'Inactif';
      }
    }

    if (cardIconWrap) {
      cardIconWrap.classList.toggle('active', !!isRunning);
    }

    if (cardSubtitle) {
      if (isRunning) {
        const p = config?.preset ? config.preset.replace('-', 'Niveau ') : 'Actif';
        const d = config?.dns ? (config.dns === 'cloudflare' ? 'Cloudflare' : config.dns) : '';
        cardSubtitle.textContent = `${p} • ${d}`;
      } else {
        cardSubtitle.textContent = 'Protection désactivée';
      }
    }

    if (trayModeBadge && config) {
      trayModeBadge.textContent = config.isServiceMode ? 'Service Windows' : 'Session Directe';
    }
  }

  async function loadState() {
    if (!core) return;
    try {
      const [cfg, isRunning] = await Promise.all([
        core.invoke('load_config'),
        core.invoke('check_status')
      ]);
      currentConfig = cfg;
      if (cfg && cfg.theme) {
        applyTheme(cfg.theme);
      }
      updateStatus(isRunning, currentConfig);
    } catch (e) {
      console.error('Failed to load tray state:', e);
    }
  }

  if (trayToggleSwitch) {
    trayToggleSwitch.addEventListener('change', async () => {
      if (!core || isToggling) return;
      isToggling = true;
      trayToggleSwitch.disabled = true;

      try {
        if (trayToggleSwitch.checked) {
          currentConfig = await core.invoke('load_config');
          if (currentConfig?.theme) applyTheme(currentConfig.theme);
          const res = await core.invoke('start_bypass', { config: currentConfig });
          if (!res || !res.success) {
            trayToggleSwitch.checked = false;
            updateStatus(false, currentConfig);
          } else {
            updateStatus(true, currentConfig);
          }
        } else {
          await core.invoke('stop_bypass');
          updateStatus(false, currentConfig);
        }
      } catch (err) {
        console.error('Toggle error:', err);
      } finally {
        trayToggleSwitch.disabled = false;
        isToggling = false;
      }
    });
  }

  if (btnOpenApp) {
    btnOpenApp.addEventListener('click', async () => {
      if (core) {
        await core.invoke('open_main_window');
      }
    });
  }

  if (btnQuitApp) {
    btnQuitApp.addEventListener('click', async () => {
      if (core) {
        await core.invoke('quit_app');
      }
    });
  }

  if (eventApi) {
    eventApi.listen('status-changed', async (event) => {
      if (!currentConfig && core) {
        try {
          currentConfig = await core.invoke('load_config');
          if (currentConfig?.theme) applyTheme(currentConfig.theme);
        } catch (_) {}
      }
      updateStatus(event.payload, currentConfig);
    });

    eventApi.listen('theme-changed', (event) => {
      if (event && event.payload) {
        applyTheme(event.payload);
        if (currentConfig) {
          currentConfig.theme = event.payload;
        }
      }
    });
  }

  window.addEventListener('focus', loadState);

  await loadState();
})();
