import { Environments, InteractContracts, Wallets } from '@backend/actions/types';
import { useEffect, useState } from 'react';
import { VSCode } from '@/types';
import { MessageType } from '@backend/enums.ts';

export type ResourceManager = {
  wallets: Wallets;
  environments: Environments;
  interactContracts: InteractContracts;
  openingPanelId: string;
  setOpeningPanelId: (id: string) => void;
};

export const useResourceManager = (vscode: VSCode): ResourceManager => {
  const [wallets, setWallets] = useState<Wallets>([]);
  const [environments, setEnvironments] = useState<Environments>([]);
  const [interactContracts, setInteractContracts] = useState<InteractContracts>([]);
  const [openingPanelId, setOpeningPanelId] = useState<string>('');

  useEffect(() => {
    if (!vscode) {
      return;
    }
    vscode.postMessage({ type: MessageType.GET_WALLETS });
    vscode.postMessage({ type: MessageType.GET_ENVIRONMENTS });
    vscode.postMessage({ type: MessageType.GET_INTERACT_CONTRACTS });
  }, [vscode]);

  useEffect(() => {
    const listener = (event: WindowEventMap['message']) => {
      switch (event.data.type) {
        case MessageType.WALLETS: {
          setWallets(event.data.wallets);
          break;
        }
        case MessageType.ENVIRONMENTS: {
          setEnvironments(event.data.environments);
          break;
        }
        case MessageType.INTERACT_CONTRACTS: {
          setInteractContracts(event.data.contracts);
          break;
        }
        case MessageType.OPEN_PANEL_RESPONSE: {
          setOpeningPanelId(event.data.id);
          break;
        }
      }
    };
    window.addEventListener('message', listener);
    return () => window.removeEventListener('message', listener);
  }, []);

  return {
    wallets,
    environments,
    interactContracts,
    openingPanelId,
    setOpeningPanelId,
  };
};
