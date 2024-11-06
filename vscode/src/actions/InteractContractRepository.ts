import * as path from 'path';
import * as fs from 'fs';
import { Abi, Address } from 'viem';
import { InteractContract, InteractContracts, RpcUrl } from './types';
import { v4 as uuidv4 } from 'uuid';

export class InteractContractRepository {
  private _contracts: InteractContracts = [];
  private readonly _contractsPath: string;
  private readonly _osmiumPath: string;

  constructor(workspacePath: string) {
    this._osmiumPath = path.join(workspacePath, '.osmium');
    this._contractsPath = path.join(this._osmiumPath, 'contracts.json');
    this.load();
  }

  private _save(): void {
    const json = JSON.stringify({ contracts: this._contracts });
    fs.writeFileSync(this._contractsPath, json, { encoding: 'utf-8' });
  }

  public load(): void {
    if (!fs.existsSync(this._osmiumPath)) {
      fs.mkdirSync(this._osmiumPath);
    }
    if (!fs.existsSync(this._contractsPath)) {
      this._contracts = [];
      fs.writeFileSync(this._contractsPath, JSON.stringify({ contracts: this._contracts }));
    } else {
      const raw = fs.readFileSync(this._contractsPath);
      const json = JSON.parse(raw.toString());
      this._contracts = json.contracts;
    }
  }

  public getContracts(): InteractContracts {
    return this._contracts;
  }

  public getContract(id: InteractContract['id']): InteractContract | undefined {
    return this._contracts.find((c) => c.id === id);
  }

  public createContract(address: Address, abi: Abi, chainId: number, name: string, rpc: RpcUrl): void {
    const contract: InteractContract = { address, abi, chainId, name, rpc, id: uuidv4() };
    if (this._contracts.find((c) => c.address === address)) {
      // replace
      this._contracts = this._contracts.map((w) => {
        if (w.address === address) {
          return contract;
        }
        return w;
      });
    } else {
      this._contracts.push(contract);
    }
    this._save();
  }

  public updateContract(
    id: InteractContract['id'],
    key: 'name' | 'address' | 'abi' | 'chainId' | 'rpc',
    value: string,
  ): void {
    const environment = this._contracts.find((e) => e.id === id);
    if (environment) {
      // @ts-ignore to change
      environment[key] = value;
      this._save();
    }
  }

  public deleteContract(id: InteractContract['id']): void {
    this._contracts = this._contracts.filter((c) => c.id !== id);
    this._save();
  }
}
