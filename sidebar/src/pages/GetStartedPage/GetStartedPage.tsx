import './GetStartedPage.css';
import { VSCodeButton } from '@vscode/webview-ui-toolkit/react';

export const GetStartedPage = () => {
  return (
    <div className="page-container">
      <h1 className="title">Get Started with Osmium </h1>
      <p className="subtitle">Explore all of the features in Osmium Solidity</p>
      <VSCodeButton className="documentation-button">
        Documentation
      </VSCodeButton>
      <VSCodeButton className="walkthrough-button">
        Walkthrough
      </VSCodeButton>
    </div>
  );
};
