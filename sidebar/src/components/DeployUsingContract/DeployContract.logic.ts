import { useFormContext } from 'react-hook-form';
import { DFormContract } from '../../types';
import {VSCode} from '../../types';
import {MessageTypeContract} from "../../pages/DeployPage/DeployPage.logic.ts";

export const useDeployContract = (vscode: VSCode) => {
  const form = useFormContext<DFormContract>();
  const { formState: { errors } } = form;

  const editEnvironment = () => {
    vscode.postMessage({ type: MessageTypeContract.EDIT_ENVIRONMENT });
  }

  return { form, errors, editEnvironment};
};