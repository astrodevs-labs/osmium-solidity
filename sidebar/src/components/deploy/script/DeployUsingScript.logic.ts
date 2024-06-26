import { useFormContext } from 'react-hook-form';
import { IDeployScriptForm, VSCode } from '@/types';
import { useEdit } from '@hooks/useEdit.ts';
import { useEffect, useState } from 'react';
import { MessageType } from '@backend/enums.ts';
import { Environments, Scripts } from '@backend/actions/types';

export const useDeployUsingScript = (vscode: VSCode, scripts: Scripts, environments: Environments) => {
  const { editEnvironment } = useEdit(vscode);
  const form = useFormContext<IDeployScriptForm>();
  const {
    formState: { errors },
  } = form;
  const [response, setResponse] = useState<{
    exitCode: number;
    output: string;
  } | null>(null);

  useEffect(() => {
    const listener = (event: WindowEventMap['message']) => {
      switch (event.data.type) {
        case MessageType.DEPLOY_SCRIPT_RESPONSE: {
          setResponse(event.data.response);
          break;
        }
      }
    };
    window.addEventListener('message', listener);
    return () => window.removeEventListener('message', listener);
  }, []);

  useEffect(() => {
    form.setValue('script', scripts[0]?.id || '');
  }, [scripts]);

  useEffect(() => {
    form.setValue('environment', environments[0]?.id || '');
  }, [environments]);

  return { form, errors, response, editEnvironment };
};
