import Loader from '@/components/Loader.tsx';
import { VSCode } from '@/types';
import { DeployContracts, Environments, Wallets } from '@backend/actions/types';
import { useDeployUsingContract } from '@components/deploy/contract/DeployUsingContract.logic.ts';
import {
  VSCodeButton,
  VSCodeDivider,
  VSCodeDropdown,
  VSCodeOption,
  VSCodeTextField,
} from '@vscode/webview-ui-toolkit/react';
import './DeployUsingContract.css';
import { DeployContractParams } from './params/DeployContractParams.tsx';

export const DeployUsingContract = ({
  wallets,
  deployContracts,
  vscode,
  environments,
  isPending,
  setIsPending,
}: {
  wallets: Wallets;
  deployContracts: DeployContracts;
  vscode: VSCode;
  environments: Environments;
  isPending: boolean;
  setIsPending: (isPending: boolean) => void;
}) => {
  const logic = useDeployUsingContract(vscode, wallets, deployContracts, environments, setIsPending);

  return (
    <div>
      <div>
        <div className="title-contract">Deploy using contract</div>
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
            <VSCodeButton className="add-wallet-button" onClick={() => logic.openPanel('tab-wallets')}>
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
            <VSCodeButton className="add-wallet-button" onClick={() => logic.openPanel('tab-environments')}>
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
      <DeployContractParams contracts={deployContracts} />
      <VSCodeDivider className="divider" />
      <VSCodeButton className="submit-button" appearance="primary" type="submit">
        Deploy with contract
      </VSCodeButton>
      {isPending && !logic.response && <Loader />}
      {logic.response && (
        <div className={logic.response.exitCode !== 0 ? 'error-message' : ''}>
          <VSCodeDivider className="divider" />
          {logic.response.output}
        </div>
      )}
    </div>
  );
};
