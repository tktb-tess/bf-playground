import { AsyncWorker } from '@tktb-tess/util-fns/async_worker';
import type { BFMessage } from './worker_types';

let worker: AsyncWorker<BFMessage, string> | undefined;

export const getWorker = () => {
  if (!worker) {
    const w = new Worker(new URL('./wasm_wrapper.ts', import.meta.url), {
      type: 'module',
    });
    worker = new AsyncWorker(w);
  }
  return worker;
};
