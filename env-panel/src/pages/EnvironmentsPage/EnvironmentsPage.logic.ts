import { EnvironmentForm, VSCode } from '@/types';
import { MessageType } from '@backend/enums.ts';
import { SubmitHandler, useForm } from 'react-hook-form';

export const useEnvironmentsPageLogic = (vscode: VSCode) => {
  const form = useForm<EnvironmentForm>({
    defaultValues: {
      name: '',
      rpc: '',
    },
  });

  const onSubmit: SubmitHandler<EnvironmentForm> = (data) => {
    if (!data.name.length) form.setError('name', { type: 'manual', message: 'Invalid string' });
    if (!data.rpc.length) form.setError('rpc', { type: 'manual', message: 'Invalid string' });

    vscode.postMessage({
      type: MessageType.ADD_ENVIRONMENT,
      data,
    });
  };

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
    form,
    onSubmit,
    deleteEnvironment,
    editEnvironment,
  };
};
