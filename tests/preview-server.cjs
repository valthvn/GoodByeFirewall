// Browser-only preview: the Tauri bridge is simulated; no daemon or service runs.
const http = require('node:http');
const fs = require('node:fs');
const path = require('node:path');
const root = path.resolve(__dirname, '../renderer');
const types = { '.html': 'text/html', '.js': 'text/javascript', '.css': 'text/css', '.woff2': 'font/woff2', '.png': 'image/png', '.ico': 'image/x-icon' };
const bridge = '<script src="/preview-bridge.js"></script>';
const csp = JSON.parse(fs.readFileSync(path.join(__dirname, '../src-tauri/tauri.conf.json'))).app.security.csp;
http.createServer((req, res) => {
  res.setHeader('Content-Security-Policy', csp);
  if (req.url === '/preview-bridge.js') {
    res.setHeader('Content-Type', 'text/javascript');
    res.end(fs.readFileSync(path.join(__dirname, 'preview-bridge.js')));
    return;
  }
  const file = path.resolve(root, '.' + new URL(req.url, 'http://localhost').pathname.replace(/\/$/, '/index.html'));
  if (!file.startsWith(root + path.sep)) { res.writeHead(403).end(); return; }
  fs.readFile(file, (err, data) => {
    if (err) { res.writeHead(404).end(); return; }
    res.setHeader('Content-Type', types[path.extname(file)] || 'application/octet-stream');
    res.end(path.extname(file) === '.html' ? data.toString().replace('<head>', '<head>' + bridge) : data);
  });
}).listen(4173, '127.0.0.1', () => console.log('Simulated Tauri preview: http://127.0.0.1:4173'));
