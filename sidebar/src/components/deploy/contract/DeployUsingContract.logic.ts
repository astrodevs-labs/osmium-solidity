import { IDeployContractForm, VSCode } from '@/types';
import { useFormContext } from 'react-hook-form';
import { useEdit } from '@hooks/useEdit.ts';

export const useDeployUsingContract = (vscode: VSCode) => {
  const { editEnvironment, editWallet } = useEdit(vscode);
  const form = useFormContext<IDeployContractForm>();
  const {
    formState: { errors },
  } = form;

  return { form, errors, editEnvironment, editWallet };
};
