import { useDeployPageScript } from '@/pages/DeployPage/DeployPage.logic';
import { VSCode } from '@/types';
import { InteractContract } from '@backend/actions/types';
import { useDeployScript } from '@hooks/useDeployScript.ts';
import { useInteractContracts } from '@hooks/useInteractContracts.ts';
import { VSCodeButton, VSCodeDivider, VSCodeDropdown, VSCodeOption } from '@vscode/webview-ui-toolkit/react';
import { FormProvider } from 'react-hook-form';
import './DeployUsingScript.css';

export const DeployUsingScript = ({ vscode, contracts }: { vscode: VSCode; contracts: InteractContract[] }) => {
  const logicScript = useDeployPageScript(vscode);
  const logic = useDeployScript();
  const edit = useInteractContracts(contracts, vscode);

  return (
    <FormProvider {...logicScript.form}>
      <form onSubmit={logicScript.form.handleSubmit(logicScript.onSubmit)}>
        <div>
          <div>
            <div>DEPLOY USING SCRIPT</div>
            <div className="dropdown-container">
              <label htmlFor="dropdown-wallets" className="label">
                Select account:
              </label>
              <div className="wallet-container">
                <VSCodeDropdown
                  id="dropdown-wallets"
                  className="dropdown-wallets"
                  {...logic.form?.register('wallet', {
                    required: true,
                  })}
                >
                  {logicScript.wallets?.map((wallet) => (
                    <VSCodeOption value={wallet.address}>
                      {wallet.name} - {wallet.address}
                    </VSCodeOption>
                  ))}
                </VSCodeDropdown>
                <VSCodeButton className="add-wallet-button" onClick={edit.editWallet}>
                  Edit
                </VSCodeButton>
              </div>
            </div>
            <div className="dropdown-container">
              <label htmlFor="dropdown" className="label">
                Select script:
              </label>
              <VSCodeDropdown
                id="dropdown"
                {...logic.form?.register('script', {
                  required: true,
                })}
              >
                {logicScript.scripts?.map((scripts) => (
                  <VSCodeOption>
                    {scripts.name} ({scripts.path})
                  </VSCodeOption>
                ))}
              </VSCodeDropdown>
            </div>
          </div>
          <VSCodeDivider className="divider" />
          <VSCodeButton className="submit-button" type="submit">
            Deploy with script
          </VSCodeButton>
        </div>
      </form>
    </FormProvider>
  );
};
