const i18n = {
  fr: {
    title: 'GoodByeFirewall',
    statusConnected: 'PROTECTION ACTIVE',
    statusDisconnected: 'DÉCONNECTÉ / INACTIF',
    hintConnected: 'Votre trafic internet est sécurisé et le filtrage DPI contourné',
    hintDisconnected: 'Cliquez pour activer le contournement DPI et pare-feu',
    modeRunOnce: 'Session Rapide',
    modeService: 'Service Windows',
    switchEnabled: 'Activé',
    switchDisabled: 'Désactivé',
    cardDpiTitle: 'NIVEAU DPI',
    cardServiceTitle: 'SERVICE AUTO',
    cardDnsTitle: 'SERVEUR DNS',
    cardDnsSubtitle: 'Anti-censure résolveur',
    cardAdvancedSettingsTitle: 'PARAMÈTRES AVANCÉS',
    subAdvancedSettings: 'Tromperie DPI (TTL) & Options',
    lblTTL: 'TROMPERIE DPI (TTL / FAUX PAQUETS)',
    lblExtraArgs: 'Arguments Personnalisés Libres',
    cardInspectorTitle: 'CONSOLE D\'INSPECTION',
    cardAdvancedTitle: 'CONSOLE D\'INSPECTION',
    subInspector: 'Flux de télémétrie en direct',
    driverReady: 'Moteur WinDivert : Prêt',
    driverActive: 'Moteur WinDivert : Actif (Filtrage)',
    btnUninstall: 'Désinstaller Service',
    uninstallConfirm: 'Voulez-vous vraiment désinstaller le service GoodByeFirewall et WinDivert ?',
    uninstallSuccess: 'Service Windows et pilotes WinDivert désinstallés.',
    errStart: 'Erreur lors du démarrage : ',
    tipClose: 'Masquer dans les icônes cachées (Actif en arrière-plan)',
    tipMin: 'Réduire dans la barre des tâches',
    tipTheme: 'Changer de thème (Clair / Sombre)',
    tipPing: 'Latence réseau en direct',
    btnPowerActivate: 'Activer la protection',
    btnPowerDeactivate: 'Désactiver la protection',
    logStopping: '[Action] Arrêt de la protection...',
    logStarting: '[Action] Démarrage de GoodByeFirewall...',
    logHidden: '[Système] Fenêtre masquée. GoodByeFirewall reste actif dans les icônes cachées.',
    logThemeDark: '[Thème] Basculé vers le mode Sombre',
    logThemeLight: '[Thème] Basculé vers le mode Clair',
    logUninstalling: '[Service] Désinstallation demandée...',
    adminModalTitle: 'Autorisation Administrateur',
    adminModalDesc: 'Pour activer le contournement DPI et charger le pilote réseau sécurisé <strong>WinDivert</strong>, GoodByeFirewall requiert les privilèges Administrateur. Cette autorisation ne vous sera demandée qu\'une seule fois : GoodByeFirewall sera configuré pour s\'exécuter avec les privilèges requis sans vous redemander confirmation.',
    adminModalNote: 'Cliquez sur Autoriser pour accorder les privilèges et mémoriser l\'accès permanent.',
    adminBtnClose: 'Fermer',
    adminBtnAuthorize: 'Autoriser (UAC)',
    adminAuthorizing: 'Demande en cours...',
    adminSuccess: 'Autorisé avec succès ! Redémarrage...',
    adminDenied: 'Autorisation refusée ou annulée par l\'utilisateur.',
    placeholderExtraArgs: 'ex: -a -p --reverse-frag',
    presets: {
      '-5': 'Niveau 5 (Agressif - Recommandé)',
      '-1': 'Niveau 1 (Standard)',
      '-2': 'Niveau 2 (Quick ACK)',
      '-3': 'Niveau 3 (Sans fragmentation)',
      '-4': 'Niveau 4 (Ultra Rapide)',
      '-6': 'Niveau 6 (Avancé)',
      '-7': 'Niveau 7 (Avancé)',
      '-8': 'Niveau 8 (Avancé)',
      '-9': 'Niveau 9 (Avancé)',
      'custom': 'Arguments Libres...'
    },
    dns: {
      'cloudflare': 'Cloudflare (1.1.1.1)',
      'fdn': 'FDN (France - 80.67.169.12)',
      'adguard': 'AdGuard DNS (94.140.14.14)',
      'quad9': 'Quad9 (9.9.9.9)',
      'google': 'Google DNS (8.8.8.8)',
      'none': 'DNS Système (Par défaut)',
      'custom': 'Personnalisé...'
    },
    ttl: {
      'none': 'Désactivé (Recommandé)',
      '--auto-ttl': '--auto-ttl (Calcul Auto)',
      '--set-ttl 5': '--set-ttl 5',
      '--set-ttl 4': '--set-ttl 4',
      '--set-ttl 3': '--set-ttl 3',
      '--set-ttl 6': '--set-ttl 6'
    }
  },
  en: {
    title: 'GoodByeFirewall',
    statusConnected: 'PROTECTION ACTIVE',
    statusDisconnected: 'DISCONNECTED / INACTIVE',
    hintConnected: 'Your internet traffic is shielded against DPI inspection',
    hintDisconnected: 'Click to activate DPI & firewall bypass',
    modeRunOnce: 'Quick Session',
    modeService: 'Windows Service',
    switchEnabled: 'Enabled',
    switchDisabled: 'Disabled',
    cardDpiTitle: 'DPI LEVEL',
    cardServiceTitle: 'AUTO SERVICE',
    cardDnsTitle: 'DNS SERVER',
    cardDnsSubtitle: 'Anti-censorship resolver',
    cardAdvancedSettingsTitle: 'ADVANCED SETTINGS',
    subAdvancedSettings: 'DPI Spoofing (TTL) & Options',
    lblTTL: 'DPI SPOOFING (TTL / FAKE PACKETS)',
    lblExtraArgs: 'Custom Command Line Arguments',
    cardInspectorTitle: 'INSPECTOR CONSOLE',
    cardAdvancedTitle: 'INSPECTOR CONSOLE',
    subInspector: 'Live telemetry stream',
    driverReady: 'WinDivert Engine: Ready',
    driverActive: 'WinDivert Engine: Active (Filtering)',
    btnUninstall: 'Uninstall Service',
    uninstallConfirm: 'Do you really want to uninstall GoodByeFirewall and WinDivert services?',
    uninstallSuccess: 'Windows service and WinDivert drivers uninstalled.',
    errStart: 'Failed to start bypass: ',
    tipClose: 'Hide to notification tray (Active in background)',
    tipMin: 'Minimize to taskbar',
    tipTheme: 'Toggle theme (Light / Dark)',
    tipPing: 'Live network latency',
    btnPowerActivate: 'Activate protection',
    btnPowerDeactivate: 'Deactivate protection',
    logStopping: '[Action] Stopping protection...',
    logStarting: '[Action] Starting GoodByeFirewall...',
    logHidden: '[System] Window hidden. GoodByeFirewall remains active in system tray.',
    logThemeDark: '[Theme] Switched to Dark mode',
    logThemeLight: '[Theme] Switched to Light mode',
    logUninstalling: '[Service] Uninstall requested...',
    adminModalTitle: 'Administrator Privileges',
    adminModalDesc: 'To enable DPI bypass and load the secure <strong>WinDivert</strong> network driver, GoodByeFirewall requires Administrator privileges. This authorization is only required once: GoodByeFirewall will be configured to run with elevated privileges permanently.',
    adminModalNote: 'Click Grant Access to allow network privileges and memorize permanent access.',
    adminBtnClose: 'Close',
    adminBtnAuthorize: 'Grant Access (UAC)',
    adminAuthorizing: 'Requesting access...',
    adminSuccess: 'Access granted! Restarting...',
    adminDenied: 'Access was canceled or denied by the user.',
    placeholderExtraArgs: 'e.g. -a -p --reverse-frag',
    presets: {
      '-5': 'Level 5 (Aggressive - Recommended)',
      '-1': 'Level 1 (Standard)',
      '-2': 'Level 2 (Quick ACK)',
      '-3': 'Level 3 (No fragmentation)',
      '-4': 'Level 4 (Ultra Fast)',
      '-6': 'Level 6 (Advanced)',
      '-7': 'Level 7 (Advanced)',
      '-8': 'Level 8 (Advanced)',
      '-9': 'Level 9 (Advanced)',
      'custom': 'Custom Arguments...'
    },
    dns: {
      'cloudflare': 'Cloudflare (1.1.1.1)',
      'fdn': 'FDN (France - 80.67.169.12)',
      'adguard': 'AdGuard DNS (94.140.14.14)',
      'quad9': 'Quad9 (9.9.9.9)',
      'google': 'Google DNS (8.8.8.8)',
      'none': 'System DNS (Default)',
      'custom': 'Custom...'
    },
    ttl: {
      'none': 'Disabled (Recommended)',
      '--auto-ttl': '--auto-ttl (Auto Calculate)',
      '--set-ttl 5': '--set-ttl 5',
      '--set-ttl 4': '--set-ttl 4',
      '--set-ttl 3': '--set-ttl 3',
      '--set-ttl 6': '--set-ttl 6'
    }
  }
};

