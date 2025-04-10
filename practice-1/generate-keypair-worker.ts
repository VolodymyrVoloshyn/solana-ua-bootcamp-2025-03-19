import { parentPort, workerData } from "worker_threads";
import { Keypair } from "@solana/web3.js";

const { publicKeyStart } = workerData;
let count = 0;

while (true) {
    const keypair = Keypair.generate();
    const publicKeyStr = keypair.publicKey.toBase58();
    count++;

    if (count % 10000 === 0) {
        parentPort?.postMessage({ type: "progress", count });
    }

    if (publicKeyStr.startsWith(publicKeyStart)) {
        parentPort?.postMessage({
            type: "found",
            publicKey: publicKeyStr,
            secretKey: Array.from(keypair.secretKey),
            count,
        });
        break;
    }
}
