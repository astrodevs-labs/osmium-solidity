import { VSCodePanels, VSCodePanelTab, VSCodePanelView } from '@vscode/webview-ui-toolkit/react';
import './App.css';
import { WalletsPage } from '@pages/WalletsPage/WalletsPage.tsx';
import { EnvironmentsPage } from '@pages/EnvironmentsPage/EnvironmentsPage.tsx';
import { ContractsPage } from '@pages/ContractsPage/ContractsPage.tsx';
import { useApp } from '@/App.logic.ts';

export const App = () => {
  const logic = useApp();

  return (
    <div className="app-container">
      <VSCodePanels activeid={logic.resourceManager.openingPanelId}>
        <VSCodePanelTab id="tab-wallets">WALLETS</VSCodePanelTab>
        <VSCodePanelTab id="tab-environments">ENVIRONMENTS</VSCodePanelTab>
        <VSCodePanelTab id="tab-contracts">CONTRACTS</VSCodePanelTab>
        <VSCodePanelView id="view-wallets">
          <WalletsPage resourceManager={logic.resourceManager} vscode={logic.vscode} />
        </VSCodePanelView>
        <VSCodePanelView id="view-environments">
          <EnvironmentsPage resourceManager={logic.resourceManager} vscode={logic.vscode} />
        </VSCodePanelView>
        <VSCodePanelView id="view-contracts">
          <ContractsPage resourceManager={logic.resourceManager} vscode={logic.vscode} />
        </VSCodePanelView>
      </VSCodePanels>
    </div>
  );
};
