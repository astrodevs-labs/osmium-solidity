import { VSCode } from '@/types';
import { MessageType } from '@backend/enums.ts';

export const useContractsPageLogic = (vscode: VSCode) => {
  const deleteContract = (id: string) => {
    vscode.postMessage({
      type: MessageType.DELETE_CONTRACT,
      data: {
        id: id,
      },
    });
  };

  const editContract = (id: string, key: string, value: string) => {
    vscode.postMessage({
      type: MessageType.EDIT_CONTRACT,
      data: {
        id: id,
        key: key,
        value: value,
      },
    });
  };

  return {
    deleteContract,
    editContract,
  };
};
