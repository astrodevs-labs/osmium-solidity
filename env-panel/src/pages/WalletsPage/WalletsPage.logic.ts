import './WalletsPage.css';
import { VSCode } from '@/types';
import { MessageType } from '@backend/enums.ts';

export const useWalletsPageLogic = (vscode: VSCode) => {
  const deleteWallet = (id: string) => {
    vscode.postMessage({
      type: MessageType.DELETE_WALLET,
      data: {
        id: id,
      },
    });
  };

  const editWallet = (id: string, key: string, value: string) => {
    vscode.postMessage({
      type: MessageType.EDIT_WALLET,
      data: {
        id: id,
        key: key,
        value: value,
      },
    });
  };

  return {
    deleteWallet,
    editWallet,
  };
};