const btnPower = document.getElementById('btnPower');
const powerRing = document.getElementById('powerRing');
const statusBadge = document.getElementById('statusBadge');
const statusText = document.getElementById('statusText');
const statusHint = document.getElementById('statusHint');
const pingValue = document.getElementById('pingValue');
const pingBadge = document.getElementById('pingBadge');
const langSelect = document.getElementById('langSelect');
const iosServiceToggle = document.getElementById('iosServiceToggle');
const serviceSubtitle = document.getElementById('serviceSubtitle');
const switchStateText = document.getElementById('switchStateText');
const btnModeRunOnce = document.getElementById('btnModeRunOnce');
const btnModeService = document.getElementById('btnModeService');
const selectPreset = document.getElementById('selectPreset');
const presetSubtitle = document.getElementById('presetSubtitle');
const selectDNS = document.getElementById('selectDNS');

function updatePresetSubtitle(val) {
  if (presetSubtitle) {
    const dict = i18n[currentLang] || i18n.fr;
    presetSubtitle.textContent = (dict.presets && dict.presets[val]) || (val ? `Niveau / Level ${val}` : '');
  }
}
const customDnsRow = document.getElementById('customDnsRow');
const inputCustomIp = document.getElementById('inputCustomIp');
const inputCustomPort = document.getElementById('inputCustomPort');
const selectTTL = document.getElementById('selectTTL');
const inputExtraArgs = document.getElementById('inputExtraArgs');
const terminalToggle = document.getElementById('terminalToggle');
const terminalLogs = document.getElementById('terminalLogs');
const driverStatusText = document.getElementById('driverStatusText');
const btnUninstallService = document.getElementById('btnUninstallService');
const btnMin = document.getElementById('btnMin');
const btnClose = document.getElementById('btnClose');
const btnTheme = document.getElementById('btnTheme');
const iconSun = document.getElementById('iconSun');
const iconMoon = document.getElementById('iconMoon');
const waveCanvas = document.getElementById('waveCanvas');

const modalAdminPermission = document.getElementById('modalAdminPermission');
const btnAdminClose = document.getElementById('btnAdminClose');
const btnAdminAuthorize = document.getElementById('btnAdminAuthorize');
const btnAdminAuthorizeText = document.getElementById('btnAdminAuthorizeText');
const adminModalError = document.getElementById('adminModalError');

let isRunning = false;
let isServiceMode = false;
let currentLang = 'fr';
let pingInterval = null;

class LiquidAuroraEngine {
  constructor(canvas) {
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d');
    this.width = 0;
    this.height = 0;
    this.animId = null;
    this.lastTime = 0;
    this.elapsed = 0;

    this.active = false;
    this.intensity = 0; // 0 (idle) to 1 (active)
    this.targetIntensity = 0;
    this.isDark = document.body.classList.contains('dark-theme');

    this.surgeActive = false;
    this.surgeProgress = 0;
    this.surgeDuration = 700; // ms
    this.surgeStartTime = 0;

    this.initCanvasSize();
    window.addEventListener('resize', () => this.initCanvasSize());

    if (window.ResizeObserver && this.canvas.parentElement) {
      this.resizeObserver = new ResizeObserver(() => this.initCanvasSize());
      this.resizeObserver.observe(this.canvas.parentElement);
    }
  }

