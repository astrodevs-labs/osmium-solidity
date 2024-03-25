import { DFormContract, VSCode } from '@/types';
import { MessageTypeContract } from "@pages/DeployPage/DeployPage.logic.ts";
import { useFormContext } from 'react-hook-form';

export const useDeployContract = (vscode: VSCode) => {
  const form = useFormContext<DFormContract>();
  const { formState: { errors } } = form;

  const editEnvironment = () => {
    vscode.postMessage({ type: MessageTypeContract.EDIT_ENVIRONMENT });
  }

  return { form, errors, editEnvironment};
};