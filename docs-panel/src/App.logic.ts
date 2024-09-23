import { useEffect, useState } from 'react';
import { VSCode } from '@/types';
import { useResourceManager } from '@hooks/useResourceManager.ts';

export const useApp = () => {
  const [vscode, setVscode] = useState<VSCode>();
  const resourceManager = useResourceManager(vscode);

  useEffect(() => {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    setVscode(acquireVsCodeApi());
  }, []);

  return {
    vscode,
    resourceManager,
  };
};