  setDark(isDark) {
    this.isDark = !!isDark;
  }

  initCanvasSize() {
    const parent = this.canvas.parentElement;
    const rect = parent ? parent.getBoundingClientRect() : this.canvas.getBoundingClientRect();
    const dpr = window.devicePixelRatio || 1;
    
    const w = Math.ceil(rect.width) || this.canvas.clientWidth || 448;
    const h = Math.ceil(rect.height) || this.canvas.clientHeight || 120;
    
    this.width = w;
    this.height = h;
    this.canvas.width = Math.round(w * dpr);
    this.canvas.height = Math.round(h * dpr);
    this.ctx.setTransform(1, 0, 0, 1, 0, 0);
    this.ctx.scale(dpr, dpr);
  }

  setActive(active) {
    const wasActive = this.active;
    this.active = !!active;
    this.targetIntensity = this.active ? 1.0 : 0.0;

    if (this.active && !wasActive) {
      this.surgeActive = true;
      this.surgeStartTime = performance.now();
      this.surgeProgress = 0;
    }
  }

  render(timestamp) {
    if (!this.lastTime) this.lastTime = timestamp;
    const delta = Math.min(timestamp - this.lastTime, 64);
    this.lastTime = timestamp;
    this.elapsed += delta * 0.001;

    const parent = this.canvas.parentElement;
    if (parent) {
      const pRect = parent.getBoundingClientRect();
      const targetW = Math.ceil(pRect.width);
      const targetH = Math.ceil(pRect.height);
      if (targetW > 0 && (Math.abs(targetW - this.width) >= 1 || Math.abs(targetH - this.height) >= 1)) {
        this.initCanvasSize();
      }
    }

    this.intensity += (this.targetIntensity - this.intensity) * Math.min(delta * 0.006, 0.2);

    if (this.surgeActive) {
      const surgeElapsed = timestamp - this.surgeStartTime;
      this.surgeProgress = Math.min(surgeElapsed / this.surgeDuration, 1.0);
      if (this.surgeProgress >= 1.0) {
        this.surgeActive = false;
      }
    }

    const isDark = this.isDark;
    const ctx = this.ctx;
    const w = this.width;
    const h = this.height;

    ctx.clearRect(-10, -10, w + 20, h + 20);

    const baseGrad = ctx.createLinearGradient(0, 0, w, h);
    if (isDark) {
      const idleAlpha = 0.06;
      const activeAlpha = 0.18 * this.intensity;
      baseGrad.addColorStop(0, `rgba(15, 23, 42, ${0.1 + idleAlpha})`);
      baseGrad.addColorStop(0.35, `rgba(30, 58, 138, ${0.05 * (1 - this.intensity)})`);
      baseGrad.addColorStop(0.7, `rgba(48, 209, 88, ${activeAlpha * 0.8})`);
      baseGrad.addColorStop(1, `rgba(50, 215, 230, ${activeAlpha})`);
    } else {
      const idleAlpha = 0.04;
      const activeAlpha = 0.13 * this.intensity;
      baseGrad.addColorStop(0, `rgba(255, 255, 255, 0.25)`);
      baseGrad.addColorStop(0.3, `rgba(0, 113, 227, ${idleAlpha * (1 - this.intensity)})`);
      baseGrad.addColorStop(0.65, `rgba(52, 199, 89, ${activeAlpha * 0.9})`);
      baseGrad.addColorStop(1, `rgba(48, 209, 88, ${activeAlpha})`);
    }

    ctx.fillStyle = baseGrad;
    ctx.fillRect(-5, -5, w + 10, h + 10);

    // Runs cleanly beyond both edges (from -16 to w + 16) to completely eliminate any edge cutoffs
    const speed1 = 1.1;
    const freq1 = 0.012;
    const amp1 = 8 + this.intensity * 7;
    const yBase1 = h * 0.52;
    const ext = 16; // extension beyond border

    const calcY1 = (x) => yBase1 + 
      Math.sin(x * freq1 + this.elapsed * speed1) * amp1 + 
      Math.cos(x * 0.007 - this.elapsed * 0.7) * (amp1 * 0.4);

    ctx.beginPath();
    ctx.moveTo(-ext, h + ext);
    ctx.lineTo(-ext, calcY1(-ext));

    const step = 4; // Finer step for silk-smooth curvature
    for (let x = -ext; x <= w + ext; x += step) {
      ctx.lineTo(x, calcY1(x));
    }
    ctx.lineTo(w + ext, calcY1(w + ext));
    ctx.lineTo(w + ext, h + ext);
    ctx.lineTo(-ext, h + ext);
    ctx.closePath();

    const waveGrad1 = ctx.createLinearGradient(0, 0, w, h);
    if (isDark) {
      waveGrad1.addColorStop(0, `rgba(48, 209, 88, ${0.08 * this.intensity})`);
      waveGrad1.addColorStop(0.5, `rgba(50, 215, 230, ${0.12 * this.intensity})`);
      waveGrad1.addColorStop(1, `rgba(48, 209, 88, ${0.18 * this.intensity})`);
    } else {
      waveGrad1.addColorStop(0, `rgba(0, 113, 227, ${0.04 * (1 - this.intensity)})`);
      waveGrad1.addColorStop(0.5, `rgba(52, 199, 89, ${0.10 * this.intensity})`);
      waveGrad1.addColorStop(1, `rgba(48, 209, 88, ${0.14 * this.intensity})`);
    }
    ctx.fillStyle = waveGrad1;
    ctx.fill();

    const speed2 = 0.85;
    const freq2 = 0.016;
    const amp2 = 6 + this.intensity * 5;
    const yBase2 = h * 0.70;

    const calcY2 = (x) => yBase2 + 
      Math.sin(x * freq2 - this.elapsed * speed2 + 2.2) * amp2 + 
      Math.cos(x * 0.009 + this.elapsed * 0.5) * (amp2 * 0.35);

    ctx.beginPath();
    ctx.moveTo(-ext, h + ext);
    ctx.lineTo(-ext, calcY2(-ext));

    for (let x = -ext; x <= w + ext; x += step) {
      ctx.lineTo(x, calcY2(x));
    }
    ctx.lineTo(w + ext, calcY2(w + ext));
    ctx.lineTo(w + ext, h + ext);
    ctx.lineTo(-ext, h + ext);
    ctx.closePath();

    const waveGrad2 = ctx.createLinearGradient(w, 0, 0, h);
    if (isDark) {
      waveGrad2.addColorStop(0, `rgba(50, 215, 230, ${0.07 * this.intensity})`);
      waveGrad2.addColorStop(1, `rgba(48, 209, 88, ${0.12 * this.intensity})`);
    } else {
      waveGrad2.addColorStop(0, `rgba(52, 199, 89, ${0.06 * this.intensity})`);
      waveGrad2.addColorStop(1, `rgba(0, 113, 227, ${0.03 * (1 - this.intensity)})`);
    }
    ctx.fillStyle = waveGrad2;
    ctx.fill();

    if (this.surgeActive) {
      const p = this.surgeProgress;
      const ease = 1 - Math.pow(1 - p, 3);
      const sweepWidth = w * 0.55;
      const sweepCenter = -sweepWidth + ease * (w + sweepWidth * 2);

      const surgeGrad = ctx.createLinearGradient(sweepCenter - sweepWidth * 0.5, 0, sweepCenter + sweepWidth * 0.5, 0);
      const peakAlpha = isDark ? 0.36 : 0.26;
      const fade = Math.sin(p * Math.PI); // fades in and out smoothly

      surgeGrad.addColorStop(0, 'rgba(52, 199, 89, 0)');
      surgeGrad.addColorStop(0.35, `rgba(50, 215, 230, ${peakAlpha * 0.65 * fade})`);
      surgeGrad.addColorStop(0.5, `rgba(52, 199, 89, ${peakAlpha * fade})`);
      surgeGrad.addColorStop(0.65, `rgba(160, 255, 180, ${peakAlpha * 0.85 * fade})`);
      surgeGrad.addColorStop(1, 'rgba(52, 199, 89, 0)');

      ctx.fillStyle = surgeGrad;
      ctx.fillRect(-5, -5, w + 10, h + 10);
    }

    const specularGrad = ctx.createLinearGradient(0, 0, w, 0);
    const specAlpha = isDark ? (0.22 + 0.28 * this.intensity) : (0.40 + 0.25 * this.intensity);
    specularGrad.addColorStop(0, `rgba(255, 255, 255, 0)`);
    specularGrad.addColorStop(0.3 + 0.1 * Math.sin(this.elapsed), `rgba(255, 255, 255, ${specAlpha})`);
    specularGrad.addColorStop(0.7 + 0.1 * Math.cos(this.elapsed * 0.8), `rgba(52, 199, 89, ${specAlpha * this.intensity})`);
    specularGrad.addColorStop(1, `rgba(255, 255, 255, 0)`);

    ctx.fillStyle = specularGrad;
    ctx.fillRect(0, 0, w, 1.5);

    this.animId = requestAnimationFrame((ts) => this.render(ts));
  }

