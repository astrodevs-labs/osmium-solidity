/* eslint-disable @typescript-eslint/no-explicit-any */
export type VSCode = any;

export type WalletForm = {
  name: string;
  address: string;
  privateKey: string;
};

export type EnvironmentForm = {
  name: string;
  rpc: string;
};

export type ContractForm = {
  name: string;
  address: string;
  chainId: string;
  rpc: string;
  abi: string;
};
