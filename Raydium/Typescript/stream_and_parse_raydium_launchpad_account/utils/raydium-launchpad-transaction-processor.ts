import { base64ToBase58, GLOBAL_CONFIG_DISC, PLATFORM_CONFIG_DISC, VESTING_RECORD_DISC } from "./raydium-launchpad-pool-utils";
import {
  POOLSTATE_DISC,
  decodeGlobalConfig,
  decodePlatformConfig,
  decodePoolState,
  decodeVestingRecord,
} from "./raydium-launchpad-pool-utils";

export async function decodeRaydiumLaunchpadTxnData(data) {
  try {
    if (!data || !data.account || !data.account.account) return;

    const dataTx = data.account.account;

    const signature = dataTx.txnSignature ? base64ToBase58(dataTx.txnSignature) : null;
    const pubKey = dataTx.pubkey ? base64ToBase58(dataTx.pubkey) : null;
    const owner = dataTx.owner ? base64ToBase58(dataTx.owner) : null;

    let poolstate = null;
    const discriminator = Buffer.from(dataTx.data.slice(0, 8)); // Extract the first 8 bytes
    
    if (discriminator.equals(GLOBAL_CONFIG_DISC)) {
      poolstate = decodeGlobalConfig(dataTx.data);
    } else if (discriminator.equals(PLATFORM_CONFIG_DISC)) {
      poolstate = decodePlatformConfig(dataTx.data);
    } else if (discriminator.equals(POOLSTATE_DISC)) {
      poolstate = decodePoolState(dataTx.data);
    } else if (discriminator.equals(VESTING_RECORD_DISC)) {
      poolstate = decodeVestingRecord(dataTx.data);
    } else {
      console.warn("Unknown discriminator:", discriminator);
    }

    return {
      signature,
      pubKey,
      owner,
      poolstate,
    };
  } catch (error) {
    console.error("Error decoding transaction data:", error);
    throw error;
  }
}
