/* eslint-disable @typescript-eslint/no-explicit-any */
import { useEffect } from 'react';

export const useEditableDataGridLogic = (
  deleteCallback: (id: string) => void,
  editCallback: (id: string, key: string, value: string) => void,
) => {
  const dataGrid = document.getElementById('data-grid');

  useEffect(() => {
    if (dataGrid) {
      dataGrid.onclick = cellClick;
    }
  }, [dataGrid]);

  const cellClick = (cell: any) => {
    const { srcElement } = cell;
    if (srcElement && srcElement.id === 'editable-cell') {
      const handleChange = (target: any) => {
        const newValue = target.textContent;

        editCallback(srcElement.className.split(' ')[0], srcElement.className.split(' ')[1], newValue);

        srcElement.setAttribute('contenteditable', 'false');

        srcElement.onkeydown = undefined;
        srcElement.onblur = undefined;
      };

      srcElement.onkeydown = (e: any) => {
        if (e.code === 'Enter') {
          handleChange(e.target);

          return false;
        }
      };

      srcElement.onblur = (e: any) => {
        handleChange(e.target);
      };

      srcElement.setAttribute('contenteditable', 'true');
    }
  };

  const deleteRow = async (event: any) => {
    const id = event.target.id;
    deleteCallback(id);
  };

  return {
    deleteRow,
  };
};
