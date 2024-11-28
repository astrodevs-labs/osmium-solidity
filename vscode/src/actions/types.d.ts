import { Abi, Address } from 'viem';

export type RpcUrl = `ws://${string}` | `wss://${string}` | `http://${string}` | `https://${string}` | string;

export interface DeployContract {
  name: string;
  path: string;
  abi: Abi;
  id: string;
}

export type DeployContracts = DeployContract[];

export interface InteractContract {
  name: string;
  address: Address;
  abi: Abi;
  chainId: number;
  rpc: RpcUrl;
  id: string;
}

export type InteractContracts = InteractContract[];

export interface Wallet {
  name: string;
  address: Address;
  privateKey: Address;
  id: string;
}

export type Wallets = Wallet[];

export interface Environment {
  name: string;
  rpc: RpcUrl;
  id: string;
}

export type Environments = Environment[];

export interface Script {
  name: string;
  path: string;
  id: string;
}

export type Scripts = Script[];

export type ContractParam = any;

export type ContractParams = ContractParam[];
