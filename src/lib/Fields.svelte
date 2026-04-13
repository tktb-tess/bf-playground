<script lang="ts">
  import type { BFRuntimeError, Result } from '$lib/util';
  import { exec } from '$lib/func';

  let code = $state(
    '++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.',
  );

  let input = $state('');

  const handleClick = () => {
    resultA = exec(code, { input });
  };

  let resultA: Promise<Result<string, BFRuntimeError>> = $state(
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
  <button type="button" class="self-center" onclick={handleClick}>実行</button>
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

<style lang="postcss">
  @reference '../app.css';
  @layer components {
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
