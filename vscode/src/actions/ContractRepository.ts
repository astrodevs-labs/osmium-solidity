import * as path from "path";
import * as fs from "fs";
import {Address} from "viem";
import {InteractContract, InteractContracts} from "./types";

export class ContractRepository {
  private _contracts: InteractContracts = [];
  private _contractsPath: string;
  private _osmiumPath: string

  constructor(workspacePath: string) {
    this._osmiumPath = path.join(workspacePath, ".osmium");
    this._contractsPath = path.join(this._osmiumPath, "contracts.json");
    this.load();
  }

  private _save(): void {
    const json = JSON.stringify({ contracts: this._contracts });
    fs.writeFileSync(this._contractsPath, json, { encoding: "utf-8" });
  }

  public load(): void {
    if (!fs.existsSync(this._osmiumPath))
      fs.mkdirSync(this._osmiumPath);
    if (!fs.existsSync(this._contractsPath)) {
      this._contracts = [];
      fs.writeFileSync(
        this._contractsPath,
        JSON.stringify({ contracts: this._contracts }),
      );
    } else {
      const raw = fs.readFileSync(this._contractsPath);
      const json = JSON.parse(raw.toString());
      this._contracts = json.contracts;
    }
  }

  public getContracts(): InteractContracts {
    return this._contracts;
  }

  public getContract(name: InteractContract["address"]): InteractContract | undefined {
    return this._contracts.find((c) => c.address === name);
  }

  public createContract(contract: InteractContract): void {
    this._contracts.push(contract);
    this._save();
  }

  public deleteContract(address: Address): void {
    this._contracts = this._contracts.filter((c) => c.address !== address);
    this._save();
  }
}
