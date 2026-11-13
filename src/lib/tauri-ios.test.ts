// src/lib/tauri-ios.test.ts
// Guards the iOS shipping setup: npm scripts, Tauri iOS config and the custom
// (web-derived) app icon source used instead of the default Tauri icons.

import { describe, it, expect } from 'vitest';
import { readFileSync, existsSync } from 'node:fs';
import { join } from 'node:path';
import { buildInstallArgs, IPA_PATH, MISSING_DEVICE_ID_MESSAGE } from '../../scripts/ios-install.mjs';

// vitest runs from the repo root (see `npm run test`).
const root = process.cwd();
const read = (p: string) => readFileSync(join(root, p), 'utf-8');

const pkg = JSON.parse(read('package.json'));
const tauriConf = JSON.parse(read('src-tauri/tauri.conf.json'));

describe('iOS npm scripts', () => {
  it('ios:build builds a debugging-export IPA', () => {
    expect(pkg.scripts['ios:build']).toBe('tauri ios build --export-method debugging');
  });

  it('ios:install runs the install script (device id comes from env, not package.json)', () => {
    expect(pkg.scripts['ios:install']).toBe('node scripts/ios-install.mjs');
    expect(pkg.scripts['ios:install']).not.toMatch(/[0-9A-F]{8}-[0-9A-F]{16}/);
  });

  it('ios:device runs build then install', () => {
    expect(pkg.scripts['ios:device']).toBe('npm run ios:build && npm run ios:install');
  });

  it('ios:icons regenerates icons from the custom app icon source', () => {
    expect(pkg.scripts['ios:icons']).toContain('tauri icon src-tauri/app-icon.svg');
    expect(pkg.scripts['ios:icons']).toContain('--ios-color');
  });
});

describe('ios-install script', () => {
  it('builds the devicectl args from IOS_DEVICE_ID and the IPA path', () => {
    expect(buildInstallArgs('ABC-123')).toEqual([
      'devicectl', 'device', 'install', 'app', '--device', 'ABC-123',
      'src-tauri/gen/apple/build/arm64/cracked-stack.ipa',
    ]);
    expect(IPA_PATH).toBe('src-tauri/gen/apple/build/arm64/cracked-stack.ipa');
  });

  it('trims whitespace around the device id', () => {
    expect(buildInstallArgs('  ABC-123\n')[5]).toBe('ABC-123');
  });

  it('throws a helpful error when IOS_DEVICE_ID is missing or blank', () => {
    expect(() => buildInstallArgs(undefined)).toThrow(MISSING_DEVICE_ID_MESSAGE);
    expect(() => buildInstallArgs('   ')).toThrow('IOS_DEVICE_ID is not set');
    expect(MISSING_DEVICE_ID_MESSAGE).toContain('.env');
  });

  it('rejects a device id that looks like a flag', () => {
    expect(() => buildInstallArgs('--help')).toThrow('must not start with "-"');
  });

  it('.env.example documents IOS_DEVICE_ID with an empty value', () => {
    expect(read('.env.example')).toMatch(/^IOS_DEVICE_ID=$/m);
  });

  it('no real device id is hardcoded in tracked docs or scripts', () => {
    const udid = /[0-9A-F]{8}-[0-9A-F]{16}/;
    for (const f of ['package.json', 'README.md', '.env.example', 'scripts/ios-install.mjs']) {
      expect(read(f)).not.toMatch(udid);
    }
  });
});

describe('tauri.conf.json iOS settings', () => {
  it('uses productName cracked-stack so the IPA is named cracked-stack.ipa', () => {
    expect(tauriConf.productName).toBe('cracked-stack');
  });

  it('sets the Apple development team for signing', () => {
    expect(tauriConf.bundle.iOS.developmentTeam).toBe('KQG876G5PV');
  });
});

describe('custom app icon source', () => {
  const appIcon = read('src-tauri/app-icon.svg');
  const favicon = read('static/favicon.svg');

  it('exists', () => {
    expect(existsSync(join(root, 'src-tauri/app-icon.svg'))).toBe(true);
  });

  it('is derived from the web favicon (same bolt artwork)', () => {
    const bolt = /<path d="([^"]+)"/.exec(favicon)?.[1];
    expect(bolt).toBeTruthy();
    expect(appIcon).toContain(bolt!);
  });

  it('is square, large, and has a full-bleed opaque background (iOS rounds corners itself)', () => {
    expect(appIcon).toContain('width="1024" height="1024"');
    expect(appIcon).toContain('<rect width="32" height="32" fill="#0F1C32"/>');
    expect(appIcon).not.toContain('rx="');
  });
});
