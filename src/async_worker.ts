import { type WorkerResult } from './util';

class Queue<T> {
  readonly #in: T[] = [];
  readonly #out: T[] = [];

  get length() {
    return this.#in.length + this.#out.length;
  }

  #transfer = () => {
    while (this.#in.length > 0) {
      this.#out.push(this.#in.pop()!);
    }
  };

  enqueue = (item: T) => {
    this.#in.push(item);
    return this.length;
  };

  dequeue = () => {
    if (this.#out.length === 0) {
      this.#transfer();
    }
    return this.#out.pop();
  };

  toArray = () => {
    this.#transfer();
    return this.#out.slice();
  };
}

export const asyncWorkerFactory = <TPost, TRecv>(worker: Worker) => {
  const queue: Queue<WorkerResult<TRecv, unknown>> = new Queue();

  worker.onmessage = (e: MessageEvent<WorkerResult<TRecv, unknown>>) => {
    queue.enqueue(e.data);
  };

  worker.onerror = (e) => {
    throw Error(e.message, { cause: e });
  };

  const postMessage = (
    message: TPost,
    options?: StructuredSerializeOptions
  ) => {
    worker.postMessage(message, options);
  };

  const receive = () => {
    return new Promise<TRecv>((resolve, reject) => {
      const id = setInterval(() => {
        if (queue.length > 0) {
          clearInterval(id);
          const res = queue.dequeue()!;

          if (res.success) {
            resolve(res.value);
          } else {
            reject(res.error);
          }
        }
      }, 10);
    });
  };

  async function* iter(max: number) {
    for (let i = 0; i < max; i++) {
      if (queue.length > 0) {
        const res = queue.dequeue()!;
        if (res.success) {
          yield res.value;
        } else {
          throw res.error;
        }
      } else {
        yield receive();
      }
    }
  }

  return {
    postMessage,
    receive,
    iter,
    worker,
  };
};
