import * as path from 'path';
import * as fs from 'fs';
import { Address } from 'viem';
import { Wallet, Wallets } from './types';
import { v4 as uuidv4 } from 'uuid';
import { privateKeyToAddress } from 'viem/accounts';

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
    this._wallets = this._wallets.map((w: Wallet) => {
      try {
        return { ...w, address: privateKeyToAddress(w.privateKey) };
      } catch (e) {
        console.error('Impossible to parse private key: ', w.privateKey);
        return { ...w, address: '0x' };
      }
    });
  }

  public getWallets(): Wallets {
    return this._wallets;
  }

  public getWallet(id: Wallet['id']): Wallet | undefined {
    return this._wallets.find((w) => w.id === id);
  }

  public createWallet(name: string, privateKey: Address): void {
    const address = privateKeyToAddress(privateKey);
    const wallet: Wallet = { name, address, privateKey, id: uuidv4() };

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

  public updateWallet(id: Wallet['id'], key: string, value: string): void {
    const wallet = this._wallets.find((w) => w.id === id);

    if (wallet) {
      // @ts-ignore
      wallet[key] = value;
      this._save();
    }
  }

  public deleteWallet(id: Wallet['id']): void {
    this._wallets = this._wallets.filter((w) => w.id !== id);
    this._save();
  }
}