  start() {
    if (!this.animId) {
      this.lastTime = performance.now();
      this.animId = requestAnimationFrame((ts) => this.render(ts));
    }
  }

  pause() {
    if (this.animId) {
      cancelAnimationFrame(this.animId);
      this.animId = null;
    }
  }

  resume() {
    if (!this.animId) {
      this.lastTime = performance.now();
      this.animId = requestAnimationFrame((ts) => this.render(ts));
    }
  }
}

let waveEngine = null;

function applyLanguage(lang, persist = true) {
  currentLang = (lang === 'en') ? 'en' : 'fr';
  const dict = i18n[currentLang] || i18n.fr;

  document.querySelectorAll('[data-i18n]').forEach((el) => {
    const key = el.getAttribute('data-i18n');
    if (dict[key]) {
      if (dict[key].includes('<')) {
        el.innerHTML = dict[key];
      } else {
        el.textContent = dict[key];
      }
    }
  });

  if (btnClose) {
    btnClose.title = dict.tipClose;
    btnClose.setAttribute('aria-label', dict.tipClose);
  }
  if (btnMin) {
    btnMin.title = dict.tipMin;
    btnMin.setAttribute('aria-label', dict.tipMin);
  }
  if (btnTheme) {
    btnTheme.title = dict.tipTheme;
    btnTheme.setAttribute('aria-label', dict.tipTheme);
  }
  if (pingBadge) {
    pingBadge.title = dict.tipPing;
  }
  if (inputExtraArgs) {
    inputExtraArgs.placeholder = dict.placeholderExtraArgs;
  }

  if (selectPreset) {
    selectPreset.querySelectorAll('option').forEach(opt => {
      const v = opt.value;
      if (dict.presets && dict.presets[v]) opt.textContent = dict.presets[v];
    });
    const menuPreset = document.getElementById('menuPreset');
    if (menuPreset) {
      menuPreset.querySelectorAll('.custom-option').forEach(opt => {
        const v = opt.getAttribute('data-value');
        if (dict.presets && dict.presets[v]) opt.textContent = dict.presets[v];
      });
    }
    updatePresetSubtitle(selectPreset.value);
    syncCustomDropdown('containerPreset', selectPreset.value);
  }

  if (selectDNS) {
    selectDNS.querySelectorAll('option').forEach(opt => {
      const v = opt.value;
      if (dict.dns && dict.dns[v]) opt.textContent = dict.dns[v];
    });
    const menuDNS = document.getElementById('menuDNS');
    if (menuDNS) {
      menuDNS.querySelectorAll('.custom-option').forEach(opt => {
        const v = opt.getAttribute('data-value');
        if (dict.dns && dict.dns[v]) opt.textContent = dict.dns[v];
      });
    }
    syncCustomDropdown('containerDNS', selectDNS.value);
  }

  if (selectTTL) {
    selectTTL.querySelectorAll('option').forEach(opt => {
      const v = opt.value;
      if (dict.ttl && dict.ttl[v]) opt.textContent = dict.ttl[v];
    });
    const menuTTL = document.getElementById('menuTTL');
    if (menuTTL) {
      menuTTL.querySelectorAll('.custom-option').forEach(opt => {
        const v = opt.getAttribute('data-value');
        if (dict.ttl && dict.ttl[v]) opt.textContent = dict.ttl[v];
      });
    }
    syncCustomDropdown('containerTTL', selectTTL.value);
  }

  const labelLang = document.getElementById('labelLang');
  if (labelLang) {
    labelLang.textContent = currentLang.toUpperCase();
  }
  syncCustomDropdown('containerLang', currentLang);

  setMode(isServiceMode);
  updateStatusUI(isRunning);

  if (persist) saveCurrentConfig();
}

