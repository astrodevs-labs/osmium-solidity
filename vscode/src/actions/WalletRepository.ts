import * as path from "path";
import * as fs from "fs";
import {InteractWallet} from "./types";

export class WalletRepository {
    private _wallets: InteractWallet[] = [];
    private _walletsPath: string;
    private _osmiumPath: string;

    constructor(workspacePath: string) {
        this._osmiumPath = path.join(workspacePath, ".osmium");
        this._walletsPath = path.join(this._osmiumPath, "wallets.json");
        this.load();
    }

    public load(): void {
        if (!fs.existsSync(this._osmiumPath)) {
            fs.mkdirSync(this._osmiumPath);
        }
        if (fs.existsSync(this._walletsPath)) {
            const walletData = fs.readFileSync(this._walletsPath, "utf8");
            const walletJson = JSON.parse(walletData);
            this._wallets = walletJson.wallets;
        } else {
            fs.writeFileSync(this._walletsPath, JSON.stringify({wallets: []}));
            this._wallets = [];
        }
    }

    public getWallets(): InteractWallet[] {
        return this._wallets;
    }

    public getWallet(address: `0x${string}`): InteractWallet | undefined {
        return this._wallets.find((w) => w.address === address);
    }

    public async createWallet(wallet: InteractWallet): Promise<InteractWallet> {
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

        const walletJson = JSON.stringify({wallets: this._wallets});
        fs.writeFileSync(this._walletsPath, walletJson, "utf8");
        return wallet;
    }

    public async deleteWallet(address: `0x${string}`): Promise<void> {
        this._wallets = this._wallets.filter((w) => w.address !== address);
        const walletJson = JSON.stringify({wallets: this._wallets});
        fs.writeFileSync(this._walletsPath, walletJson, "utf8");
    }
}
