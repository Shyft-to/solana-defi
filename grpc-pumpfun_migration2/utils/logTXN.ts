export function searchForInitialize2(transaction) {
    const logMessages = transaction.meta?.logMessages || [];
     if (logMessages.some(log => log.includes('initialize2'))) {
      return transaction;
     }
}
