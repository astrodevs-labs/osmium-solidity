import { VSCode } from '@/types';
import { DeployContracts, Environment, InteractContracts, Wallets } from '@backend/actions/types';
import { useDeployContract } from '@hooks/useDeployContract.ts';
import { useInteractContracts } from '@hooks/useInteractContracts.ts';
import {
  VSCodeButton,
  VSCodeDivider,
  VSCodeDropdown,
  VSCodeOption,
  VSCodeTextField,
} from '@vscode/webview-ui-toolkit/react';
import './DeployUsingContract.css';
import { DeployContractsParams } from './params/DeployContractsParams.tsx';

export const DeployUsingContract = ({
  wallets,
  deployContracts,
  vscode,
  editContracts,
  environments,
}: {
  wallets: Wallets;
  deployContracts: DeployContracts;
  vscode: VSCode;
  editContracts: InteractContracts;
  environments: Environment[];
}) => {
  const logicContract = useDeployContract(vscode);
  const edit = useInteractContracts(editContracts, vscode);

  return (
    <div>
      <div>
        <div> DEPLOY USING CONTRACT </div>
        <div className="dropdown-container">
          <label htmlFor="dropdown-wallets" className="label">
            Select account:
          </label>
          <div className="wallet-container">
            <VSCodeDropdown
              id="dropdown-wallets"
              className="dropdown-wallets"
              {...logicContract.form?.register('wallet', {
                required: true,
              })}
            >
              {wallets?.map((wallet) => (
                <VSCodeOption value={wallet.id}>
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
            Select contract:
          </label>
          <VSCodeDropdown id="dropdown" {...logicContract.form?.register('contract', { required: true })}>
            {deployContracts?.map((deployContracts) => (
              <VSCodeOption value={deployContracts.id}>
                {' '}
                {deployContracts.name} ({deployContracts.path})
              </VSCodeOption>
            ))}
          </VSCodeDropdown>
        </div>
        <div className="environment-container">
          <VSCodeDropdown
            id="dropdown-environment"
            className="dropdown-environment"
            {...logicContract.form?.register('environment', { required: true })}
          >
            {environments.map((environment) => (
              <VSCodeOption>
                {environment.name} ({environment.rpc})
              </VSCodeOption>
            ))}
          </VSCodeDropdown>
          <VSCodeButton className="add-wallet-button" onClick={logicContract.editEnvironment}>
            Edit
          </VSCodeButton>
        </div>
        <div className="gas-limit-container">
          <VSCodeTextField
            className="gas-limit-textfield"
            {...logicContract.form?.register('gasLimit', {
              required: true,
              valueAsNumber: true,
            })}
          >
            Gas limit
          </VSCodeTextField>
          {logicContract.errors.gasLimit && <span className="error-message">Invalid number</span>}
        </div>
        <div className="value-container">
          <label className="label">Value:</label>
          <div className="value-field-container">
            <VSCodeTextField
              className="value-textfield"
              {...logicContract.form?.register('value', {
                required: true,
                valueAsNumber: true,
              })}
            />
            <VSCodeDropdown
              className="value-dropdown"
              id="dropdown"
              {...logicContract.form?.register('valueUnit', {
                required: true,
              })}
            >
              <VSCodeOption value="wei">Wei</VSCodeOption>
              <VSCodeOption value="gwei">Gwei</VSCodeOption>
              <VSCodeOption value="ether">Eth</VSCodeOption>
            </VSCodeDropdown>
          </div>
          {logicContract.errors.value && <span className="error-message">Invalid number</span>}
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
