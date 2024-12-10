import { createJupiterApiClient, QuoteGetRequest, QuoteResponse } from "@jup-ag/api";

const jupiterQuoteApi = createJupiterApiClient();

async function getQuote() {
    const params: QuoteGetRequest = {
        inputMint:  "5LafQUrVco6o7KMz42eqVEJ9LW31StPyGjeeu5sKoMtA",
        outputMint: "So11111111111111111111111111111111111111112", // $WIF5LafQUrVco6o7KMz42eqVEJ9LW31StPyGjeeu5sKoMtA
        amount: 38923052, // 0.0001 SOL
        autoSlippage: true,
        dexes:['Raydium'],
        autoSlippageCollisionUsdValue: 1_000,
        maxAutoSlippageBps: 1000, // 10%
        minimizeSlippage: true,
        onlyDirectRoutes: true,
        asLegacyTransaction: false,
      };
          // get quote
          const quote = await jupiterQuoteApi.quoteGet(params);
         
          if (!quote) {
            throw new Error("unable to quote");
          }
          console.log(quote.routePlan)
          return quote;
    }
   getQuote();
      