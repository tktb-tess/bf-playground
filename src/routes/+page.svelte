<script lang="ts">
  import '../app.css';
  import type { WorkerMsg } from '$lib/util';
  import { exec } from '$lib/func';

  const title = 'BF Playground';

  let code = $state(
    '++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.',
  );

  let input = $state('');

  const handleClick = () => {
    resultA = exec(code, { input });
  };

  let resultA: Promise<WorkerMsg> = $state(
    Promise.resolve({ success: true, value: '' }),
  );

  $effect(() => {
    resultA
      .then((e) => {
        if (!e.success) {
          console.error(e.error.message);
        }
      })
      .catch((e) => console.error(e));
  });
</script>

<div class="_cont_">
  <header>
    <h1 id="title">{title}</h1>
  </header>
  <main>
    <p class="back-btn"><a href="https://tktb-tess.github.io">戻る</a></p>
    <p>Brainf*ck の簡易的な実行環境です。</p>

    <div class="bf-playground">
      <div class="__inputs">
        <section class="__input-section">
          <label for="code">コード</label>
          <textarea id="code" bind:value={code}></textarea>
        </section>
        <section class="__input-section">
          <label for="input">入力</label>
          <textarea id="input" bind:value={input}></textarea>
        </section>
      </div>
      <button type="button" class="self-center" onclick={handleClick}>
        実行
      </button>
      <div class="__input-section">
        <label for="result" class="text-center">実行結果</label>
        {#await resultA}
          <textarea id="result" readonly>Executing...</textarea>
        {:then result}
          <textarea
            id="result"
            value={result.success
              ? result.value
              : `${result.error.name}: ${result.error.message}`}
            readonly
            class={!result.success ? 'text-caution' : ''}
          ></textarea>
        {:catch}
          <textarea id="result" readonly>Unexpected error</textarea>
        {/await}
      </div>
    </div>
  </main>
  <footer></footer>
</div>

<style lang="postcss">
  @reference '../app.css';
  @layer components {
    ._cont_ {
      @apply max-xl:flow-root xl:grid;
      --side: minmax(min(16rem, 50%), 1fr);
      --main: minmax(0, 72rem);

      grid-template-areas:
        'h h h'
        'l m r'
        'f f f';

      grid-template-columns:
        var(--side)
        var(--main)
        var(--side);

      grid-template-rows: auto 1fr auto;
    }
    header,
    main,
    footer {
      @apply flow-root;
    }

    main {
      @apply px-4 pbe-36 bg-main min-block-lvh;
      grid-area: m;
    }

    #title {
      @apply max-md:text-4xl md:text-5xl font-extralight text-balance my-10 text-center;
    }

    header {
      grid-area: h;
    }

    footer {
      grid-area: f;
    }

    .back-btn {
      @apply mbs-paragraph;
    }

    .bf-playground {
      @apply flex flex-col gap-4 mbs-paragraph;
    }

    .__inputs {
      @apply grid gap-2 cols-auto-fit-90 *:min-inline-0;
    }

    .__input-section {
      @apply flex flex-col gap-1;

      > :where(label) {
        @apply text-center;
      }

      > :where(textarea#code) {
        @apply break-all;
      }
    }
  }
</style>
