#!/usr/bin/env node
// Aegis Security — npm binary wrapper
// Falls back to cargo-installed aegis, then to local bin/aegis.

const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

const args = process.argv.slice(2).join(' ');
const localBin = path.join(__dirname, 'bin', process.platform === 'win32' ? 'aegis.exe' : 'aegis');

function run() {
  // Try cargo-installed aegis first
  try {
    execSync(`aegis ${args}`, { stdio: 'inherit' });
    return;
  } catch {
    // Not found via cargo, try local binary
  }

  // Try local binary
  if (fs.existsSync(localBin)) {
    try {
      execSync(`"${localBin}" ${args}`, { stdio: 'inherit' });
      return;
    } catch (e) {
      process.exit(e.status || 1);
    }
  }

  console.error('Aegis not found. Install it via: cargo install aegis');
  console.error('Or visit: https://github.com/vahapogut/Aegis-Security');
  process.exit(1);
}

run();
