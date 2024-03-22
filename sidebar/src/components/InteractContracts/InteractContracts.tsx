import './InteractContracts.css';
import { InteractContract, InteractWallet } from '../../../../vscode/src/actions/types';
import { useInteractContracts } from './InteractContracts.logic.ts';
import { VSCodeButton, VSCodeDropdown, VSCodeOption, VSCodeTextField } from '@vscode/webview-ui-toolkit/react';
import {VSCode} from "../../types";

export const InteractContracts = (props: { wallets: InteractWallet[], contracts: InteractContract[], vscode: VSCode }) => {
  const { wallets, contracts, vscode } = props;
  const logic = useInteractContracts(contracts, vscode);

  return (
    <div>
      <div className="dropdown-container">
        <label htmlFor="dropdown-wallets" className="label">Select account:</label>
        <div className="wallet-container">
          <VSCodeDropdown id="dropdown-wallets" className="dropdown-wallets" {...logic.register('wallet', {
            required: true,
          })}>
            {wallets?.map((wallet) => (
              <VSCodeOption value={wallet.address}>{wallet.name} - {wallet.address}</VSCodeOption>
            ))}
          </VSCodeDropdown>
          <VSCodeButton className="add-wallet-button" onClick={logic.editWallet}>Edit</VSCodeButton>
        </div>
      </div>
      <div className="dropdown-container">
        <label htmlFor="dropdown-contracts" className="label">Select contract:</label>
        <div className="contract-container">
          <VSCodeDropdown id="dropdown-contracts"
                          className="dropdown-contracts"  {...logic.register('contract', { required: true })}>
            {contracts?.map((contract) => (
              <VSCodeOption value={contract.address}>{contract.name} - {contract.address}</VSCodeOption>
            ))}
          </VSCodeDropdown>
          <VSCodeButton className="add-contract-button" onClick={logic.editContract}>Edit</VSCodeButton>
        </div>
      </div>
      <div className="dropdown-container">
        <label htmlFor="dropdown-functions" className="label">Select function:</label>
        <VSCodeDropdown id="dropdown-functions" {...logic.register('function', { required: true })}>
          {logic.functions?.map((func) => {
            if (!func) return null;
            return <VSCodeOption value={func}>{func}</VSCodeOption>;
          })}
        </VSCodeDropdown>
      </div>
      <div className="gas-limit-container">
        <VSCodeTextField className="gas-limit-textfield" {...logic.register('gasLimit', {
          required: true,
          valueAsNumber: true,
        })}>Gas
          limit</VSCodeTextField>
        {logic.errors.gasLimit && <span className="error-message">Invalid number</span>}
      </div>
      <div className="value-container">
        <label className="label">Value:</label>
        <div className="value-field-container">
          <VSCodeTextField className="value-textfield" {...logic.register('value', {
            required: true,
            valueAsNumber: true,
          })} />
          <VSCodeDropdown className="value-dropdown" id="dropdown" {...logic.register('valueUnit', {
            required: true,
          })}>
            <VSCodeOption value="wei">Wei</VSCodeOption>
            <VSCodeOption value="gwei">Gwei</VSCodeOption>
            <VSCodeOption value="ether">Eth</VSCodeOption>
          </VSCodeDropdown>
        </div>
        {logic.errors.value && <span className="error-message">Invalid number</span>}
      </div>
    </div>
  );
};
