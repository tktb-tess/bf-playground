import { type WorkerResult } from './util';

class Queue<T> {
  readonly #in: T[] = [];
  readonly #out: T[] = [];

  get length() {
    return this.#in.length + this.#out.length;
  }

  #transfer() {
    while (this.#in.length > 0) {
      this.#out.push(this.#in.pop()!);
    }
  }

  enqueue(item: T) {
    this.#in.push(item);
    return this.length;
  }

  dequeue() {
    if (this.#out.length === 0) {
      this.#transfer();
    }
    return this.#out.pop();
  }

  toArray() {
    this.#transfer();
    return this.#out.slice();
  }
}

export class AsyncWorker<TPost, TRecv> {
  readonly #queue: Queue<WorkerResult<TRecv, unknown>> = new Queue();
  readonly worker: Worker;

  constructor(worker: Worker) {
    this.worker = worker;
    this.worker.onmessage = (e: MessageEvent<WorkerResult<TRecv, unknown>>) => {
      this.#queue.enqueue(e.data);
    };

    this.worker.onerror = (e) => {
      throw Error(e.message, { cause: e });
    };
  }

  postMessage(message: TPost, options?: StructuredSerializeOptions) {
    this.worker.postMessage(message, options);
  }

  receive() {
    return new Promise<TRecv>((resolve, reject) => {
      const id = setInterval(() => {
        if (this.#queue.length > 0) {
          clearInterval(id);
          const res = this.#queue.dequeue()!;

          if (res.success) {
            resolve(res.value);
          } else {
            reject(res.error);
          }
        }
      }, 10);
    });
  }

  async *iter(max: number) {
    for (let i = 0; i < max; i++) {
      if (this.#queue.length > 0) {
        const res = this.#queue.dequeue()!;
        if (res.success) {
          yield res.value;
        } else {
          throw res.error;
        }
      } else {
        yield this.receive();
      }
    }
  }
}
