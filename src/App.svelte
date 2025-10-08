<script lang="ts">
  import { exec as exec_ } from '@tktb-tess/brainf_ck-interpreter';
  import { Result } from 'neverthrow';
  const title = 'BF Playground';
  let code = $state(
    '++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.'
  );
  const exec = Result.fromThrowable(exec_, (e) => {
    if (e instanceof Error) {
      return e;
    } else {
      return Error(`${e}`, { cause: e });
    }
  });

  const handleClick = () => {
    result = exec(code);
  };

  let result = $state(exec(''));
</script>

<header>
  <h1>{title}</h1>
</header>
<main>
  <p><a href="../">戻る</a></p>
  <p>Brainf*ck の簡易的な実行環境です。</p>

  <div class="flex flex-col gap-2">
    <label for="input" class="text-center">コード</label>
    <textarea id="input" bind:value={code}></textarea>
    <button type="button" class="self-center" onclick={handleClick}>実行</button>
    <label for="result" class="text-center">実行結果</label>
    <textarea
      id="result"
      value={result.isOk()
        ? result.value
        : `${result.error.name}: ${result.error.message}`}
      readonly
      class={result.isErr() ? 'text-red-500' : ''}
    ></textarea>
  </div>
</main>
