import './GetStartedPage.css';
import { VSCodeButton } from '@vscode/webview-ui-toolkit/react';

export const GetStartedPage = () => {
  return (
    <div className="getStarted-container">
      <div className="text-container">
        <h1 className="title">Get Started with Osmium</h1>
        <p className="subtitle">Discover Osmium Solidity's features</p>
      </div>
      <div className="button-container">
        <VSCodeButton className="documentation-button">
          Documentation
        </VSCodeButton>
        <VSCodeButton className="walkthrough-button">
          Walkthrough
        </VSCodeButton>
      </div>
    </div>
  );
};
