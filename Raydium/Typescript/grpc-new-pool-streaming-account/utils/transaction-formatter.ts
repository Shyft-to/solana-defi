import * as base58 from "bs58";
import {
  AccountMeta,
  CompiledInstruction,
  ConfirmedTransactionMeta,
  LoadedAddresses,
  Message,
  MessageCompiledInstruction,
  MessageV0,
  PublicKey,
  TransactionInstruction,
  VersionedMessage,
  VersionedTransactionResponse,
} from "@solana/web3.js";

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
      base58.encode(s),
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
        recentBlockhash: base58.encode(
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
            data: base58.encode(Buffer.from(data || "", "base64")),
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
        recentBlockhash: base58.encode(
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
              accounts: instruction.accounts || [],
              data: base58.encode(
                Buffer.from(instruction.data || "", "base64"),
              ),
            })),
          }),
        ) || [],
    };
  }

  private parseTransactionAccounts<T extends Message | VersionedMessage>(
    message: T,
    loadedAddresses: T extends VersionedMessage
      ? LoadedAddresses | undefined
      : undefined = undefined,
  ): AccountMeta[] {
    const accounts: PublicKey[] =
      message.version === "legacy"
        ? message.accountKeys
        : message.staticAccountKeys;
    const readonlySignedAccountsCount =
      message.header.numReadonlySignedAccounts;
    const readonlyUnsignedAccountsCount =
      message.header.numReadonlyUnsignedAccounts;
    const requiredSignaturesAccountsCount =
      message.header.numRequiredSignatures;
    const totalAccounts = accounts.length;
    let parsedAccounts: AccountMeta[] = accounts.map((account, idx) => {
      const isWritable =
        idx < requiredSignaturesAccountsCount - readonlySignedAccountsCount ||
        (idx >= requiredSignaturesAccountsCount &&
          idx < totalAccounts - readonlyUnsignedAccountsCount);

      return {
        isSigner: idx < requiredSignaturesAccountsCount,
        isWritable,
        pubkey: new PublicKey(account),
      } as AccountMeta;
    });
    const [ALTWritable, ALTReadOnly] =
      message.version === "legacy"
        ? [[], []]
        : loadedAddresses
          ? [loadedAddresses.writable, loadedAddresses.readonly]
          : [[], []]; // message.getAccountKeys({ accountKeysFromLookups: loadedAddresses }).keySegments().slice(1); // omit static keys
    if (ALTWritable)
      parsedAccounts = [
        ...parsedAccounts,
        ...ALTWritable.map((pubkey) => ({
          isSigner: false,
          isWritable: true,
          pubkey,
        })),
      ];
    if (ALTReadOnly)
      parsedAccounts = [
        ...parsedAccounts,
        ...ALTReadOnly.map((pubkey) => ({
          isSigner: false,
          isWritable: false,
          pubkey,
        })),
      ];

    return parsedAccounts;
  }

  private compiledInstructionToInstruction<
    Ix extends CompiledInstruction | MessageCompiledInstruction,
  >(
    compiledInstruction: Ix,
    parsedAccounts: AccountMeta[],
  ): TransactionInstruction {
    if (typeof compiledInstruction.data === "string") {
      const ci = compiledInstruction as CompiledInstruction;
      return new TransactionInstruction({
        data: Buffer.from(base58.decode(ci.data)),
        programId: parsedAccounts[ci.programIdIndex].pubkey,
        keys: ci.accounts.map((accountIdx) => parsedAccounts[accountIdx]),
      });
    } else {
      const ci = compiledInstruction as MessageCompiledInstruction;

      return new TransactionInstruction({
        data: Buffer.from(ci.data),
        programId: parsedAccounts[ci.programIdIndex].pubkey,
        keys: ci.accountKeyIndexes.map((accountIndex) => {
          if (accountIndex >= parsedAccounts.length)
            throw new Error(
              `Trying to resolve account at index ${accountIndex} while parsedAccounts is only ${parsedAccounts.length}. \
              Looks like you're trying to parse versioned transaction, make sure that LoadedAddresses are passed to the \
              parseTransactionAccounts function`,
            );

          return parsedAccounts[accountIndex];
        }),
      });
    }
  }

  public flattenTransactionResponse(
    transaction: VersionedTransactionResponse,
  ): TransactionInstruction[] {
    const result: TransactionInstruction[] = [];
    if (transaction === null || transaction === undefined) return [];
    const txInstructions = transaction.transaction.message.compiledInstructions;
    const accountsMeta = this.parseTransactionAccounts(
      transaction.transaction.message,
      transaction.meta?.loadedAddresses,
    );

    const orderedCII = (transaction?.meta?.innerInstructions || []).sort(
      (a, b) => a.index - b.index,
    );
    const totalCalls =
      (transaction.meta?.innerInstructions || []).reduce(
        (accumulator, cii) => accumulator + cii.instructions.length,
        0,
      ) + txInstructions.length;
    let lastPushedIx = -1;
    let callIndex = -1;
    for (const CII of orderedCII) {
      // push original instructions until we meet CPI
      while (lastPushedIx !== CII.index) {
        lastPushedIx += 1;
        callIndex += 1;
        result.push(
          this.compiledInstructionToInstruction(
            txInstructions[lastPushedIx],
            accountsMeta,
          ),
        );
      }
      for (const CIIEntry of CII.instructions) {
        result.push(
          this.compiledInstructionToInstruction(CIIEntry, accountsMeta),
        );
        callIndex += 1;
      }
    }
    while (callIndex < totalCalls - 1) {
      lastPushedIx += 1;
      callIndex += 1;
      result.push(
        this.compiledInstructionToInstruction(
          txInstructions[lastPushedIx],
          accountsMeta,
        ),
      );
    }
    return result;
  }
}
