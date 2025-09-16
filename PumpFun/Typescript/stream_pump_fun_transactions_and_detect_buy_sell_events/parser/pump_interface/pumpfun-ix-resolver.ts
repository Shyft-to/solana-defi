import { PUMP_FUN_PROGRAM_ID } from "../../utils/type";
import { PumpFunInstructionParser } from "./src/instructions";

export function parseAndFilterPumpFunInstructions(parsedInnerInstructions: any[]) {
    const filterPumpFunInstructions = (instructions: any[]) => 
        instructions.filter((instruction) => 
            instruction.name === "unknown" && 
            instruction.programId.equals(PUMP_FUN_PROGRAM_ID) &&
            instruction.accounts && 
            (instruction.accounts.length > 13 && instruction.accounts.length < 17) 
        );

    const filteredPumpFunInstructions: any[] = filterPumpFunInstructions(parsedInnerInstructions);
    
    if (filteredPumpFunInstructions.length === 0) {
        return parsedInnerInstructions;     
    } 
    
    const firstPumpFunInstruction = filteredPumpFunInstructions.find(
        (instruction: any) => instruction.accounts && instruction.accounts.length > 0
    );
    
    if (!firstPumpFunInstruction) {
        console.log("No valid Pump.fun instruction found");
        return parsedInnerInstructions;
    }

    const accountsWithStringPubkeys = firstPumpFunInstruction.accounts.map((account: any) => ({
        isSigner: account.isSigner,
        isWritable: account.isWritable,
        pubkey: account.pubkey.toString() 
    }));
    
    const instructionArgs = firstPumpFunInstruction.args.unknown;   
    const parsedInstruction = PumpFunInstructionParser.parse(accountsWithStringPubkeys, instructionArgs); 
    
    if (parsedInstruction) {
        return Array.isArray(parsedInstruction) ? parsedInstruction : [parsedInstruction];
    } else {
        return parsedInnerInstructions;
    }
}