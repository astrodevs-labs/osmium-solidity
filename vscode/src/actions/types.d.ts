import { Abi, Address } from "viem";

export type RpcUrl =
  | `ws://${string}`
  | `wss://${string}`
  | `http://${string}`
  | `https://${string}`;

export type DeployContracts = {
  name: string;
  path: string;
  abi: Abi;
  address?: string;
};

export type DeployScript = {
  path: string;
  name: string;
};

export type DeployEnvironment = {
  name: string;
  rpc: string;
};

export type DeployScriptArgs = {
  rpcUrl: string;
  script: DeployScript;
  verify: boolean;
};

export type DeployContractArgs = {
  rpcUrl: string;
  contract: DeployContracts;
  verify: boolean;
  cstrArgs: string[];
};

export interface InteractContract {
  name: string;
  address: Address;
  abi: Abi;
  chainId: number;
  rpc: RpcUrl;
}

export type InteractContracts = InteractContract[];

export interface InteractWallet {
  name: string;
  address: Address;
  privateKey: Address;
  rpc: RpcUrl;
}

export type InteractWallets = InteractWallet[];

export interface Environment {
  name: string;
  rpc: RpcUrl;
}

export type Environments = Environment[];

export type ContractParam = any;

export type ContractParams = ContractParam[];

export interface ReadContractOptions {
  contract: Address;
  method: string;
  params?: ContractParams;
}

export interface WriteContractOptions {
  account: Address;
  address: Address;
  abi: Abi;
  functionName: string;
  params?: ContractParams;
  gasLimit?: bigint;
  value?: bigint;
}
