import base58 from "bs58";
import { BorshAccountsCoder } from "@coral-xyz/anchor";
import * as fs from 'fs';
import { bnLayoutFormatter } from "./bn-layout-formatter";

const program_idl = JSON.parse(fs.readFileSync('./idls/raydium_launchpad.json', "utf8"));

const coder = new BorshAccountsCoder(program_idl);

export async function decodeRaydiumLaunchpadTxnData(data) {
  if (!data || !data.account || !data.account.account) return;

const dataTx = data.account.account;

const signature = dataTx.txnSignature ? base64ToBase58(dataTx.txnSignature) : null;
const pubKey = dataTx.pubkey ? base64ToBase58(dataTx.pubkey) : null;
const owner = dataTx.owner ? base64ToBase58(dataTx.owner) : null;

let parsedAccount;
try {
    parsedAccount = coder.decodeAny(dataTx?.data);
    bnLayoutFormatter(parsedAccount)
} catch (error) {
    console.error("Failed to decode pool state:", error);
}

return {
    signature,
    pubKey,
    owner,
    parsedAccount
};
}
 function base64ToBase58(data: string) {
  return base58.encode(Buffer.from(data, 'base64'));
}
