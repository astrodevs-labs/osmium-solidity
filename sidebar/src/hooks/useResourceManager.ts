import { DeployContracts, Environments, InteractContracts, Scripts, Wallets } from '@backend/actions/types';
import { useEffect, useState } from 'react';
import { VSCode } from '@/types';
import { MessageType } from '@backend/enums.ts';

export type ResourceManager = {
  wallets: Wallets;
  scripts: Scripts;
  environments: Environments;
  deployContracts: DeployContracts;
  interactContracts: InteractContracts;
};

export const useResourceManager = (vscode: VSCode): ResourceManager => {
  const [wallets, setWallets] = useState<Wallets>([]);
  const [scripts, setScripts] = useState<Scripts>([]);
  const [environments, setEnvironments] = useState<Environments>([]);
  const [interactContracts, setInteractContracts] = useState<InteractContracts>([]);
  const [deployContracts, setDeployContracts] = useState<DeployContracts>([]);

  useEffect(() => {
    if (!vscode) {
      return;
    }
    vscode.postMessage({ type: MessageType.GET_WALLETS });
    vscode.postMessage({ type: MessageType.GET_SCRIPTS });
    vscode.postMessage({ type: MessageType.GET_ENVIRONMENTS });
    vscode.postMessage({ type: MessageType.GET_DEPLOY_CONTRACTS });
    vscode.postMessage({ type: MessageType.GET_INTERACT_CONTRACTS });
  }, [vscode]);

  useEffect(() => {
    const listener = (event: WindowEventMap['message']) => {
      switch (event.data.type) {
        case MessageType.WALLETS: {
          setWallets(event.data.wallets);
          break;
        }
        case MessageType.SCRIPTS: {
          setScripts(event.data.scripts);
          break;
        }
        case MessageType.ENVIRONMENTS: {
          setEnvironments(event.data.environments);
          break;
        }
        case MessageType.DEPLOY_CONTRACTS: {
          setDeployContracts(event.data.contracts);
          break;
        }
        case MessageType.INTERACT_CONTRACTS: {
          setInteractContracts(event.data.contracts);
          break;
        }
        default: {
          throw Error('Unknown command: ' + event.type);
        }
      }
    };
    window.addEventListener('message', listener);
    return () => window.removeEventListener('message', listener);
  }, []);

  return {
    wallets,
    scripts,
    environments,
    interactContracts,
    deployContracts,
  };
};
