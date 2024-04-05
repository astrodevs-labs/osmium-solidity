import * as path from 'path';
import * as fs from 'fs';
import { Address } from 'viem';
import { RpcUrl, Wallet, Wallets } from './types';
import { v4 as uuidv4 } from 'uuid';

export class WalletRepository {
  private _wallets: Wallets = [];
  private readonly _walletsPath: string;
  private readonly _osmiumPath: string;

  constructor(workspacePath: string) {
    this._osmiumPath = path.join(workspacePath, '.osmium');
    this._walletsPath = path.join(this._osmiumPath, 'wallets.json');
    this.load();
  }

  private _save(): void {
    const json = JSON.stringify({ wallets: this._wallets });
    fs.writeFileSync(this._walletsPath, json, { encoding: 'utf-8' });
  }

  public load(): void {
    if (!fs.existsSync(this._osmiumPath)) {
      fs.mkdirSync(this._osmiumPath);
    }
    if (!fs.existsSync(this._walletsPath)) {
      this._wallets = [];
      fs.writeFileSync(this._walletsPath, JSON.stringify({ wallets: this._wallets }));
    } else {
      const raw = fs.readFileSync(this._walletsPath);
      const json = JSON.parse(raw.toString());
      this._wallets = json.wallets;
    }
  }

  public getWallets(): Wallets {
    return this._wallets;
  }

  public getWallet(id: Wallet['id']): Wallet | undefined {
    return this._wallets.find((w) => w.id === id);
  }

  public createWallet(name: string, address: Address, privateKey: Address, rpc: RpcUrl): void {
    const wallet: Wallet = { name, address, privateKey, rpc, id: uuidv4() };
    if (this._wallets.find((w) => w.address === address)) {
      // replace
      this._wallets = this._wallets.map((w) => {
        if (w.address === address) {
          return wallet;
        }
        return w;
      });
    } else {
      this._wallets.push(wallet);
    }
    this._save();
  }

  public deleteWallet(name: string): void {
    this._wallets = this._wallets.filter((w) => w.name !== name);
    this._save();
  }
}
