import * as path from 'path';
import * as fs from 'fs';
import { Abi, Address } from 'viem';
import { DeployContract, DeployContracts, InteractContract, InteractContracts, RpcUrl, Script, Scripts } from './types';
import { v4 as uuidv4 } from 'uuid';
import { getTomlValue } from '../utils';
import { workspace } from 'vscode';

export class DeployContractRepository {
  private _contracts: DeployContracts = [];
  private readonly _srcFolderPath: string;
  private readonly _outFolderPath: string;
  private readonly _foundryConfigPath: string;

  constructor(workspacePath: string) {
    this._foundryConfigPath = path.join(workspacePath, 'foundry.toml');

    if (fs.existsSync(this._foundryConfigPath)) {
      this._srcFolderPath = path.join(workspacePath, getTomlValue(this._foundryConfigPath, 'src') ?? 'src');
      this._outFolderPath = path.join(workspacePath, getTomlValue(this._foundryConfigPath, 'out') ?? 'out');
    } else {
      this._srcFolderPath = path.join(workspacePath, 'src');
      this._outFolderPath = path.join(workspacePath, 'out');
    }

    this.load();
  }

  public async load(): Promise<void> {
    this._contracts = [];

    if (!fs.existsSync(this._srcFolderPath) || !fs.existsSync(this._outFolderPath)) {
      return;
    }

    const jsonFiles = await workspace.findFiles(`${this._outFolderPath}/**/*.json`);

    for (const jsonFile of jsonFiles) {
      const json = JSON.parse(fs.readFileSync(jsonFile.fsPath).toString());

      console.log(json);
    }

    // const regex = new RegExp(/contract\s+(\w+)\s+is\s+Script/g);
    //
    // fs.readdirSync(this._scriptFolderPath).forEach((file) => {
    //   if (!file.endsWith('.s.sol')) {
    //     return;
    //   }
    //   const content = fs.readFileSync(path.join(this._scriptFolderPath, file), 'utf-8');
    //   let matches;
    //
    //   while ((matches = regex.exec(content)) !== null) {
    //     this._addScript(matches[1], file);
    //   }
    // });
  }

  // _addScript(name: string, path: string): void {
  //     if (this._scripts.find((s) => s.name === name)) {
  //         if (this._scripts.find((s) => s.path === path)) {
  //             return;
  //         }
  //     }
  //     this._scripts.push({
  //         name,
  //         path,
  //         id: uuidv4(),
  //     });
  // }

  public getContracts(): DeployContracts {
    return this._contracts;
  }

  public getContract(id: DeployContract['id']): DeployContract | undefined {
    return this._contracts.find((c) => c.id === id);
  }
}
