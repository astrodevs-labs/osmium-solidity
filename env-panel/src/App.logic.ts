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

  const onTabChange = (event: any) => {
    const tabId = event.detail?.id;

    if (!tabId || !tabId.startsWith('tab-')) return;

    resourceManager.setOpeningPanelId(tabId);
  };

  return {
    vscode,
    resourceManager,
    onTabChange,
  };
};
