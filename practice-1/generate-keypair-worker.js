"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var worker_threads_1 = require("worker_threads");
var web3_js_1 = require("@solana/web3.js");
var publicKeyStart = worker_threads_1.workerData.publicKeyStart;
var count = 0;
while (true) {
    var keypair = web3_js_1.Keypair.generate();
    var publicKeyStr = keypair.publicKey.toBase58();
    count++;
    if (count % 10000 === 0) {
        worker_threads_1.parentPort === null || worker_threads_1.parentPort === void 0 ? void 0 : worker_threads_1.parentPort.postMessage({ type: "progress", count: count });
    }
    if (publicKeyStr.startsWith(publicKeyStart)) {
        worker_threads_1.parentPort === null || worker_threads_1.parentPort === void 0 ? void 0 : worker_threads_1.parentPort.postMessage({
            type: "found",
            publicKey: publicKeyStr,
            secretKey: Array.from(keypair.secretKey),
            count: count,
        });
        break;
    }
}
