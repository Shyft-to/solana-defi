async function getQoute(){
  const quoteResponse = await (
  await fetch(`https://jup.ny.shyft.to/quote?inputMint=So11111111111111111111111111111111111111112&outputMint=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v&amount=100000000`
   ,{
   //  method: 'POST',
    headers: {
      'Content-Type': 'application/json',
       'x-api-key' : 'api'
    },
  }
  )
).json();
console.log(quoteResponse)
}
getQoute();