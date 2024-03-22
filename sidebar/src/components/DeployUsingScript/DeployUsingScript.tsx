import {
  VSCodeButton,
  VSCodeDropdown,
  VSCodeOption,
  VSCodeDivider
} from "@vscode/webview-ui-toolkit/react";
import { InteractWallet, DeployScript, InteractContract } from "../../../../vscode/src/actions/types";
import "./DeployUsingScript.css";
import { useDeployScript } from "./DeployScript.logic.ts";
import { useInteractContracts } from '../InteractContracts/InteractContracts.logic.ts';
import {VSCode} from "../../types";

export const DeployUsingScript = (
  { wallets, scripts, vscode, contracts }: { wallets: InteractWallet[]; scripts: DeployScript[], vscode: VSCode, contracts: InteractContract[]},
) => {
  const logic = useDeployScript();
  const edit = useInteractContracts(contracts, vscode);

  return (
    <div>
      <div>
        <div>DEPLOY USING SCRIPT</div>
        <div className="dropdown-container">
          <label htmlFor="dropdown-wallets" className="label">
            Select account:
          </label>
          <div className="wallet-container">
            <VSCodeDropdown
              id="dropdown-wallets" className='dropdown-wallets'
              {...logic.form?.register("wallet", {
                required: true,
              })}
            >
              {wallets?.map((wallet) => (
                <VSCodeOption value={wallet.address}>
                  {wallet.name} - {wallet.address}
                </VSCodeOption>
              ))}
            </VSCodeDropdown>
            <VSCodeButton className="add-wallet-button" onClick={edit.editWallet}>Edit</VSCodeButton>
          </div>
        </div>
        <div className="dropdown-container">
          <label htmlFor="dropdown" className="label">Select script:</label>
          <VSCodeDropdown 
            id="dropdown"
            {...logic.form?.register("script", {
              required: true,
            })}
          >
            {scripts?.map((scripts) => (
              <VSCodeOption>{scripts.name} ({scripts.path})</VSCodeOption>
            ))}
          </VSCodeDropdown>
        </div>
      </div>
      <VSCodeDivider className='divider'/>
      <VSCodeButton className="submit-button" type="submit" >
        Deploy with script
      </VSCodeButton>
      <VSCodeDivider className='divider'/>
    </div>
  );
};
