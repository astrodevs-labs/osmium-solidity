import { DFormScript } from '@/types';
import { useFormContext } from 'react-hook-form';

export const useDeployScript = () => {
  const form = useFormContext<DFormScript>();

  return { form };
};