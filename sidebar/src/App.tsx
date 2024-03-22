import { VSCodePanels, VSCodePanelTab, VSCodePanelView } from '@vscode/webview-ui-toolkit/react';
import { DeployPage } from './pages/DeployPage/DeployPage.tsx';
import { InteractPage } from './pages/InteractPage/InteractPage.tsx';
import { useEffect, useState } from 'react';
import './App.css';
import { VSCode } from './types';

export const App = () => {
  const [vscode, setVscode] = useState<VSCode>();

  useEffect(() => {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    setVscode(acquireVsCodeApi());
  }, []);

  return (
    <VSCodePanels>
      <VSCodePanelTab id="tab-interact">INTERACT</VSCodePanelTab>
      <VSCodePanelTab id="tab-deploy">DEPLOY</VSCodePanelTab>
      <VSCodePanelView id="view-interact">
        <InteractPage vscode={vscode} />
      </VSCodePanelView>
      <VSCodePanelView id="view-deploy">
        <DeployPage vscode={vscode} />
      </VSCodePanelView>
    </VSCodePanels>
  );
};
