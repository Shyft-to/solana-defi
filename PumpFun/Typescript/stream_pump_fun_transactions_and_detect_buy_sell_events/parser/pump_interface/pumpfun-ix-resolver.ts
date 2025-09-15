import { PUMP_FUN_PROGRAM_ID } from "../../utils/type";
import { mapAccounts } from "./src/instructions";

export function filtered_parsed_txn(parsedInnerIxs: any[]) {
      const filteredParsedInnerIxs = (ixs: any[]) => 
        ixs.filter((ix) => 
            ix.name === "unknown" && 
            ix.programId.equals(PUMP_FUN_PROGRAM_ID) &&
            ix.accounts && 
            (ix.accounts.length > 13 && ix.accounts.length < 17) 
        );
    const filteredPumpfunInnerIx: any[] = filteredParsedInnerIxs(parsedInnerIxs);
    
    if (filteredPumpfunInnerIx.length === 0){
     return parsedInnerIxs;     
    } 
    
    const firstIx = filteredPumpfunInnerIx.find((ix: any) => ix.accounts && ix.accounts.length > 0);
    
    if (!firstIx) {
        console.log("No valid Pump.fun instruction found");
        return parsedInnerIxs;
    }

    
    const accountsWithStringPubkeys = firstIx.accounts.map((acc: any) => ({
        isSigner: acc.isSigner,
        isWritable: acc.isWritable,
        pubkey: acc.pubkey.toString() 
    }));
    const args = firstIx.args.unknown;   
    const manual_parsing = mapAccounts(accountsWithStringPubkeys, args);
    
    if (manual_parsing) {
        
        
    return Array.isArray(manual_parsing) ? manual_parsing : [manual_parsing];
    } else {
        return parsedInnerIxs;
    }
}