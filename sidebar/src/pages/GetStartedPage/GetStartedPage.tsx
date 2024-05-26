import './GetStartedPage.css';
import { VSCodeButton } from '@vscode/webview-ui-toolkit/react';
import { useState } from 'react';

export const GetStartedPage = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [isVisible, setIsVisible] = useState(true);

  return isVisible && (
    <div className="getStarted-container">
      <div className="text-container" onClick={() => setIsOpen(!isOpen)}>
        <div className="close-button" onClick={() => setIsVisible(false)}>X</div>
        <h1 className="title">
          <span style={{ cursor: 'pointer' }}>{isOpen ? '▼' : '►'}</span>
          Get Started with Osmium 🚀
        </h1>
        <p className="subtitle">Unfold to discover Osmium Solidity's features</p>
      </div>
      {isOpen && (
        <div className="button-container">
          <VSCodeButton className="documentation-button">Documentation</VSCodeButton>
          <VSCodeButton className="walkthrough-button">Walkthrough</VSCodeButton>
        </div>
      )}
    </div>
  );
};
