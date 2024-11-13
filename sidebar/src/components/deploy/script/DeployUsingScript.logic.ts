import { IDeployScriptForm, VSCode } from '@/types';
import { Scripts, Environments } from '@backend/actions/types';
import { MessageType } from '@backend/enums';
import { useEffect, useState } from 'react';
import { useFormContext } from 'react-hook-form';

export const useDeployUsingScript = (
  vscode: VSCode,
  scripts: Scripts,
  environments: Environments,
  setIsPending: (isPending: boolean) => void,
) => {
  const form = useFormContext<IDeployScriptForm>();
  const {
    formState: { errors },
  } = form;
  const [response, setResponse] = useState<{
    exitCode: number;
    output: string;
  } | null>(null);

  const openPanel = (id: string) => {
    vscode.postMessage({ type: MessageType.OPEN_PANEL, data: { id } });
  };

  useEffect(() => {
    const listener = (event: WindowEventMap['message']) => {
      switch (event.data.type) {
        case MessageType.DEPLOY_SCRIPT_RESPONSE: {
          setResponse(event.data.response);
          setIsPending(false);
          break;
        }
      }
    };
    window.addEventListener('message', listener);
    return () => window.removeEventListener('message', listener);
  }, [setIsPending]);

  useEffect(() => {
    form.setValue('script', scripts[0]?.id || '');
  }, [scripts]);

  useEffect(() => {
    form.setValue('environment', environments[0]?.id || '');
  }, [environments]);

  return { form, errors, response, openPanel };
};
