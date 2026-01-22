import "dotenv/config";
import * as anchor from "@coral-xyz/anchor";
import { Idl } from "@coral-xyz/anchor";
import { Parser, TransactionStreamer } from "@shyft-to/ladybug-sdk";
import { Connection, PublicKey, clusterApiUrl } from "@solana/web3.js";
import pumpFunIdl from "./idls/pump_0.1.0.json";

// 🔁 Toggle IDL source here
// true  = fetch IDL from chain
// false = use local JSON IDL
const USE_ONCHAIN_IDL = false;

const PROGRAM_ID = new PublicKey(
  "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"
);

/* ------------------------------------------------------------------ */
/*                       IDL FETCHER                                  */
/* ------------------------------------------------------------------ */
async function fetchOnchainIdl(programId: PublicKey): Promise<Idl> {
  const connection = new Connection(clusterApiUrl("mainnet-beta"));
  const wallet = new anchor.Wallet(anchor.web3.Keypair.generate());

  const provider = new anchor.AnchorProvider(connection, wallet, {});
  anchor.setProvider(provider);

  const idl = await anchor.Program.fetchIdl(programId, provider);

  if (!idl) {
    throw new Error("IDL not found on-chain for this program");
  }

  return idl;
}
async function main() {
  const idl: Idl = USE_ONCHAIN_IDL
    ? await fetchOnchainIdl(PROGRAM_ID)
    : (pumpFunIdl as Idl);

  const parser = new Parser();
  parser.addIDL(PROGRAM_ID, idl);

  const streamer = new TransactionStreamer(
    process.env.ENDPOINT!,
    process.env.X_TOKEN!
  );

  streamer.addParser(parser);
  streamer.addAddresses([PROGRAM_ID.toBase58()]);
  streamer.onData(processData);

  streamer.start();
}
async function processData(tx: any) {
  console.log(
    new Date(),
    ":",
    `New transaction https://translator.shyft.to/tx/${tx.transaction.signatures[0]}\n`,
    JSON.stringify(tx, null, 2),
    "\n"
  );
  console.log(
    "--------------------------------------------------------------------------------------------------"
  );
}
main().catch(console.error);