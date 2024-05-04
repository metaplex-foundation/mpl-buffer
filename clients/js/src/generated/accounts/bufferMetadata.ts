/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Account,
  Context,
  Pda,
  PublicKey,
  RpcAccount,
  RpcGetAccountOptions,
  RpcGetAccountsOptions,
  assertAccountExists,
  deserializeAccount,
  gpaBuilder,
  publicKey as toPublicKey,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  publicKey as publicKeySerializer,
  string,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import { Key, KeyArgs, getKeySerializer } from '../types';

export type BufferMetadata = Account<BufferMetadataAccountData>;

export type BufferMetadataAccountData = {
  key: Key;
  buffer: PublicKey;
  bump: number;
  authority: PublicKey;
};

export type BufferMetadataAccountDataArgs = {
  key: KeyArgs;
  buffer: PublicKey;
  bump: number;
  authority: PublicKey;
};

export function getBufferMetadataAccountDataSerializer(): Serializer<
  BufferMetadataAccountDataArgs,
  BufferMetadataAccountData
> {
  return struct<BufferMetadataAccountData>(
    [
      ['key', getKeySerializer()],
      ['buffer', publicKeySerializer()],
      ['bump', u8()],
      ['authority', publicKeySerializer()],
    ],
    { description: 'BufferMetadataAccountData' }
  ) as Serializer<BufferMetadataAccountDataArgs, BufferMetadataAccountData>;
}

export function deserializeBufferMetadata(
  rawAccount: RpcAccount
): BufferMetadata {
  return deserializeAccount(
    rawAccount,
    getBufferMetadataAccountDataSerializer()
  );
}

export async function fetchBufferMetadata(
  context: Pick<Context, 'rpc'>,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions
): Promise<BufferMetadata> {
  const maybeAccount = await context.rpc.getAccount(
    toPublicKey(publicKey, false),
    options
  );
  assertAccountExists(maybeAccount, 'BufferMetadata');
  return deserializeBufferMetadata(maybeAccount);
}

export async function safeFetchBufferMetadata(
  context: Pick<Context, 'rpc'>,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions
): Promise<BufferMetadata | null> {
  const maybeAccount = await context.rpc.getAccount(
    toPublicKey(publicKey, false),
    options
  );
  return maybeAccount.exists ? deserializeBufferMetadata(maybeAccount) : null;
}

export async function fetchAllBufferMetadata(
  context: Pick<Context, 'rpc'>,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions
): Promise<BufferMetadata[]> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options
  );
  return maybeAccounts.map((maybeAccount) => {
    assertAccountExists(maybeAccount, 'BufferMetadata');
    return deserializeBufferMetadata(maybeAccount);
  });
}

export async function safeFetchAllBufferMetadata(
  context: Pick<Context, 'rpc'>,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions
): Promise<BufferMetadata[]> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options
  );
  return maybeAccounts
    .filter((maybeAccount) => maybeAccount.exists)
    .map((maybeAccount) =>
      deserializeBufferMetadata(maybeAccount as RpcAccount)
    );
}

export function getBufferMetadataGpaBuilder(
  context: Pick<Context, 'rpc' | 'programs'>
) {
  const programId = context.programs.getPublicKey(
    'mplBuffer',
    'BUFFCb9CHNtEVhhTmZ8LBdzLgWQbvCxtcy7iNv3RZCLH'
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      key: KeyArgs;
      buffer: PublicKey;
      bump: number;
      authority: PublicKey;
    }>({
      key: [0, getKeySerializer()],
      buffer: [1, publicKeySerializer()],
      bump: [33, u8()],
      authority: [34, publicKeySerializer()],
    })
    .deserializeUsing<BufferMetadata>((account) =>
      deserializeBufferMetadata(account)
    );
}

export function getBufferMetadataSize(): number {
  return 66;
}

export function findBufferMetadataPda(
  context: Pick<Context, 'eddsa' | 'programs'>,
  seeds: {
    /** The address of the buffer account */
    buffer: PublicKey;
  }
): Pda {
  const programId = context.programs.getPublicKey(
    'mplBuffer',
    'BUFFCb9CHNtEVhhTmZ8LBdzLgWQbvCxtcy7iNv3RZCLH'
  );
  return context.eddsa.findPda(programId, [
    string({ size: 'variable' }).serialize('Buffer'),
    publicKeySerializer().serialize(programId),
    publicKeySerializer().serialize(seeds.buffer),
  ]);
}

export async function fetchBufferMetadataFromSeeds(
  context: Pick<Context, 'eddsa' | 'programs' | 'rpc'>,
  seeds: Parameters<typeof findBufferMetadataPda>[1],
  options?: RpcGetAccountOptions
): Promise<BufferMetadata> {
  return fetchBufferMetadata(
    context,
    findBufferMetadataPda(context, seeds),
    options
  );
}

export async function safeFetchBufferMetadataFromSeeds(
  context: Pick<Context, 'eddsa' | 'programs' | 'rpc'>,
  seeds: Parameters<typeof findBufferMetadataPda>[1],
  options?: RpcGetAccountOptions
): Promise<BufferMetadata | null> {
  return safeFetchBufferMetadata(
    context,
    findBufferMetadataPda(context, seeds),
    options
  );
}