function updateStatusUI(active) {
  isRunning = active;
  const dict = i18n[currentLang] || i18n.fr;

  if (waveEngine) {
    waveEngine.setActive(active);
  }

  if (btnPower) {
    btnPower.setAttribute('aria-label', active ? dict.btnPowerDeactivate : dict.btnPowerActivate);
  }

  if (active) {
    document.body.classList.add('shield-active');
    statusBadge.classList.remove('disconnected');
    statusBadge.classList.add('connected');
    statusText.textContent = dict.statusConnected;
    statusHint.textContent = dict.hintConnected;
    driverStatusText.textContent = dict.driverActive;
  } else {
    document.body.classList.remove('shield-active');
    statusBadge.classList.remove('connected');
    statusBadge.classList.add('disconnected');
    statusText.textContent = dict.statusDisconnected;
    statusHint.textContent = dict.hintDisconnected;
    driverStatusText.textContent = dict.driverReady;
  }
}

function getActiveDnsIp() {
  const dns = selectDNS ? selectDNS.value : 'cloudflare';
  switch (dns) {
    case 'cloudflare': return '1.1.1.1';
    case 'fdn': return '80.67.169.12';
    case 'adguard': return '94.140.14.14';
    case 'google': return '8.8.8.8';
    case 'quad9': return '9.9.9.9';
    case 'custom': return (inputCustomIp && inputCustomIp.value.trim()) || '1.1.1.1';
    case 'none':
    default:
      return '1.1.1.1';
  }
}

let isPinging = false;
async function checkPing() {
  // Don't waste network connections or CPU when window is hidden/minimized
  if (isPinging || document.hidden || (typeof document.hasFocus === 'function' && !document.hasFocus())) return;
  isPinging = true;
  try {
    const targetIp = getActiveDnsIp();
    const ms = await window.api.measurePing(targetIp);
    if (ms !== null && ms !== undefined) {
      pingValue.textContent = `${ms} ms`;
      if (ms < 45) {
        pingBadge.style.color = '#15803d';
      } else if (ms < 110) {
        pingBadge.style.color = '#d97706';
      } else {
        pingBadge.style.color = '#dc2626';
      }
    } else {
      pingValue.textContent = '-- ms';
      pingBadge.style.color = 'var(--text-secondary)';
    }
  } catch (e) {
    pingValue.textContent = '-- ms';
  } finally {
    isPinging = false;
  }
}

function setTheme(theme) {
  const isDark = theme === 'dark';
  document.body.classList.toggle('dark-theme', isDark);
  if (iconSun && iconMoon) {
    iconSun.style.display = isDark ? 'none' : 'block';
    iconMoon.style.display = isDark ? 'block' : 'none';
  }
  if (waveEngine) {
    waveEngine.setDark(isDark);
  }
  try {
    localStorage.setItem('gbf_theme', theme);
  } catch (_) {}
  if (window.__TAURI__?.event?.emit) {
    window.__TAURI__.event.emit('theme-changed', theme);
  }
}

function syncCustomDropdown(containerId, value) {
  const container = document.getElementById(containerId);
  if (!container) return;
  const label = container.querySelector('.trigger-label');
  const options = container.querySelectorAll('.custom-option');
  let matched = false;

  options.forEach(opt => {
    if (opt.getAttribute('data-value') === value) {
      opt.classList.add('selected');
      if (label) {
        if (containerId === 'containerLang') {
          label.textContent = value.toUpperCase();
        } else {
          label.textContent = opt.textContent.trim();
        }
      }
      matched = true;
    } else {
      opt.classList.remove('selected');
    }
  });

  if (!matched && label) {
    label.textContent = containerId === 'containerLang' ? value.toUpperCase() : value;
  }
}

