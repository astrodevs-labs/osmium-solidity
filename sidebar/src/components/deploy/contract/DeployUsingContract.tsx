import { VSCode } from '@/types';
import { DeployContracts, Environment, Wallets } from '@backend/actions/types';
import {
  VSCodeButton,
  VSCodeDivider,
  VSCodeDropdown,
  VSCodeOption,
  VSCodeTextField,
} from '@vscode/webview-ui-toolkit/react';
import './DeployUsingContract.css';
import { DeployContractsParams } from './params/DeployContractsParams.tsx';
import { useDeployUsingContract } from '@components/deploy/contract/DeployUsingContract.logic.ts';

export const DeployUsingContract = ({
  wallets,
  deployContracts,
  vscode,
  environments,
}: {
  wallets: Wallets;
  deployContracts: DeployContracts;
  vscode: VSCode;
  environments: Environment[];
}) => {
  const logic = useDeployUsingContract(vscode);

  return (
    <div>
      <div>
        <div> DEPLOY USING CONTRACT</div>
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
              {wallets?.map((wallet) => (
                <VSCodeOption value={wallet.id}>
                  {wallet.name} - {wallet.address}
                </VSCodeOption>
              ))}
            </VSCodeDropdown>
            <VSCodeButton className="add-wallet-button" onClick={logic.editWallet}>
              Edit
            </VSCodeButton>
          </div>
        </div>
        <div className="dropdown-container">
          <label htmlFor="dropdown" className="label">
            Select contract:
          </label>
          <VSCodeDropdown id="dropdown" {...logic.form?.register('contract', { required: true })}>
            {deployContracts?.map((deployContract) => (
              <VSCodeOption value={deployContract.id}>
                {deployContract.name} ({deployContract.path})
              </VSCodeOption>
            ))}
          </VSCodeDropdown>
        </div>
        <div className="dropdown-container">
          <label htmlFor="dropdown-environment" className="label">
            Select environment:
          </label>
          <div className="environment-container">
            <VSCodeDropdown
              id="dropdown-environment"
              className="dropdown-environment"
              {...logic.form?.register('environment', { required: true })}
            >
              {environments.map((environment) => (
                <VSCodeOption value={environment.id}>
                  {environment.name} ({environment.rpc})
                </VSCodeOption>
              ))}
            </VSCodeDropdown>
            <VSCodeButton className="add-wallet-button" onClick={logic.editEnvironment}>
              Edit
            </VSCodeButton>
          </div>
        </div>
        <div className="gas-limit-container">
          <VSCodeTextField
            className="gas-limit-textfield"
            {...logic.form?.register('gasLimit', {
              required: true,
              valueAsNumber: true,
            })}
          >
            Gas limit
          </VSCodeTextField>
          {logic.errors.gasLimit && <span className="error-message">Invalid number</span>}
        </div>
        <div className="value-container">
          <label className="label">Value:</label>
          <div className="value-field-container">
            <VSCodeTextField
              className="value-textfield"
              {...logic.form?.register('value', {
                required: true,
                valueAsNumber: true,
              })}
            />
            <VSCodeDropdown
              className="value-dropdown"
              id="dropdown"
              {...logic.form?.register('valueUnit', {
                required: true,
              })}
            >
              <VSCodeOption value="wei">Wei</VSCodeOption>
              <VSCodeOption value="gwei">Gwei</VSCodeOption>
              <VSCodeOption value="ether">Eth</VSCodeOption>
            </VSCodeDropdown>
          </div>
          {logic.errors.value && <span className="error-message">Invalid number</span>}
        </div>
      </div>
      <VSCodeDivider className="divider" />
      <DeployContractsParams contracts={deployContracts} />
      <VSCodeButton className="submit-button" type="submit">
        Deploy with contract
      </VSCodeButton>
    </div>
  );
};
