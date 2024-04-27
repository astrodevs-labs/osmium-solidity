import { VSCode } from '@/types';
import { InteractContracts } from '@components/interact/contracts/InteractContracts.tsx';
import { InteractParams } from '@components/interact/contracts/params/InteractParams.tsx';
import { VSCodeButton, VSCodeDivider } from '@vscode/webview-ui-toolkit/react';
import { FormProvider } from 'react-hook-form';
import './InteractPage.css';
import { useInteractPage } from './InteractPage.logic.ts';
import { ResourceManager } from '@hooks/useResourceManager.ts';
import { MessageType } from '@backend/enums.ts';

const Response = (result: { responseType: MessageType; data: string }) => {
  return (
    <div>
      <VSCodeDivider className="divider" />
      <p>{result.responseType === MessageType.READ ? 'Read response:' : 'Transaction hash:'}</p>
      <p>{result.data}</p>
    </div>
  );
};

export const InteractPage = (props: { vscode: VSCode; resourceManager: ResourceManager }) => {
  const logic = useInteractPage(props.vscode, props.resourceManager);

  return (
    <div className="page-container">
      <FormProvider {...logic.form}>
        <form onSubmit={logic.form.handleSubmit(logic.onSubmit)}>
          <InteractContracts wallets={logic.wallets} contracts={logic.contracts} vscode={props.vscode} />
          <VSCodeDivider className="divider" />
          <InteractParams contracts={logic.contracts} />
          <VSCodeButton className="submit-button" type="submit">
            Send transaction
          </VSCodeButton>
          {logic.response && Response({ ...logic.response })}
        </form>
      </FormProvider>
    </div>
  );
};
