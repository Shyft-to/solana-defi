import {
  ConfirmedTransactionMeta,
  Message,
  MessageV0,
  PublicKey,
  VersionedMessage,
  VersionedTransactionResponse,
} from "@solana/web3.js";
import { utils } from "@coral-xyz/anchor";

function toPublicKey(address: any): PublicKey {
  if (address instanceof PublicKey) return address;
  if (Buffer.isBuffer(address) || address instanceof Uint8Array) return new PublicKey(address);
  if (typeof address === "string") {
    const buf = Buffer.from(address, "base64");
    if (buf.length === 32) return new PublicKey(buf);
    return new PublicKey(address); 
  }
  throw new Error();
}

function describeValue(v: any): string {
  if (v === undefined) return;
  if (v === null) return ;
  if (Buffer.isBuffer(v)) return `Buffer(${v.length}) hex=${v.slice(0, 4).toString("hex")}...`;
  if (v instanceof Uint8Array) return `Uint8Array(${v.length})`;
  if (v instanceof PublicKey) return `PublicKey(${v.toBase58()})`;
  if (Array.isArray(v)) return `Array(${v.length})${v.length ? " → [0]: " + describeValue(v[0]) : ""}`;
  if (typeof v === "object") return `object { ${Object.keys(v).join(", ")} }`;
  return `${typeof v} = ${String(v).slice(0, 100)}`;
}


let _diagDone = false;

function diagnose(data: any): void {
  if (_diagDone) return;
  _diagDone = true;

  const p = (label: string, v: any) => console.log(`  ${label}: ${describeValue(v)}`);

  p("data.slot", data.slot);

  const rawTx = data["transaction"];
  if (rawTx == null) { console.log("    !! NULL / UNDEFINED — wrong key?"); }
  else {

    const tx = rawTx.transaction;
    if (tx == null) { console.log("    !! NULL / UNDEFINED"); }
    else {
      const msg = tx.message;
      if (msg == null) { console.log("    !! NULL / UNDEFINED"); }
      else {
        p("versioned", msg.versioned);
        p("accountKeys", msg.accountKeys);
        p("addressTableLookups", msg.addressTableLookups);
      }
    }

    const meta = rawTx.meta;
    if (meta == null) { console.log("    !! NULL / UNDEFINED"); }
    else {
      for (const [k, v] of Object.entries(meta)) {
        console.log(`    ${k}: ${describeValue(v)}`);
      }
      p("meta.loadedWritableAddresses", meta.loadedWritableAddresses);
      p("meta.loadedReadonlyAddresses", meta.loadedReadonlyAddresses);
    }
  }

}

export class TransactionFormatter {
  public formTransactionFromJson(
    data: any,
    time: number
  ): VersionedTransactionResponse {
    diagnose(data); // ← runs automatically, only prints once

    const rawTx = data["transaction"];
    const slot = data.slot;
    const version = rawTx.transaction.message.versioned ? 0 : "legacy";

    let meta = null;
    if (rawTx.meta) {
      meta = this.formMeta(rawTx.meta);
    }

    const signatures = rawTx.transaction.signatures.map((s: Buffer) =>
      utils.bytes.bs58.encode(s)
    );

    const message = this.formTxnMessage(rawTx.transaction.message);

    return { slot, version, blockTime: time, meta, transaction: { signatures, message } };
  }

  private formTxnMessage(message: any): VersionedMessage {
    if (!message.versioned) {
      return new Message({
        header: {
          numRequiredSignatures: message.header.numRequiredSignatures,
          numReadonlySignedAccounts: message.header.numReadonlySignedAccounts,
          numReadonlyUnsignedAccounts: message.header.numReadonlyUnsignedAccounts,
        },
        recentBlockhash: utils.bytes.bs58.encode(
          Buffer.from(message.recentBlockhash, "base64")
        ),
        accountKeys: message.accountKeys?.map((d: string) => Buffer.from(d, "base64")),
        instructions: message.instructions.map(({ data, programIdIndex, accounts }: any) => ({
          programIdIndex,
          accounts: Array.from(accounts),
          data: utils.bytes.bs58.encode(Buffer.from(data || "", "base64")),
        })),
      });
    } else {
      return new MessageV0({
        header: {
          numRequiredSignatures: message.header.numRequiredSignatures,
          numReadonlySignedAccounts: message.header.numReadonlySignedAccounts,
          numReadonlyUnsignedAccounts: message.header.numReadonlyUnsignedAccounts,
        },
        recentBlockhash: utils.bytes.bs58.encode(
          Buffer.from(message.recentBlockhash, "base64")
        ),
        staticAccountKeys: message.accountKeys.map(
          (k: string) => new PublicKey(Buffer.from(k, "base64"))
        ),
        compiledInstructions: message.instructions.map(
          ({ programIdIndex, accounts, data }: any) => ({
            programIdIndex,
            accountKeyIndexes: Array.from(accounts),
            data: Uint8Array.from(Buffer.from(data || "", "base64")),
          })
        ),
        addressTableLookups:
          message.addressTableLookups?.map(
            ({ accountKey, writableIndexes, readonlyIndexes }: any) => ({
              writableIndexes: writableIndexes || [],
              readonlyIndexes: readonlyIndexes || [],
              accountKey: new PublicKey(Buffer.from(accountKey, "base64")),
            })
          ) || [],
      });
    }
  }

  private formMeta(meta: any): ConfirmedTransactionMeta {
    const writable = meta.loadedWritableAddresses;
    const readonly = meta.loadedReadonlyAddresses;

    const loadedAddresses =
      writable?.length || readonly?.length
        ? {
            writable: (writable || []).map(toPublicKey),
            readonly: (readonly || []).map(toPublicKey),
          }
        : undefined;

    return {
      err: meta.errorInfo ? { err: meta.errorInfo } : null,
      fee: meta.fee,
      preBalances: meta.preBalances,
      postBalances: meta.postBalances,
      preTokenBalances: meta.preTokenBalances || [],
      postTokenBalances: meta.postTokenBalances || [],
      logMessages: meta.logMessages || [],
      loadedAddresses,
      innerInstructions:
        meta.innerInstructions?.map((i: { index: number; instructions: any }) => ({
          index: i.index || 0,
          instructions: i.instructions.map((instruction: any) => ({
            programIdIndex: instruction.programIdIndex,
            accounts: Array.from(instruction.accounts),
            data: utils.bytes.bs58.encode(
              Buffer.from(instruction.data || "", "base64")
            ),
          })),
        })) || [],
    };
  }
}