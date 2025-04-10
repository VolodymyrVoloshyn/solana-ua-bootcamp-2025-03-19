import { Worker, isMainThread } from "worker_threads";
import { fileURLToPath } from "url";
import * as os from "os";
import * as path from 'path';

// const __filename = fileURLToPath(import.meta.url);
// const __dirname = path.dirname(__filename);

const publicKeyStart = "VOV";
const numThreads = os.cpus().length - 4;

if (isMainThread) {
    let startTime = new Date();
    let found = false;

    console.log(`🔍 Searching for public key starting with '${publicKeyStart}' using ${numThreads} threads...`);

    const workers: Worker[] = [];

    const workerPath = path.resolve(__dirname, './generate-keypair-worker.js')
    
    for (let i = 0; i < numThreads; i++) {
        const worker = new Worker(workerPath, {
            workerData: { publicKeyStart },
        });

        worker.on("message", (msg) => {
            if (msg.type === "found" && !found) {
                found = true;
                console.log(`\n✅ Public key: ${msg.publicKey}`);
                console.log(`🔐 Secret key:`, msg.secretKey);
                console.log(`🕒 Time: ${((new Date()).getTime() - startTime.getTime()) / 1000} seconds`);

                for (const w of workers) w.terminate();
            } else if (msg.type === "progress") {
                console.log(`Worker ${worker.threadId} count:`, msg.count);
            }
        });

        worker.on("error", (err) => {
            console.error(`Worker error: ${err}`);
        });

        worker.on("exit", (code) => {
            if (code !== 0 && !found) {
                console.log(`Worker stopped with exit code ${code}`);
            }
        });

        workers.push(worker);
    }
}
