import { Worker } from 'worker_threads';
import * as path from 'path';

const worker = new Worker(path.resolve(__dirname, './factorialWorker.js'), {
  workerData: {
    value: 15
  }
});

worker.on('message', (result) => {
  console.log(result);
});

worker.on('error', (err) => {
  console.error('Worker error:', err);
});

worker.on('exit', (code) => {
  if (code !== 0) {
    console.error(`Worker stopped with exit code ${code}`);
  }
});