import './WalletsPage.css';
import { ResourceManager } from '@hooks/useResourceManager.ts';
import { VSCode } from '@/types';
import { EditableDataGrid } from '@components/EditableDataGrid/EditableDataGrid.tsx';
import { useWalletsPageLogic } from '@pages/WalletsPage/WalletsPage.logic.ts';
import { VSCodeButton, VSCodeTextField } from '@vscode/webview-ui-toolkit/react';
import { FormProvider } from 'react-hook-form';

export const WalletsPage = (props: { vscode: VSCode; resourceManager: ResourceManager }) => {
  const logic = useWalletsPageLogic(props.vscode);

  return (
    <FormProvider {...logic.form}>
      <form className="wallets-container" onSubmit={logic.form.handleSubmit(logic.onSubmit)}>
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
              <span style={{ color: 'var(--vscode-errorForeground)' }}>Name is required</span>
            )}
          </div>
          <div style={{ width: '100%', display: 'flex', flexDirection: 'column' }}>
            <VSCodeTextField
              {...logic.form.register(`privateKey`, {
                required: true,
              })}
            >
              Private Key
            </VSCodeTextField>
            {logic.form.formState.errors.privateKey && (
              <span style={{ color: 'var(--vscode-errorForeground)' }}>Private Key is required</span>
            )}
          </div>
        </div>
        <VSCodeButton style={{ width: '100%', marginBottom: '1rem' }} type="submit">
          Add
        </VSCodeButton>
        <div style={{ display: 'flex', flexDirection: 'column' }}></div>
        <EditableDataGrid
          headers={['Name', 'Address', 'Private Key']}
          data={props.resourceManager.wallets}
          deleteCallback={logic.deleteWallet}
          editCallback={logic.editWallet}
          gridId="wallets-grid"
        />
      </form>
    </FormProvider>
  );
};
