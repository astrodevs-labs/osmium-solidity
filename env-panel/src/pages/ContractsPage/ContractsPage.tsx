import './ContractsPage.css';
import { ResourceManager } from '@hooks/useResourceManager.ts';
import { VSCode } from '@/types';
import { EditableDataGrid } from '@components/EditableDataGrid/EditableDataGrid.tsx';
import { useContractsPageLogic } from '@pages/ContractsPage/ContractsPage.logic.ts';
import { VSCodeButton, VSCodeTextField } from '@vscode/webview-ui-toolkit/react';
import { FormProvider } from 'react-hook-form';

export const ContractsPage = (props: { vscode: VSCode; resourceManager: ResourceManager }) => {
  const logic = useContractsPageLogic(props.vscode);

  return (
    <FormProvider {...logic.form}>
      <form className="contracts-container" onSubmit={logic.form.handleSubmit(logic.onSubmit)}>
        <div style={{ display: 'flex', flexDirection: 'row', gap: '1rem', marginBottom: '1rem' }}>
          <div style={{ width: '100%', display: 'flex', flexDirection: 'column' }}>
            <VSCodeTextField
              {...logic.form.register(`name`, {
                required: true,
              })}
            >
              Name
            </VSCodeTextField>
            {logic.form.formState.errors.name && (
              <span style={{ color: 'var(--vscode-errorForeground)' }}>Invalid name</span>
            )}
          </div>
          <div style={{ width: '100%', display: 'flex', flexDirection: 'column' }}>
            <VSCodeTextField
              {...logic.form.register(`address`, {
                required: true,
              })}
            >
              Address
            </VSCodeTextField>
            {logic.form.formState.errors.address && (
              <span style={{ color: 'var(--vscode-errorForeground)' }}>Invalid address</span>
            )}
          </div>
        </div>
        <div style={{ display: 'flex', flexDirection: 'row', gap: '1rem', marginBottom: '1rem' }}>
          <div style={{ width: '100%', display: 'flex', flexDirection: 'column' }}>
            <VSCodeTextField
              {...logic.form.register(`rpc`, {
                required: true,
              })}
            >
              Rpc
            </VSCodeTextField>
            {logic.form.formState.errors.rpc && (
              <span style={{ color: 'var(--vscode-errorForeground)' }}>Invalid rpc</span>
            )}
          </div>
        </div>
        <div style={{ display: 'flex', flexDirection: 'column', gap: '1rem', marginBottom: '1rem', width: '100%' }}>
          <VSCodeTextField
            {...logic.form.register(`abi`, {
              required: true,
            })}
          >
            Abi
          </VSCodeTextField>
          {logic.form.formState.errors.abi && (
            <span style={{ color: 'var(--vscode-errorForeground)' }}>Invalid RPC</span>
          )}
        </div>
        <VSCodeButton style={{ width: '100%', marginBottom: '1rem' }} type="submit">
          Add
        </VSCodeButton>
        <EditableDataGrid
          headers={['Address', 'Abi', 'Chain ID', 'Name', 'Rpc']}
          data={props.resourceManager.interactContracts}
          deleteCallback={logic.deleteContract}
          editCallback={logic.editContract}
          gridId="contracts-grid"
        />
      </form>
    </FormProvider>
  );
};
