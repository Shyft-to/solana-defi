
import { getInitWhirlpoolConfigsTx, OrcaWhirlpoolClient, sqrtPriceX64ToPrice } from "@orca-so/whirlpool-sdk";
import { decodeTransact } from "./decodeTransaction";
import { OrcaPool } from "@orca-so/whirlpool-sdk/dist/pool/orca-pool";
import { AccountName, WHIRLPOOL_CODER } from "@orca-so/whirlpools-sdk";

/**
 * Deserializes and trims blockchain data to extract relevant information.
 * @param {Object} data - The data object containing blockchain account information.
 * @returns {Object} - An object containing the deserialized signature, public key, owner, and pool state.
 */
export function tOutPut(data) {
    // Ensure data is defined and contains the necessary properties
    if (!data || !data.account || !data.account.account) {
    }

    const dataTx = data?.account?.account;

    // Safely decode each piece of transaction data
    const signature = dataTx.txnSignature ? decodeTransact(dataTx.txnSignature) : null;
    const pubKey = dataTx.pubkey ? decodeTransact(dataTx.pubkey) : null;
    const owner = dataTx.owner ? decodeTransact(dataTx.owner) : null;
    
    let poolstate = null;

    try {
        poolstate = WHIRLPOOL_CODER.decode(AccountName.Whirlpool,dataTx.data);
    } catch (error) {
    }

    return {
        signature,
        pubKey,
        owner,
        poolstate
    };
}
