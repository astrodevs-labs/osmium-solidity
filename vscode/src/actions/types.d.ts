import { Abi } from "viem";

export type DeployContracts = {
    name: string;
    path: string;
    abi: Abi;
    address?: string;
}

export type DeployScript = {
    path: string;
    name: string;
}

export type DeployEnvironment = {
    name: string;
    rpc: string;
}

export type DeployScriptArgs = {
    rpcUrl: string;
    script: DeployScript;
    verify: boolean;
}

export type DeployContractArgs = {
    rpcUrl: string;
    contract: DeployContracts;
    verify: boolean;
    cstrArgs: string[];
}

export interface InteractContract {
    name: string;
    address: `0x${string}`;
    abi: Abi;
    chainId: number;
    rpc:
        | `ws://${string}`
        | `wss://${string}`
        | `http://${string}`
        | `https://${string}`;
}

export type InteractContracts = InteractContract[];

export interface InteractWallet {
    name: string;
    address: `0x${string}`;
    privateKey: `0x${string}`;
    rpc: string;
}
