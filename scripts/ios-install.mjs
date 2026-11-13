// scripts/ios-install.mjs
//
// Installs the built IPA on a physical iPhone:
//   xcrun devicectl device install app --device <IOS_DEVICE_ID> <ipa>
//
// The device id is read from IOS_DEVICE_ID (shell env wins, otherwise `.env`)
// so it never has to be committed. Run via `npm run ios:install`.

import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

export const IPA_PATH = 'src-tauri/gen/apple/build/arm64/cracked-stack.ipa';

export const MISSING_DEVICE_ID_MESSAGE =
  'IOS_DEVICE_ID is not set. Add IOS_DEVICE_ID=<udid> to .env (or export it in your shell). ' +
  'Find your device id with: xcrun devicectl list devices';

/**
 * Builds the devicectl argument list. Throws if the device id is missing or
 * looks like a flag (we never go through a shell, but this keeps it from being
 * parsed as an option).
 * @param {string | undefined} deviceId
 * @param {string} [ipaPath]
 * @returns {string[]}
 */
export function buildInstallArgs(deviceId, ipaPath = IPA_PATH) {
  const id = (deviceId ?? '').trim();
  if (!id) throw new Error(MISSING_DEVICE_ID_MESSAGE);
  if (id.startsWith('-')) throw new Error('IOS_DEVICE_ID must not start with "-".');
  return ['devicectl', 'device', 'install', 'app', '--device', id, ipaPath];
}

function main() {
  const root = resolve(fileURLToPath(import.meta.url), '..', '..');
  const envFile = resolve(root, '.env');
  // loadEnvFile does not override variables that are already set in the shell.
  if (existsSync(envFile)) process.loadEnvFile(envFile);

  let args;
  try {
    args = buildInstallArgs(process.env.IOS_DEVICE_ID);
  } catch (e) {
    console.error(e instanceof Error ? e.message : String(e));
    process.exit(1);
  }

  if (!existsSync(resolve(root, IPA_PATH))) {
    console.error(`IPA not found at ${IPA_PATH}. Run: npm run ios:build`);
    process.exit(1);
  }

  const result = spawnSync('xcrun', args, { cwd: root, stdio: 'inherit' });
  process.exit(result.status ?? 1);
}

// Only run when executed directly (not when imported by tests).
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main();
}
