import './InteractPage.css';
import { ResponseType, useInteractPage } from './InteractPage.logic.ts';
import { VSCode } from '../../types';
import { FormProvider } from 'react-hook-form';
import { VSCodeButton, VSCodeDivider } from '@vscode/webview-ui-toolkit/react';
import { InteractContracts } from '../../components/InteractContracts/InteractContracts.tsx';
import { InteractParams } from '../../components/InteractParams/InteractParams.tsx';

export const InteractPage = (props: { vscode: VSCode }) => {
  const logic = useInteractPage(props.vscode);

  return (
    <div className="page-container">
      <FormProvider {...logic.form} >
        <form onSubmit={logic.form.handleSubmit(logic.onSubmit)}>
          <InteractContracts wallets={logic.wallets} contracts={logic.contracts} vscode={props.vscode} />
          <VSCodeDivider className="divider" />
          <InteractParams contracts={logic.contracts} />
          <VSCodeButton className="submit-button" type="submit">Send transaction</VSCodeButton>
          {logic.result &&
            <div>
              <VSCodeDivider className="divider" />
              <p>{logic.result.responseType === ResponseType.READ ? 'Read response:' : 'Transaction hash:'}</p>
              <p>{logic.result.data + ''}</p>
            </div>
          }
        </form>
      </FormProvider>
    </div>
  );
};