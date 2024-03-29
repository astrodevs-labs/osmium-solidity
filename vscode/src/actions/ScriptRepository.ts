import * as fs from 'fs';
import * as path from 'path';
import { Scripts } from './types';
import { getTomlValue } from '../utils';
import { v4 as uuidv4 } from 'uuid';

export class ScriptRepository {
  private _scripts: Scripts = [];
  private readonly _scriptFolderPath: string;
  private readonly _foundryConfigPath: string;

  constructor(workspacePath: string) {
    this._foundryConfigPath = path.join(workspacePath, 'foundry.toml');

    if (fs.existsSync(this._foundryConfigPath)) {
      const script = getTomlValue(this._foundryConfigPath, 'script');

      this._scriptFolderPath = path.join(workspacePath, script ?? 'script');
    } else {
      this._scriptFolderPath = path.join(workspacePath, 'script');
    }

    this.load();
  }

  public load(): void {
    this._scripts = [];

    if (!fs.existsSync(this._scriptFolderPath)) {
      return;
    }

    const regex = new RegExp(/contract\s+(\w+)\s+is\s+Script/g);

    fs.readdirSync(this._scriptFolderPath).forEach((file) => {
      if (!file.endsWith('.s.sol')) {
        return;
      }
      const content = fs.readFileSync(path.join(this._scriptFolderPath, file), 'utf-8');
      let matches;

      while ((matches = regex.exec(content)) !== null) {
        this._addScript(matches[1], file);
      }
    });
  }

  _addScript(name: string, path: string): void {
    if (this._scripts.find((s) => s.name === name)) {
      if (this._scripts.find((s) => s.path === path)) {
        return;
      }
    }
    this._scripts.push({
      name,
      path,
      id: uuidv4(),
    });
  }

  public getScripts(): Scripts {
    return this._scripts;
  }
}
