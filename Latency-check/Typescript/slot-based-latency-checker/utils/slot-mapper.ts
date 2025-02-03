import { ReportGenerator } from "./report-generator";

type TxnData = {
  signature: string;
  createdAt: number;
  blockTime: number;
  receivedTime: number;
};
type GroupedTxnData = {
  [slot: string]: TxnData[];
};

let reportGen = new ReportGenerator();

export class SlotMapper {
  private groupedTxnData: GroupedTxnData = {};
  private accumLatency = 0;
  private count = 0;

  addDatafromTransaction(
    slot: string,
    signature: string,
    createdAt: number,
    blockTime: number,
    receivedTime: number
  ) {
    if (!this.groupedTxnData[slot]) {
      this.groupedTxnData[slot] = [];
    }
    this.groupedTxnData[slot].push({
      signature,
      createdAt,
      blockTime,
      receivedTime,
    });
  }

  addDatafromBlockMeta(slot: string, blockTime: number) {
    if (!this.groupedTxnData[slot]) return;

    for (let i = 0; i < this.groupedTxnData[slot].length; i++) {
      //add check to remove older blocks

      this.groupedTxnData[slot][i].blockTime = blockTime;
      console.log("Signature: ", this.groupedTxnData[slot][i].signature);
      console.log(
        "BlockTime: ",
        blockTime,
        "ReceivedTime: ",
        this.groupedTxnData[slot][i].receivedTime,
        "created At: ",
        this.groupedTxnData[slot][i].createdAt
      );
      console.log(
        "Observed Latency: ",
        this.groupedTxnData[slot][i].receivedTime - blockTime
      );
      console.log(
        "Latency based on created at: ",
        this.groupedTxnData[slot][i].receivedTime -
          this.groupedTxnData[slot][i].createdAt
      );

      this.accumLatency +=
        this.groupedTxnData[slot][i].receivedTime - blockTime;
      this.count++;

      console.log("\nAverage Latency: ", this.accumLatency / this.count);

      reportGen.collectData(
        blockTime,
        this.groupedTxnData[slot][i].receivedTime
      );
    }

    // Remove slots older than 20
    const slotsToRemove: string[] = [];
    for (const storedSlot in this.groupedTxnData) {
      if (Number(storedSlot) < Number(slot) - 20) {
        slotsToRemove.push(storedSlot);
      }
    }

    slotsToRemove.forEach((slotToRemove) => {
      delete this.groupedTxnData[slotToRemove];
    });
  }
  displayGroupedTxnData() {
    reportGen.generateReport();
  }
}
