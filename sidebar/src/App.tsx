import { DeployPage } from '@pages/DeployPage/DeployPage.tsx';
import { InteractPage } from '@pages/InteractPage/InteractPage.tsx';
import { GetStartedPage } from '@pages/GetStartedPage/GetStartedPage.tsx';
import { VSCodePanels, VSCodePanelTab, VSCodePanelView } from '@vscode/webview-ui-toolkit/react';
import { useApp } from '@/App.logic.ts';

export const App = () => {
  const logic = useApp();

  return (
    <>
      <GetStartedPage />
      <VSCodePanels>
        <VSCodePanelTab id="tab-interact">INTERACT</VSCodePanelTab>
        <VSCodePanelTab id="tab-deploy">DEPLOY</VSCodePanelTab>
        <VSCodePanelView id="view-interact">
          <InteractPage vscode={logic.vscode} resourceManager={logic.resourceManager} />
        </VSCodePanelView>
        <VSCodePanelView id="view-deploy">
          <DeployPage vscode={logic.vscode} resourceManager={logic.resourceManager} />
        </VSCodePanelView>
      </VSCodePanels>
    </>
  );
};
