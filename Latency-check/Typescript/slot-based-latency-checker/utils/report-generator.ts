//10 mins data
/*
  2 mins window
  min latency
  max latency
  average latency
  total txns scanned 
  number in <300ms
  number in <800 ms
  number in < 1000ms
  number in > 1000ms
*/
export class ReportGenerator {
    minLatency: number;
    maxLatency: number;
    avgLatency: number;
    totalLatency: number;
    totalTxns: number;
    lessThan400: number;
    lessThan800: number;
    lessThan1000: number;
    lessThan1200: number;
    lessThan1500: number;
    lessThan1800: number;
    lessThan2000: number;
    moreThan2000: number;
    constructor() {
        this.minLatency = 3000;
        this.maxLatency = 0;
        this.avgLatency = 0;
        this.totalLatency = 0;
        this.totalTxns = 0;
        this.lessThan400 = 0;
        this.lessThan800 = 0;
        this.lessThan1000 = 0;
        this.lessThan1200 = 0;
        this.lessThan1500 = 0;
        this.lessThan1800 = 0;
        this.lessThan2000 = 0;
        this.moreThan2000 = 0;
    }

    private calculateLatency(transactionTime: number, timeReceived: number) :number {
        const observedDifference = timeReceived - transactionTime;
        const latency = observedDifference > 0 ? observedDifference : 0;
        return latency;
    }
    public collectData(transactionTime: number, timeReceived: number) {
        const latency = this.calculateLatency(transactionTime, timeReceived);
        if(latency < this.minLatency) {
            this.minLatency = latency;
        }
        if(latency > this.maxLatency) {
            this.maxLatency = latency;
        }
        this.totalLatency += latency;
        this.totalTxns++;
        this.avgLatency = this.totalLatency / this.totalTxns;
        if(latency < 400) {
            this.lessThan400++;
        }
        if(latency >= 400 && latency < 800) {
            this.lessThan800++;
        }
        if(latency >= 800 && latency < 1000) {
            this.lessThan1000++;
        }
        if(latency >= 1000 && latency < 1200) {
            this.lessThan1200++;
        }
        if(latency >= 1200 && latency < 1500) {
            this.lessThan1500++;
        }
        if(latency >= 1500 && latency < 1800) {
            this.lessThan1800++;
        }
        if(latency >= 1800 && latency < 2000) {
            this.lessThan2000++;
        }
        if(latency >= 2000) {
            this.moreThan2000++;
        }
    }
     public generateReport() {
        //this.avgLatency = this.totalLatency / this.totalTxns;
        console.log("**********************************************\n");
        console.log(`*  Min Latency: ${this.minLatency}`);
        console.log(`*  Max Latency: ${this.maxLatency}`);
        console.log(`*  Average Latency: ${this.avgLatency}`);
        console.log(`*  Total Txns: ${this.totalTxns}`);
        console.log(`*  0-399ms: ${this.lessThan400} | ${((this.lessThan400 / this.totalTxns) * 100).toFixed(2)} %`);
        console.log(`*  400-799ms: ${this.lessThan800} | ${((this.lessThan800 / this.totalTxns) * 100).toFixed(2)} %`);
        console.log(`*  800-999ms: ${this.lessThan1000} | ${((this.lessThan1000 / this.totalTxns) * 100).toFixed(2)} %`);
        console.log(`*  1000-1199ms: ${this.lessThan1200} | ${((this.lessThan1200 / this.totalTxns) * 100).toFixed(2)} %`);
        console.log(`*  1200-1499ms: ${this.lessThan1500} | ${((this.lessThan1500 / this.totalTxns) * 100).toFixed(2)} %`);
        console.log(`*  1500-1799ms: ${this.lessThan1800} | ${((this.lessThan1800 / this.totalTxns) * 100).toFixed(2)} %`);
        console.log(`*  1800-2000ms: ${this.lessThan2000} | ${((this.lessThan2000 / this.totalTxns) * 100).toFixed(2)} %`);
        console.log(`*  2000ms+: ${this.moreThan2000} | ${((this.moreThan2000 / this.totalTxns) * 100).toFixed(2)} %`);
        console.log("\n**********************************************");
    }

}