function initCustomDropdowns() {
  const dropdowns = [
    { containerId: 'containerPreset', triggerId: 'triggerPreset', selectId: 'selectPreset' },
    { containerId: 'containerDNS', triggerId: 'triggerDNS', selectId: 'selectDNS' },
    { containerId: 'containerTTL', triggerId: 'triggerTTL', selectId: 'selectTTL' },
    { containerId: 'containerLang', triggerId: 'triggerLang', selectId: 'langSelect' }
  ];

  dropdowns.forEach(({ containerId, triggerId, selectId }) => {
    const container = document.getElementById(containerId);
    const trigger = document.getElementById(triggerId);
    const select = document.getElementById(selectId);

    if (!container || !trigger || !select) return;

    trigger.setAttribute('role', 'combobox');
    trigger.setAttribute('aria-expanded', 'false');
    trigger.setAttribute('aria-haspopup', 'listbox');
    const menu = container.querySelector('.custom-select-menu');
    if (menu) menu.setAttribute('role', 'listbox');

    trigger.addEventListener('click', (e) => {
      e.stopPropagation();
      const wasOpen = container.classList.contains('open');

      document.querySelectorAll('.custom-select-container.open').forEach(c => {
        if (c !== container) {
          c.classList.remove('open');
          c.closest('.bento-card')?.classList.remove('dropdown-active');
          const tr = c.querySelector('.custom-select-trigger');
          if (tr) tr.setAttribute('aria-expanded', 'false');
        }
      });

      if (!wasOpen) {
        // Smart flip: dynamically check available space below and above to stay in rectangle
        const rect = trigger.getBoundingClientRect();
        const spaceBelow = window.innerHeight - rect.bottom - 12;
        const spaceAbove = rect.top - 12;

        if (spaceBelow >= 150 || spaceBelow >= spaceAbove) {
          container.classList.add('open-down');
          if (menu) {
            menu.style.maxHeight = `${Math.min(180, spaceBelow)}px`;
          }
        } else {
          container.classList.remove('open-down');
          if (menu) {
            menu.style.maxHeight = `${Math.min(180, spaceAbove)}px`;
          }
        }

        container.classList.add('open');
        container.closest('.bento-card')?.classList.add('dropdown-active');
        trigger.setAttribute('aria-expanded', 'true');
      } else {
        container.classList.remove('open');
        container.closest('.bento-card')?.classList.remove('dropdown-active');
        trigger.setAttribute('aria-expanded', 'false');
      }
    });

    trigger.addEventListener('keydown', (e) => {
      if (e.key === 'ArrowDown' || e.key === 'Enter' || e.key === ' ') {
        e.preventDefault();
        if (!container.classList.contains('open')) {
          trigger.click();
        }
        const firstSelected = container.querySelector('.custom-option.selected') || container.querySelector('.custom-option');
        if (firstSelected) firstSelected.focus();
      }
    });

    const options = container.querySelectorAll('.custom-option');
    options.forEach(opt => {
      opt.setAttribute('role', 'option');
      opt.setAttribute('tabindex', '-1');

      opt.addEventListener('click', (e) => {
        e.stopPropagation();
        const val = opt.getAttribute('data-value');

        select.value = val;
        select.dispatchEvent(new Event('change'));

        syncCustomDropdown(containerId, val);

        container.classList.remove('open');
        container.closest('.bento-card')?.classList.remove('dropdown-active');
        trigger.setAttribute('aria-expanded', 'false');
      });

      opt.addEventListener('keydown', (e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          opt.click();
          trigger.focus();
        } else if (e.key === 'ArrowDown') {
          e.preventDefault();
          const next = opt.nextElementSibling;
          if (next && next.classList.contains('custom-option')) next.focus();
        } else if (e.key === 'ArrowUp') {
          e.preventDefault();
          const prev = opt.previousElementSibling;
          if (prev && prev.classList.contains('custom-option')) {
            prev.focus();
          } else {
            trigger.focus();
          }
        } else if (e.key === 'Escape') {
          e.preventDefault();
          container.classList.remove('open');
          container.closest('.bento-card')?.classList.remove('dropdown-active');
          trigger.setAttribute('aria-expanded', 'false');
          trigger.focus();
        }
      });
    });
  });

  document.addEventListener('click', () => {
    document.querySelectorAll('.custom-select-container.open').forEach(c => {
      c.classList.remove('open');
      c.closest('.bento-card')?.classList.remove('dropdown-active');
      const tr = c.querySelector('.custom-select-trigger');
      if (tr) tr.setAttribute('aria-expanded', 'false');
    });
  });

  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
      document.querySelectorAll('.custom-select-container.open').forEach(c => {
        c.classList.remove('open');
        c.closest('.bento-card')?.classList.remove('dropdown-active');
        const tr = c.querySelector('.custom-select-trigger');
        if (tr) tr.setAttribute('aria-expanded', 'false');
      });
    }
  });
}

function getConfigFromUI() {
  return {
    preset: (selectPreset && selectPreset.value) ? selectPreset.value : '-5',
    ttl: selectTTL ? selectTTL.value : 'none',
    dns: selectDNS ? selectDNS.value : 'cloudflare',
    customDnsIp: inputCustomIp ? inputCustomIp.value.trim() : '1.1.1.1',
    customDnsPort: inputCustomPort ? inputCustomPort.value.trim() : '53',
    extraArgs: inputExtraArgs ? inputExtraArgs.value.trim() : '',
    language: currentLang,
    theme: document.body.classList.contains('dark-theme') ? 'dark' : 'light',
    isServiceMode: isServiceMode
  };
}

let configSaveTimer = null;
let pendingConfig = null;
let configSavePromise = null;

function flushConfigSave() {
  clearTimeout(configSaveTimer);
  configSaveTimer = null;
  if (configSavePromise) return configSavePromise;
  if (!pendingConfig) return Promise.resolve();
  configSavePromise = (async () => {
    try {
      while (pendingConfig) {
        const cfg = pendingConfig;
        pendingConfig = null;
        try {
          if (!await window.api.saveConfig(cfg)) {
            appendLog('[Config] Impossible de sauvegarder les paramètres.');
          }
        } catch (err) {
          appendLog(`[Config] ${err.message}`);
        }
      }
    } finally {
      configSavePromise = null;
    }
  })();
  return configSavePromise;
}

function saveCurrentConfig() {
  pendingConfig = getConfigFromUI();
  clearTimeout(configSaveTimer);
  configSaveTimer = setTimeout(flushConfigSave, 250);
}

function applyConfigToUI(cfg) {
  if (!cfg) return;

  if (cfg.theme) {
    setTheme(cfg.theme);
  }

  if (selectPreset) {
    selectPreset.value = cfg.preset || '-5';
    updatePresetSubtitle(selectPreset.value);
    syncCustomDropdown('containerPreset', selectPreset.value);
  }

  if (cfg.ttl) {
    selectTTL.value = cfg.ttl;
    syncCustomDropdown('containerTTL', cfg.ttl);
  }

  if (cfg.dns) {
    if (cfg.dns === 'yandex') {
      cfg.dns = 'cloudflare';
    }
    selectDNS.value = cfg.dns;
    syncCustomDropdown('containerDNS', selectDNS.value);
    customDnsRow.classList.toggle('hidden', cfg.dns !== 'custom');
  }

  if (cfg.customDnsIp) inputCustomIp.value = cfg.customDnsIp;
  if (cfg.customDnsPort) inputCustomPort.value = cfg.customDnsPort;
  if (cfg.extraArgs) inputExtraArgs.value = cfg.extraArgs;

  if (cfg.language) {
    currentLang = cfg.language;
    langSelect.value = cfg.language;
    applyLanguage(cfg.language, false);
    syncCustomDropdown('containerLang', cfg.language);
  }

  if (cfg.isServiceMode !== undefined) {
    setMode(cfg.isServiceMode);
  }
}

