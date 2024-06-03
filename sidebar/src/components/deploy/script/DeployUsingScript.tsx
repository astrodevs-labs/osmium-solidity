import Loader from '@/pages/DeployPage/loader.tsx';
import { VSCode } from '@/types';
import { Environments, Scripts } from '@backend/actions/types';
import { VSCodeButton, VSCodeDivider, VSCodeDropdown, VSCodeOption } from '@vscode/webview-ui-toolkit/react';
import './DeployUsingScript.css';
import { useDeployUsingScript } from './DeployUsingScript.logic.ts';

export const DeployUsingScript = ({
  scripts,
  vscode,
  environments,
  isPending,
  setIsPending,
}: {
  scripts: Scripts;
  vscode: VSCode;
  environments: Environments;
  isPending: boolean;
  setIsPending: (isPending: boolean) => void;
}) => {
  const logic = useDeployUsingScript(vscode, scripts, environments, setIsPending);

  return (
    <div>
      <div>
        <div style={{ fontWeight: 'bold' }}>Deploy using script</div>
        <div className="dropdown-container">
          <div className="dropdown-container">
            <label htmlFor="dropdown-environment" className="label">
              Environment:
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
            {scripts?.map((script) => (
              <VSCodeOption value={script.id}>
                {script.name} ({script.path})
              </VSCodeOption>
            ))}
          </VSCodeDropdown>
        </div>
      </div>
      <VSCodeDivider className="divider" />
      <VSCodeButton className="submit-button" type="submit">
        Deploy with script
      </VSCodeButton>
      {isPending && !logic.response && <Loader />}
      <VSCodeDivider className="divider" />
      {logic.response && (
        <div className={logic.response.exitCode !== 0 ? 'error-message' : ''}>
          {logic.response.output}
          <VSCodeDivider className="divider" />
        </div>
      )}
    </div>
  );
};
