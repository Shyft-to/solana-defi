export function searchForInitialize2(transaction) {
    const logMessages = transaction.meta?.logMessages || [];
     if (logMessages.some(log => log.includes('InitializeMint2'))) {
      return transaction;
     }
}
