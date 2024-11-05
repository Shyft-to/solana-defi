import { ORCA_WHIRLPOOLS_CONFIG, ORCA_WHIRLPOOLS_CONFIG_EXTENSION, ORCA_WHIRLPOOL_PROGRAM_ID, ParsableWhirlpool, ParsableWhirlpoolsConfigExtension, WHIRLPOOL_CODER, WHIRLPOOL_IDL, WhirlpoolAccountFetcher, WhirlpoolContext, WhirlpoolIx, WhirlpoolRewardInfoData, WhirlpoolRouterBuilder } from "@orca-so/whirlpools-sdk";
import { BN } from "@project-serum/anchor";
import { LIQUIDITY_STATE_LAYOUT_V4, MAINNET_PROGRAM_ID, minExpirationTime, TOKEN_PROGRAM_ID } from "@raydium-io/raydium-sdk";
import { struct, u32, u8 } from "@solana/buffer-layout";
import { bool, publicKey, u64 } from "@solana/buffer-layout-utils";
import { Connection, GetProgramAccountsFilter, PublicKey } from "@solana/web3.js";

const api = 'YMyDOr87OBzT6TWr'
// 'pCdmc-vpYN9w2etM'
const connection = new Connection(`https://rpc.shyft.to?api_key=${api}`, 'confirmed');
const ADDRESS = new PublicKey('675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8');
export interface RawMint {
  whirlpoolsConfig: PublicKey;
  whirlpoolBump: number[];
  feeRate: number;
  protocolFeeRate: number;
  liquidity: BN;
  sqrtPrice: BN;
  tickCurrentIndex: number;
  protocolFeeOwedA: BN;
  protocolFeeOwedB: BN;
  tokenMintA: PublicKey;
  tokenVaultA: PublicKey;
  feeGrowthGlobalA: BN;
  tokenMintB: PublicKey;
  tokenVaultB: PublicKey;
  feeGrowthGlobalB: BN;
  rewardLastUpdatedTimestamp: BN;
  rewardInfos: WhirlpoolRewardInfoData[];
  tickSpacing: number;
}
export const MintLayout = struct<RawMint>([
  publicKey('whirlpoolsConfig'),
  u64('whirlpoolBump'),
  u64('feeRate'),
  u64('protocolFeeRate'),
  u64('liquidity'),
  BN('sqrtPrice'),
  BN('tickCurrentIndex'),
  BN('protocolFeeOwedA'),
  BN('protocolFeeOwedB'),
  publicKey('tokenMintA'),
  publicKey('tokenVaultA'),
  BN('feeGrowthGlobalA'),
  publicKey('tokenMintB'),
  publicKey('tokenVaultB'),
  BN('feeGrowthGlobalB'),
  BN('rewardLastUpdatedTimestamp'),
  u64('tickSpacing')
]);
// DEX program addresses
const dexes = [
    { name: 'Raydium', address: '675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8' },
    { name: 'Orca', address: 'whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc' },
    { name: 'Meteora', address: 'LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo' }
];
const raydium = new PublicKey('5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1');
// Token you want to fetch pools for
const tokenA = new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v');
const filters: GetProgramAccountsFilter[] = [
    {
        dataSize: 165
    },
    {
        memcmp: {
            offset:32,
            bytes:tokenA.toString()
        }
    }
]

async function fetchRaydiumPools() {
  try {
    const accounts = await connection.getProgramAccounts(
        MAINNET_PROGRAM_ID.AmmV4,  
        {
          'commitment' : 'confirmed',  
          filters: [  
            { dataSize: LIQUIDITY_STATE_LAYOUT_V4.span }, 
            {
              memcmp: { 
                offset: LIQUIDITY_STATE_LAYOUT_V4.offsetOf("baseMint"),
                bytes: tokenA.toBase58(),  
              },
            }
          ],
        }
    );
    
   
    let rawData =  accounts.map(({ pubkey, account }) => ({
        id: pubkey.toString(),  
        data: LIQUIDITY_STATE_LAYOUT_V4.decode(account.data), 
    }));
    
    // Assuming only one relevant account is found, return the first object
    let obj = rawData[0];
    return obj;
   } catch (error) {
    // Catch any errors during the fetch process and log them
    console.log(`fetchMarketAccounts`, error);
  } 
}
async function fetchOrcaPools() {
  try {
    console.log("ADDRESS :" + ORCA_WHIRLPOOL_PROGRAM_ID)
    const accounts = await connection.getProgramAccounts(
      ORCA_WHIRLPOOL_PROGRAM_ID,  
        {
          'commitment' : 'confirmed',  
          filters: [  
            { dataSize: MintLayout.span }, 
            {
              memcmp: { 
                offset: MintLayout.offsetOf("tokenMintA"),
                bytes: tokenA.toBase58(),  
              },
            }
          ],
        }
    );
    
   
    let rawData =  accounts.map(({ pubkey, account }) => ({
        id: pubkey.toString(),  
        data: MintLayout.decode(account.data), 
    }));
    
    // Assuming only one relevant account is found, return the first object
    let obj = rawData[0];
    console.log(obj);
} catch (error) {
    // Catch any errors during the fetch process and log them
    console.log(`fetchMarketAccounts`, error);
} 
}


fetchRaydiumPools();

//main();
