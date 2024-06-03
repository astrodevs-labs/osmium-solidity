import './GetStartedPage.css';
import { VSCodeButton } from '@vscode/webview-ui-toolkit/react';
import { MessageType } from '@backend/enums.ts';
import { useState } from 'react';
import { VSCode } from '@/types';

export const GetStartedPage = (props: { vscode: VSCode }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [isVisible, setIsVisible] = useState(true);

  const openDocumentation = () => {
    props.vscode.postMessage({ type: MessageType.OPEN_DOCUMENTATION });
  };

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
          <VSCodeButton className="documentation-button" onClick={openDocumentation}>Documentation</VSCodeButton>
          <VSCodeButton className="walkthrough-button">Walkthrough</VSCodeButton>
        </div>
      )}
    </div>
  );
};
