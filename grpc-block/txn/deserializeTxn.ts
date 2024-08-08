import base58 from "bs58";

export class SignatureFormater{
    public SignatureTransactionFromJson(data){
       const signature = base58.encode(Buffer.from(data.signature,'base64'))
       const transaction = data.transaction;
       const signatures = base58.encode(Buffer.from(transaction.signatures[0]))
       const meta = data.meta;

       return {signature,transaction,signatures,meta}
    }
}