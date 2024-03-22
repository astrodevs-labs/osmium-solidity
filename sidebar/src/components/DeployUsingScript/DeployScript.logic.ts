import { useFormContext } from 'react-hook-form';
import { DFormScript } from '../../types';

export const useDeployScript = () => {
  const form = useFormContext<DFormScript>();

  return { form };
};