import {  decodeTransact } from "./decodeTransaction";
import { BorshAccountsCoder } from "@coral-xyz/anchor";
import * as fs from 'fs';

const program_idl = JSON.parse(fs.readFileSync('./idls/pump_amm_0.1.0.json', "utf8"));

const coder = new BorshAccountsCoder(program_idl);

export function parsedAccountData(data) {
    if (!data || !data.account || !data.account.account) {
        throw new Error("Invalid data format");
    }

    const dataTx = data.account.account;

    const signature = dataTx.txnSignature ? decodeTransact(dataTx.txnSignature) : null;
    const pubKey = dataTx.pubkey ? decodeTransact(dataTx.pubkey) : null;
    const owner = dataTx.owner ? decodeTransact(dataTx.owner) : null;
    
    let parsedAccount;
    try {
        parsedAccount = coder.decodeAny(dataTx?.data);
    
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
