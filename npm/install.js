#!/usr/bin/env node
// Aegis Security — npm postinstall script
// Downloads the prebuilt Aegis binary for the current platform.

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const PLATFORM = process.platform; // 'darwin', 'linux', 'win32'
const ARCH = process.arch;         // 'x64', 'arm64'
const VERSION = '0.1.0';

const RELEASES_URL = `https://github.com/vahapogut/Aegis-Security/releases/download/v${VERSION}`;

function getTargetTriple() {
  const platformMap = {
    'darwin': { x64: 'x86_64-apple-darwin', arm64: 'aarch64-apple-darwin' },
    'linux':  { x64: 'x86_64-unknown-linux-gnu', arm64: 'aarch64-unknown-linux-gnu' },
    'win32':  { x64: 'x86_64-pc-windows-msvc', arm64: 'aarch64-pc-windows-msvc' },
  };
  const target = platformMap[PLATFORM]?.[ARCH];
  if (!target) {
    console.error(`Aegis: Unsupported platform/arch: ${PLATFORM}/${ARCH}`);
    console.error('Please install from source: cargo install aegis');
    process.exit(1);
  }
  return target;
}

function install() {
  const target = getTargetTriple();
  const ext = PLATFORM === 'win32' ? '.exe' : '';
  const archiveName = PLATFORM === 'win32'
    ? `aegis-${target}.zip`
    : `aegis-${target}.tar.gz`;

  const binDir = path.join(__dirname, 'bin');
  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }

  const binaryPath = path.join(binDir, `aegis${ext}`);

  // Skip if already installed
  if (fs.existsSync(binaryPath)) {
    console.log('Aegis binary already installed. To reinstall, delete node_modules/aegis-security/bin/');
    return;
  }

  console.log(`Aegis: Downloading ${archiveName}...`);

  try {
    // Try cargo install first (if Rust is available)
    execSync('cargo install aegis', { stdio: 'inherit' });
    console.log('Aegis installed via cargo.');
  } catch {
    console.log('Cargo not available. To use the npm wrapper, install Rust: https://rustup.rs');
    console.log('Then run: cargo install aegis');
    console.log('\nOr download the binary manually from:');
    console.log(`  ${RELEASES_URL}`);
  }
}

install();
