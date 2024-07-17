import { VSCode } from '@/types';
import { MessageType } from '@backend/enums.ts';

export const useEnvironmentsPageLogic = (vscode: VSCode) => {
  const deleteEnvironment = (id: string) => {
    vscode.postMessage({
      type: MessageType.DELETE_ENVIRONMENT,
      data: {
        id: id,
      },
    });
  };

  const editEnvironment = (id: string, key: string, value: string) => {
    vscode.postMessage({
      type: MessageType.EDIT_ENVIRONMENT,
      data: {
        id: id,
        key: key,
        value: value,
      },
    });
  };

  return {
    deleteEnvironment,
    editEnvironment,
  };
};
