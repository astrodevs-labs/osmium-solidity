import { useFormContext } from 'react-hook-form';
import { IDeployScriptForm, VSCode } from '@/types';
import { useEdit } from '@hooks/useEdit.ts';

export const useDeployUsingScript = (vscode: VSCode) => {
  const { editEnvironment } = useEdit(vscode);
  const form = useFormContext<IDeployScriptForm>();
  const {
    formState: { errors },
  } = form;

  return { form, errors, editEnvironment };
};
