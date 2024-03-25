import { VSCode } from '@/types';
import { InteractContract, InteractWallet } from '@backend/actions/types';
import { useInteractContracts } from '@hooks/useInteractContracts.ts';
import { VSCodeButton, VSCodeDropdown, VSCodeOption, VSCodeTextField } from '@vscode/webview-ui-toolkit/react';
import './InteractContracts.css';

interface InteractContractsProps {
  wallets: InteractWallet[];
  contracts: InteractContract[];
  vscode: VSCode;
}

interface DropdownContainerProps {
  id: string;
  label: string;
  options: InteractContract[] | (string | undefined)[];
  logic: any;
}

const DropdownContainer = ({ id, label, options, logic }: DropdownContainerProps) => (
  <div className="dropdown-container">
    <label htmlFor={id} className="label">
      {label}:
    </label>
    <div className="contract-container">
      <VSCodeDropdown id={id} className={`dropdown-${id}`} {...logic.register(id, { required: true })}>
        {options?.map((option) => {
          if (typeof option === 'string' || option === undefined) {
            return <VSCodeOption value={option}>{option}</VSCodeOption>;
          } else {
            return (
              <VSCodeOption value={option.address}>
                {option.name} - {option.address}
              </VSCodeOption>
            );
          }
        })}
      </VSCodeDropdown>
      <VSCodeButton className={`add-${id}-button`} onClick={logic[`edit${id.charAt(0).toUpperCase() + id.slice(1)}`]}>
        Edit
      </VSCodeButton>
    </div>
  </div>
);

export const InteractContracts = ({ wallets, contracts, vscode }: InteractContractsProps) => {
  const logic = useInteractContracts(contracts, vscode);

  return (
    <div>
      <div className="dropdown-container">
        <label htmlFor="dropdown-wallets" className="label">
          Select account:
        </label>
        <div className="wallet-container">
          <VSCodeDropdown
            id="dropdown-wallets"
            className="dropdown-wallets"
            {...logic.register('wallet', {
              required: true,
            })}
          >
            {wallets?.map((wallet) => (
              <VSCodeOption value={wallet.address}>
                {wallet.name} - {wallet.address}
              </VSCodeOption>
            ))}
          </VSCodeDropdown>
          <VSCodeButton className="add-wallet-button" onClick={logic.editWallet}>
            Edit
          </VSCodeButton>
        </div>
      </div>
      <DropdownContainer id="contracts" label="Select contract" options={contracts} logic={logic} />
      <DropdownContainer id="functions" label="Select function" options={logic.functions} logic={logic} />
      <div className="gas-limit-container">
        <VSCodeTextField
          className="gas-limit-textfield"
          {...logic.register('gasLimit', {
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
            {...logic.register('value', {
              required: true,
              valueAsNumber: true,
            })}
          />
          <VSCodeDropdown
            className="value-dropdown"
            id="dropdown"
            {...logic.register('valueUnit', {
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
  );
};