function setMode(serviceMode) {
  isServiceMode = serviceMode;
  const dict = i18n[currentLang] || i18n.fr;
  if (iosServiceToggle) {
    iosServiceToggle.checked = serviceMode;
  }
  if (serviceSubtitle) {
    serviceSubtitle.textContent = serviceMode ? dict.modeService : dict.modeRunOnce;
  }
  if (switchStateText) {
    switchStateText.textContent = serviceMode ? dict.switchEnabled : dict.switchDisabled;
    switchStateText.style.color = serviceMode ? 'var(--apple-green)' : 'var(--text-secondary)';
  }
  if (btnModeService && btnModeRunOnce) {
    btnModeService.textContent = dict.modeService;
    btnModeRunOnce.textContent = dict.modeRunOnce;
    btnModeService.classList.toggle('active', serviceMode);
    btnModeRunOnce.classList.toggle('active', !serviceMode);
  }
}

const MAX_LOG_LINES = 500;
const logBuffer = [];
let logRenderTimer = null;

function appendLog(message) {
  const time = new Date().toLocaleTimeString();
  logBuffer.push(`[${time}] ${message}`);
  if (logBuffer.length > MAX_LOG_LINES) {
    logBuffer.shift();
  }
  if (logRenderTimer === null) {
    logRenderTimer = setTimeout(() => {
      logRenderTimer = null;
      terminalLogs.textContent = logBuffer.join('\n');
      terminalLogs.scrollTop = terminalLogs.scrollHeight;
    }, 100);
  }
}

function triggerPowerBounce() {
  if (!btnPower) return;
  btnPower.classList.remove('tactile-bounce');
  void btnPower.offsetWidth;
  btnPower.classList.add('tactile-bounce');
}

let isTogglingPower = false;

async function handlePowerToggle() {
  if (isTogglingPower) return;
  isTogglingPower = true;
  triggerPowerBounce();
  const dict = i18n[currentLang] || i18n.fr;

  try {
    if (isRunning) {
      appendLog(dict.logStopping);
      await window.api.stopBypass();
      updateStatusUI(false);
    } else {
      appendLog(dict.logStarting);
      const cfg = getConfigFromUI();
      await flushConfigSave();
      const res = await window.api.startBypass(cfg);
      if (!res.success) {
        appendLog(`[ERREUR / ERROR] ${res.message}`);
        alert(dict.errStart + res.message);
        updateStatusUI(false);
      } else {
        updateStatusUI(true);
      }
    }
  } catch (err) {
    appendLog(`[Exception] ${err.message}`);
  } finally {
    isTogglingPower = false;
  }
}

async function initAdminPermissionCheck() {
  if (!window.api || !window.api.checkIsAdmin) return;
  try {
    const isAdmin = await window.api.checkIsAdmin();
    if (!isAdmin && modalAdminPermission) {
      modalAdminPermission.style.display = 'flex';
    }
  } catch (err) {
    console.error('Failed to check admin status:', err);
  }

  if (btnAdminClose) {
    btnAdminClose.addEventListener('click', async () => {
      await flushConfigSave();
      if (window.api && window.api.quitApp) {
        window.api.quitApp();
      } else {
        window.close();
      }
    });
  }

  if (btnAdminAuthorize) {
    btnAdminAuthorize.addEventListener('click', async () => {
      btnAdminAuthorize.disabled = true;
      if (btnAdminClose) btnAdminClose.disabled = true;
      const dict = i18n[currentLang] || i18n.fr;
      if (btnAdminAuthorizeText) btnAdminAuthorizeText.textContent = dict.adminAuthorizing;
      if (adminModalError) adminModalError.style.display = 'none';

      try {
        await flushConfigSave();
        const res = await window.api.requestAdminElevation();
        if (res && res.success) {
          if (btnAdminAuthorizeText) btnAdminAuthorizeText.textContent = dict.adminSuccess;
          btnAdminAuthorize.style.background = 'var(--apple-green)';
          setTimeout(() => {
            if (modalAdminPermission) modalAdminPermission.style.display = 'none';
          }, 1200);
        } else {
          btnAdminAuthorize.disabled = false;
          if (btnAdminClose) btnAdminClose.disabled = false;
          if (btnAdminAuthorizeText) btnAdminAuthorizeText.textContent = dict.adminBtnAuthorize;
          if (adminModalError) {
            adminModalError.textContent = res?.message || dict.adminDenied;
            adminModalError.style.display = 'block';
          }
        }
      } catch (e) {
        btnAdminAuthorize.disabled = false;
        if (btnAdminClose) btnAdminClose.disabled = false;
        if (btnAdminAuthorizeText) btnAdminAuthorizeText.textContent = dict.adminBtnAuthorize;
        if (adminModalError) {
          adminModalError.textContent = e.message || dict.adminDenied;
          adminModalError.style.display = 'block';
        }
      }
    });
  }
}

