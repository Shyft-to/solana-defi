import {
  ConfirmedTransactionMeta,
  Message,
  MessageV0,
  PublicKey,
  VersionedMessage,
  VersionedTransactionResponse,
} from "@solana/web3.js";
import { utils } from "@project-serum/anchor";

export class TransactionFormatter {
  public formTransactionFromJson(
    data: any,
    time: number,
  ): VersionedTransactionResponse {
    const rawTx = data["transaction"];

    const slot = data.slot;
    const version = rawTx.transaction.message.versioned ? 0 : "legacy";

    const meta = this.formMeta(rawTx.meta);
    const signatures = rawTx.transaction.signatures.map((s: Buffer) =>
      utils.bytes.bs58.encode(s),
    );

    const message = this.formTxnMessage(rawTx.transaction.message);

    return {
      slot,
      version,
      blockTime: time,
      meta,
      transaction: {
        signatures,
        message,
      },
    };
  }

  private formTxnMessage(message: any): VersionedMessage {
    if (!message.versioned) {
      return new Message({
        header: {
          numRequiredSignatures: message.header.numRequiredSignatures,
          numReadonlySignedAccounts: message.header.numReadonlySignedAccounts,
          numReadonlyUnsignedAccounts:
            message.header.numReadonlyUnsignedAccounts,
        },
        recentBlockhash: utils.bytes.bs58.encode(
          Buffer.from(message.recentBlockhash, "base64"),
        ),
        accountKeys: message.accountKeys?.map((d: string) =>
          Buffer.from(d, "base64"),
        ),
        instructions: message.instructions.map(
          ({
            data,
            programIdIndex,
            accounts,
          }: {
            data: any;
            programIdIndex: any;
            accounts: any;
          }) => ({
            programIdIndex: programIdIndex,
            accounts: [...accounts] || [],
            data: utils.bytes.bs58.encode(Buffer.from(data || "", "base64")),
          }),
        ),
      });
    } else {
      return new MessageV0({
        header: {
          numRequiredSignatures: message.header.numRequiredSignatures,
          numReadonlySignedAccounts: message.header.numReadonlySignedAccounts,
          numReadonlyUnsignedAccounts:
            message.header.numReadonlyUnsignedAccounts,
        },
        recentBlockhash: utils.bytes.bs58.encode(
          Buffer.from(message.recentBlockhash, "base64"),
        ),
        staticAccountKeys: message.accountKeys.map(
          (k: string) => new PublicKey(Buffer.from(k, "base64")),
        ),
        compiledInstructions: message.instructions.map(
          ({
            programIdIndex,
            accounts,
            data,
          }: {
            programIdIndex: any;
            accounts: any;
            data: any;
          }) => ({
            programIdIndex: programIdIndex,
            accountKeyIndexes: [...accounts] || [],
            data: Uint8Array.from(Buffer.from(data || "", "base64")),
          }),
        ),
        addressTableLookups:
          message.addressTableLookups?.map(
            ({
              accountKey,
              writableIndexes,
              readonlyIndexes,
            }: {
              accountKey: any;
              writableIndexes: any;
              readonlyIndexes: any;
            }) => ({
              writableIndexes: writableIndexes || [],
              readonlyIndexes: readonlyIndexes || [],
              accountKey: new PublicKey(Buffer.from(accountKey, "base64")),
            }),
          ) || [],
      });
    }
  }

  private formMeta(meta: any): ConfirmedTransactionMeta {
    return {
      err: meta.errorInfo ? { err: meta.errorInfo } : null,
      fee: meta.fee,
      preBalances: meta.preBalances,
      postBalances: meta.postBalances,
      preTokenBalances: meta.preTokenBalances || [],
      postTokenBalances: meta.postTokenBalances || [],
      logMessages: meta.logMessages || [],
      loadedAddresses:
        meta.loadedWritableAddresses || meta.loadedReadonlyAddresses
          ? {
              writable:
                meta.loadedWritableAddresses?.map(
                  (address: string) =>
                    new PublicKey(Buffer.from(address, "base64")),
                ) || [],
              readonly:
                meta.loadedReadonlyAddresses?.map(
                  (address: string) =>
                    new PublicKey(Buffer.from(address, "base64")),
                ) || [],
            }
          : undefined,
      innerInstructions:
        meta.innerInstructions?.map(
          (i: { index: number; instructions: any }) => ({
            index: i.index || 0,
            instructions: i.instructions.map((instruction: any) => ({
              programIdIndex: instruction.programIdIndex,
              accounts: [...instruction.accounts] || [],
              data: utils.bytes.bs58.encode(
                Buffer.from(instruction.data || "", "base64"),
              ),
            })),
          }),
        ) || [],
    };
  }
}
