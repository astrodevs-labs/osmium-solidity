import { ContractForm } from '@/types';
import { z, ZodType } from 'zod';

export const ContractSchema: ZodType<ContractForm> = z.object({
  name: z
    .string()
    .min(1, { message: 'Name should be minimum 1 character long' })
    .max(20, { message: 'Name should be maximum 20 characters long' }),
  address: z
    .string()
    .min(42, {
      message: 'Address should be 42 characters long',
    })
    .max(42, { message: 'Address should be 42 characters long' })
    .startsWith('0x', { message: 'Address should start with 0x' }),
  chainId: z.number(),
  rpc: z.string().url({ message: 'RPC URL should be a valid URL' }),
  abi: z.string(),
});
