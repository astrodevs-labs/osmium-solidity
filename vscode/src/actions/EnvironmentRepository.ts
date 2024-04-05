import * as path from 'path';
import * as fs from 'fs';
import { Environment, Environments, RpcUrl } from './types';
import { v4 as uuidv4 } from 'uuid';

export class EnvironmentRepository {
  private _environments: Environments = [];
  private readonly _environmentsPath: string;
  private readonly _osmiumPath: string;

  constructor(workspacePath: string) {
    this._osmiumPath = path.join(workspacePath, '.osmium');
    this._environmentsPath = path.join(this._osmiumPath, 'environments.json');
    this.load();
  }

  private _save(): void {
    const json = JSON.stringify({ environments: this._environments });
    fs.writeFileSync(this._environmentsPath, json, { encoding: 'utf-8' });
  }

  public load(): void {
    if (!fs.existsSync(this._osmiumPath)) {
      fs.mkdirSync(this._osmiumPath);
    }
    if (!fs.existsSync(this._environmentsPath)) {
      this._environments = [];
      fs.writeFileSync(this._environmentsPath, JSON.stringify({ environments: this._environments }));
    } else {
      const raw = fs.readFileSync(this._environmentsPath);
      const json = JSON.parse(raw.toString());
      this._environments = json.environments;
    }
  }

  public getEnvironments(): Environments {
    return this._environments;
  }

  public getEnvironment(id: Environment['id']): Environment | undefined {
    return this._environments.find((e) => e.id === id);
  }

  public createEnvironment(name: string, rpc: RpcUrl): void {
    const environment: Environment = { name, rpc, id: uuidv4() };
    if (this._environments.find((e) => e.name === name)) {
      // replace
      this._environments = this._environments.map((e) => {
        if (e.name === name) {
          return environment;
        }
        return e;
      });
    } else {
      this._environments.push(environment);
    }
    this._save();
  }

  public deleteEnvironment(name: string): void {
    this._environments = this._environments.filter((e) => e.name !== name);
    this._save();
  }
}
