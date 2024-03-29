import * as path from 'path';
import * as fs from 'fs';
import { Address } from 'viem';
import { Wallet, Wallets } from './types';

export class WalletRepository {
  private _wallets: Wallet[] = [];
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

  public getWallet(address: Wallet['address']): Wallet | undefined {
    return this._wallets.find((w) => w.address === address);
  }

  public createWallet(wallet: Wallet): void {
    if (this._wallets.find((w) => w.address === wallet.address)) {
      // replace
      this._wallets = this._wallets.map((w) => {
        if (w.address === wallet.address) {
          return wallet;
        }
        return w;
      });
    } else {
      this._wallets.push(wallet);
    }
    this._save();
  }

  public deleteWallet(address: Address): void {
    this._wallets = this._wallets.filter((w) => w.address !== address);
    this._save();
  }
}
