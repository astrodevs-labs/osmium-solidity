import { WalletForm } from '@/types';
import { z, ZodType } from 'zod';

export const WalletSchema: ZodType<WalletForm> = z.object({
  name: z
    .string()
    .min(1, { message: 'Name should be minimum 1 character long' })
    .max(20, { message: 'Name should be maximum 20 characters long' }),
  privateKey: z
    .string()
    .min(66, {
      message: 'Private key should be 66 characters long',
    })
    .max(66, { message: 'Private key should be 66 characters long' })
    .startsWith('0x', { message: 'Private key should start with 0x' }),
});