async function init() {
  if (waveCanvas) {
    waveEngine = new LiquidAuroraEngine(waveCanvas);
    waveEngine.start();
  }

  const savedCfg = await window.api.loadConfig();
  applyConfigToUI(savedCfg);

  await initAdminPermissionCheck();

  const initialStatus = await window.api.checkStatus();
  updateStatusUI(initialStatus);

  window.api.onStatusChange((status) => {
    updateStatusUI(status);
  });

  window.api.onLog((log) => {
    appendLog(log);
  });

  checkPing();
  pingInterval = setInterval(checkPing, 2500);

  // Handle visibility and focus transitions smoothly (prevents occlusion lag)
  document.addEventListener('visibilitychange', () => {
    if (document.hidden) {
      if (waveEngine) waveEngine.pause();
    } else {
      if (waveEngine) waveEngine.resume();
      checkPing();
    }
  });

  window.addEventListener('focus', () => {
    if (waveEngine) waveEngine.resume();
    checkPing();
  });

  window.addEventListener('blur', () => {
    flushConfigSave();
    if (waveEngine) waveEngine.pause();
  });

  btnPower.addEventListener('click', handlePowerToggle);
  btnPower.addEventListener('animationend', (e) => {
    if (e.animationName === 'tactileSpringBounce') {
      btnPower.classList.remove('tactile-bounce');
    }
  });

  initCustomDropdowns();

  if (btnTheme) {
    btnTheme.addEventListener('click', () => {
      const isDark = document.body.classList.contains('dark-theme');
      const newTheme = isDark ? 'light' : 'dark';
      setTheme(newTheme);
      saveCurrentConfig();
      const dict = i18n[currentLang] || i18n.fr;
      appendLog(newTheme === 'dark' ? dict.logThemeDark : dict.logThemeLight);
    });
  }

  langSelect.addEventListener('change', (e) => {
    applyLanguage(e.target.value);
    syncCustomDropdown('containerLang', e.target.value);
  });

  if (iosServiceToggle) {
    iosServiceToggle.addEventListener('change', (e) => {
      setMode(e.target.checked);
      saveCurrentConfig();
      const dict = i18n[currentLang] || i18n.fr;
      appendLog(`[Mode] ${e.target.checked ? dict.modeService : dict.modeRunOnce}`);
    });
  }

  if (selectPreset) {
    selectPreset.addEventListener('change', () => {
      updatePresetSubtitle(selectPreset.value);
      syncCustomDropdown('containerPreset', selectPreset.value);
      if (selectPreset.value === 'custom') {
        const advancedDrawer = document.querySelector('.advanced-drawer-card');
        if (advancedDrawer) {
          advancedDrawer.classList.add('expanded');
        }
        inputExtraArgs.focus();
      }
      saveCurrentConfig();
      const selectedLabel = selectPreset.options[selectPreset.selectedIndex]?.text || selectPreset.value;
      appendLog(`[${currentLang === 'en' ? 'DPI Level' : 'Niveau DPI'}] ${selectedLabel}`);
    });
  }

  selectDNS.addEventListener('change', () => {
    syncCustomDropdown('containerDNS', selectDNS.value);
    customDnsRow.classList.toggle('hidden', selectDNS.value !== 'custom');
    saveCurrentConfig();
  });

  selectTTL.addEventListener('change', () => {
    syncCustomDropdown('containerTTL', selectTTL.value);
    saveCurrentConfig();
  });
  inputExtraArgs.addEventListener('input', saveCurrentConfig);
  inputCustomIp.addEventListener('input', saveCurrentConfig);
  inputCustomPort.addEventListener('input', saveCurrentConfig);

  const advancedToggle = document.getElementById('advancedToggle');
  if (advancedToggle) {
    advancedToggle.addEventListener('click', () => {
      const advancedCard = document.querySelector('.advanced-drawer-card');
      const terminalDrawer = document.querySelector('.terminal-drawer-card');
      if (advancedCard) {
        const isExpanding = !advancedCard.classList.contains('expanded');
        advancedCard.classList.toggle('expanded');
        if (isExpanding && terminalDrawer) {
          terminalDrawer.classList.remove('expanded');
        }
      }
    });
  }

  terminalToggle.addEventListener('click', () => {
    const terminalDrawer = document.querySelector('.terminal-drawer-card');
    const advancedCard = document.querySelector('.advanced-drawer-card');
    if (terminalDrawer) {
      const isExpanding = !terminalDrawer.classList.contains('expanded');
      terminalDrawer.classList.toggle('expanded');
      if (isExpanding && advancedCard) {
        advancedCard.classList.remove('expanded');
      }
    }
  });

  btnMin.addEventListener('click', () => {
    window.api.minimizeWindow();
  });

  btnClose.addEventListener('click', () => {
    const dict = i18n[currentLang] || i18n.fr;
    appendLog(dict.logHidden);
    flushConfigSave();
    window.api.closeWindow();
  });

  btnUninstallService.addEventListener('click', async () => {
    const dict = i18n[currentLang] || i18n.fr;
    if (confirm(dict.uninstallConfirm)) {
      appendLog(dict.logUninstalling);
      const res = await window.api.uninstallService();
      updateStatusUI(false);
      if (res && res.success === false) {
        alert(res.message || 'Erreur lors de la désinstallation du service.');
      } else {
        alert(dict.uninstallSuccess);
      }
    }
  });

  // Strictly prevent any window-level scrolling or sliding
  window.addEventListener('wheel', (e) => {
    // Only permit wheel scrolling inside scrollable areas (console log flow and open dropdown menu)
    const isScrollable = e.target.closest('.console-log-flow, .custom-select-menu');
    if (!isScrollable) {
      e.preventDefault();
    }
  }, { passive: false });

  // Prevent keyboard navigation keys (Space, PageUp, PageDown, Arrows) from scrolling the window
  window.addEventListener('keydown', (e) => {
    if (['Space', 'PageUp', 'PageDown'].includes(e.code)) {
      const isInput = e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA';
      if (!isInput) {
        e.preventDefault();
      }
    }
  });

  // Zero-out any accidental programmatic or browser scroll offset
  window.addEventListener('scroll', () => {
    window.scrollTo(0, 0);
    document.body.scrollTop = 0;
    document.documentElement.scrollTop = 0;
  });

  // Suspend GPU animation when window is hidden or minimized to system tray
  document.addEventListener('visibilitychange', () => {
    if (waveEngine) {
      if (document.hidden) {
        waveEngine.pause();
      } else {
        waveEngine.start();
      }
    }
  });

  window.addEventListener('blur', () => {
    if (waveEngine) {
      waveEngine.pause();
    }
  });

  window.addEventListener('focus', () => {
    if (waveEngine && !document.hidden) {
      waveEngine.start();
    }
  });
}

document.addEventListener('DOMContentLoaded', init);
