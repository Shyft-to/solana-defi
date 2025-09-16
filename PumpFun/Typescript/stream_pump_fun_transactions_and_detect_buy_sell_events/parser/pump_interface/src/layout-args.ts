export function decodeInstructionArgs(hexOrBuffer: string | Buffer) {
  const rawBuffer =
    typeof hexOrBuffer === "string"
      ? Buffer.from(hexOrBuffer.replace(/^0x/, ""), "hex")
      : hexOrBuffer;

  const amount = rawBuffer.readBigUInt64LE(8);   
  const maxSolCost = rawBuffer.readBigUInt64LE(16); 

  return {
    amount: amount.toString(),
    maxSolCost: maxSolCost.toString(),
  };
}
