import { parseError } from './util';
import type { BFOptions } from './wasm/wasm_part';
import type { WorkerMsg } from './util';

let worker: Worker | undefined;

const getWorker = () => {
  if (!worker) {
    worker = new Worker(new URL('./wasm_wrapper.ts', import.meta.url), {
      type: 'module',
    });
  }
  return worker;
};

export const exec = (code: string, options: BFOptions = {}) => {
  return new Promise<WorkerMsg>(async (resolve) => {
    const worker = getWorker();
    const messageHandle = (e: MessageEvent<WorkerMsg>) => {
      const res = e.data;
      worker.onerror = null;
      worker.onmessage = null;
      resolve(res);
    };

    const errorHandle = (e: ErrorEvent) => {
      worker.onerror = null;
      worker.onmessage = null;

      resolve({
        success: false,
        error: parseError(e.error),
      });
    };
    worker.onmessage = messageHandle;
    worker.onerror = errorHandle;
    worker.postMessage({ code, options });
  });
};
