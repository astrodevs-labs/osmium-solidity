import { VSCode } from '@/types';
import { MessageTypeContract } from '@pages/DeployPage/DeployPage.logic.ts';
import { MessageType } from '@pages/InteractPage/InteractPage.logic.ts';

export const useEdit = (vscode: VSCode) => {
  const editEnvironment = () => {
    vscode.postMessage({ type: MessageTypeContract.EDIT_ENVIRONMENT });
  };

  const editWallet = () => {
    vscode.postMessage({ type: MessageType.EDIT_WALLETS });
  };

  const editContract = () => {
    vscode.postMessage({ type: MessageType.EDIT_CONTRACTS });
  };

  return {
    editEnvironment,
    editWallet,
    editContract,
  };
};
