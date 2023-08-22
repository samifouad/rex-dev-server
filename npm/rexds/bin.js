#!/usr/bin/env node

const { execSync } = require('child_process');
const os = require('os');

let binaryName = 'rexds';
if (os.platform() === 'win32') {
    binaryName += '.exe';
}

let binaryPath;

switch (os.platform()) {
    case 'darwin':
        binaryPath = os.arch() === 'x64'
            ? `./node_modules/rexds-darwin-x64/${binaryName}`
            : `./node_modules/rexds-darwin-arm64/${binaryName}`;
        break;
    case 'win32':
        binaryPath = `./node_modules/rexds-windows-x64/${binaryName}`;
        break;
    case 'linux':
        binaryPath = `./node_modules/rexds-linux-x64-musl/${binaryName}`;
    default:
        console.error(`Unsupported platform: ${os.platform()}`);
        process.exit(1);
}

// Execute the binary with any arguments passed to the script
execSync(`"${binaryPath}" ${process.argv.slice(2).join(' ')}`, { stdio: 'inherit' });
