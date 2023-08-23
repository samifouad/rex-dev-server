#!/usr/bin/env node
const { execSync } = require('child_process');
const os = require('os');

let binaryPath;
let npmGlobal = execSync("npm root -g").toString().trim();

switch (os.platform().toString().trim()) {
    case 'darwin':
        binaryPath = os.arch() === 'x64'
            ? npmGlobal +'/rexds/node_modules/@fromafrica/rexds-darwin-x64/rexds'
            : npmGlobal +'/rexds/node_modules/@fromafrica/rexds-darwin-arm64/rexds';
        break;
    case 'win32':
        binaryPath = npmGlobal +'\\rexds\\node_modules\\@fromafrica\\rexds-windows-x64\\rexds.exe';
        break;
    case 'linux':
        binaryPath = npmGlobal +'/rexds/node_modules/@fromafrica/rexds-linux-x64-musl/rexds';
        break;
    default:
        console.error(`Unsupported platform: ${os.platform()}`);
        process.exit(1);
}

// Execute the binary with any arguments passed to the script
execSync(`"${binaryPath}" ${process.argv.slice(2).join(' ')}`, { stdio: 'inherit' });