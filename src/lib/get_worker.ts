import { AsyncWorker } from '@tktb-tess/util-fns';
import Worker from './wasm_wrapper?worker';
import type { BFMessage } from './worker_types';

let worker: AsyncWorker<BFMessage, string> | undefined;

export const getWorker = () => {
  if (!worker) {
    const w = new Worker({ name: 'wasm_wrapper' });
    worker = new AsyncWorker(w);
  }
  return worker;
};