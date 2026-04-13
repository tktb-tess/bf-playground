import init, { exec_inner } from './wasm/wasm_part';
import {
  postSuccess,
  postFailed,
  type WorkerMessage,
} from '@tktb-tess/util-fns';
import type { BFMessage } from './worker_types';

let initialized = false;

globalThis.onmessage = async (e: MessageEvent<WorkerMessage<BFMessage>>) => {
  const { id, value } = e.data;
  const { code, options } = value;
  try {
    if (!initialized) {
      await init();
      initialized = true;
    }
    const value = exec_inner(code, options);

    postSuccess(value, id);
  } catch (e) {
    postFailed(e, id);
  }
};
