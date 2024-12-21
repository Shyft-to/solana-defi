import { createJupiterApiClient, QuoteGetRequest, QuoteResponse } from "@jup-ag/api";

const jupiterQuoteApi = createJupiterApiClient();

export async function getBuyQuote(token,amount) {
    const quoteResponse = await (
    await fetch(`https://jup.ny.shyft.to/quote?inputMint=So11111111111111111111111111111111111111112&outputMint=${token}&amount=${amount}&dexes=Raydium`
     ,{
     //  method: 'POST',
      headers: {
        'Content-Type': 'application/json',
         'x-api-key' : 'api'
      },
    }
    )
  ).json();
  return quoteResponse;
  }
 export async function getSellQuote(token, amount,dex){
  const quoteResponse = await (
    await fetch(`https://jup.ny.shyft.to/quote?inputMint=${token}&outputMint=So11111111111111111111111111111111111111112&amount=${amount}&dexes=Whirlpool`
     ,{
     //  method: 'POST',
      headers: {
        'Content-Type': 'application/json',
         'x-api-key' : 'api'
      },
    }
    )
  ).json();
  return quoteResponse;
 }   
 // getBuyQuote('5LafQUrVco6o7KMz42eqVEJ9LW31StPyGjeeu5sKoMtA',100000);
      