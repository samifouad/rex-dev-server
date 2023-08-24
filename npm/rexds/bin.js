#!/usr/bin/env node
const { execSync } = require('child_process');
const os = require('os');

let binaryPath;

switch (os.platform().toString().trim()) {
    case 'darwin':
        let dariwinNpmGlobal = execSync("npm root -g").toString().trim();
        binaryPath = os.arch() === 'x64'
            ? dariwinNpmGlobal +'/rexds/node_modules/@fromafrica/rexds-darwin-x64/rexds'
            : dariwinNpmGlobal +'/rexds/node_modules/@fromafrica/rexds-darwin-arm64/rexds';
        break;
    case 'win32':
        let winNpmGlobal = execSync("npm root -g").toString().trim();
        binaryPath = winNpmGlobal +'\\rexds\\node_modules\\@fromafrica\\rexds-windows-x64\\rexds.exe';
        break;
    case 'linux':
        let linuxNpmGlobal = execSync("npm root -g").toString().trim().split('/').slice(0, -1).join('/');
        binaryPath = linuxNpmGlobal +'/rexds/node_modules/@fromafrica/rexds-linux-x64-musl/rexds';
        break;
    default:
        console.error(`Unsupported platform: ${os.platform()}`);
        process.exit(1);
}

// Execute the binary with any arguments passed to the script
execSync(`"${binaryPath}" ${process.argv.slice(2).join(' ')}`, { stdio: 'inherit' });