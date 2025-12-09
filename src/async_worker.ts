import { type Result, resolvers } from './util';

export const asyncWorkerFactory = <TPost, TRtrn>(worker: Worker) => {
  const receivers: Map<number, ReturnType<typeof resolvers<TRtrn>>> = new Map();

  let counter = 0;

  worker.onmessage = (e: MessageEvent<[number, Result<TRtrn, unknown>]>) => {
    const [id, ans] = e.data;
    const receiver = receivers.get(id);
    if (!receiver) {
      throw Error('Receiver Not Found');
    }
    if (ans.success) {
      receiver.resolve(ans.value);
    } else {
      const err = ans.error;

      receiver.reject(err);
    }
  };

  worker.onerror = (e) => {
    console.error(e.message);
    throw Error(e.message, e.error);
  };

  const postMessage = (msg: TPost) => {
    const id = counter++;
    worker.postMessage([id, msg]);
    const rslv = resolvers<TRtrn>();
    receivers.set(id, rslv);
    
  };

  const receive = async () => {
    const iterRes = receivers.entries().next();
    if (iterRes.done) return;
    const [id, { promise }] = iterRes.value;
    return promise.finally(() => receivers.delete(id));
  };

  return {
    postMessage,
    receive,
  };
};
