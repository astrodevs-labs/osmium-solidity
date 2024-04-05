import * as path from 'path';
import * as fs from 'fs';
import { DeployContract, DeployContracts } from './types';
import { getTomlValue } from '../utils';
import { v4 as uuidv4 } from 'uuid';

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

    const outFiles = fs
      .readdirSync(this._outFolderPath, { recursive: true })
      .filter((f) => f.toString().endsWith('.json'));

    const outFilesContent = outFiles.map((f) =>
      JSON.parse(fs.readFileSync(path.join(this._outFolderPath, f.toString())).toString()),
    );

    for (const outFile of outFiles) {
      const outFileContent = JSON.parse(fs.readFileSync(path.join(this._outFolderPath, outFile.toString())).toString());
      const target = Object.keys(outFileContent.metadata.settings.compilationTarget)[0];

      if (path.parse(target).dir !== path.basename(this._srcFolderPath)) {
        continue;
      }

      console.log(outFile.toString());

      this._contracts.push({
        name: path.basename(outFile.toString(), '.json'),
        path: target,
        abi: outFileContent.abi,
        id: uuidv4(),
      });
    }
  }

  public getContracts(): DeployContracts {
    return this._contracts;
  }

  public getContract(id: DeployContract['id']): DeployContract | undefined {
    return this._contracts.find((c) => c.id === id);
  }
}
