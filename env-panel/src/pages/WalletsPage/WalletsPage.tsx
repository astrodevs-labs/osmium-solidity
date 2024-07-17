import './WalletsPage.css';
import { ResourceManager } from '@hooks/useResourceManager.ts';
import { VSCode } from '@/types';
import { EditableDataGrid } from '@components/EditableDataGrid/EditableDataGrid.tsx';
import { useWalletsPageLogic } from '@pages/WalletsPage/WalletsPage.logic.ts';

export const WalletsPage = (props: { vscode: VSCode; resourceManager: ResourceManager }) => {
  const logic = useWalletsPageLogic(props.vscode);

  return (
    <div className="wallets-container">
      <EditableDataGrid
        headers={['Name', 'Address', 'Private Key']}
        data={props.resourceManager.wallets}
        deleteCallback={logic.deleteWallet}
        editCallback={logic.editWallet}
      />
    </div>
  );
};
