import * as fs from 'fs';
import * as toml from 'toml';

function getNonce(): string {
  let nonce: string = '';
  const possible: string = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  for (let i = 0; i < 32; i++) {
    nonce += possible.charAt(Math.floor(Math.random() * possible.length));
  }
  return nonce;
}

function getTomlValue(path: string, key: string): string {
  const tomlContent = fs.readFileSync(path, 'utf8');
  const parsedToml = toml.parse(tomlContent);

  return parsedToml.profile.default[key];
}

export { getNonce, getTomlValue };
