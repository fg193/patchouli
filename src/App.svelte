<script lang="ts">
  import BookCard from "./lib/components/BookCard.svelte";
  import EngineSelect from "./lib/components/EngineSelect.svelte";
  import { tauriSearchApi } from "./lib/api/client";
  import { getProvider } from "./lib/api/providers";
  import type { ApiFailure, BookResult, ProviderId } from "./lib/api/contracts";

  let provider = $state<ProviderId>("douban");
  let resultProvider = $state<ProviderId>("douban");
  let query = $state("");
  let results = $state<BookResult[]>([]);
  let total = $state(0);
  let loading = $state(false);
  let error = $state("");
  let businessCode = $state<number | undefined>(undefined);
  let errorMessage = $state("");
  let hasSearched = $state(false);

  const selectedProvider = $derived(getProvider(provider));
  const resultSource = $derived(getProvider(resultProvider));

  async function search() {
    const normalized = query.trim();
    if (!normalized || loading) return;

    loading = true;
    error = "";
    businessCode = undefined;
    errorMessage = "";
    hasSearched = true;
    resultProvider = provider;
    try {
      const response = await tauriSearchApi.search({
        provider,
        query: normalized,
        page: 1,
        pageSize: 20,
      });
      results = response.items;
      total = response.total;
    } catch (cause) {
      const failure = cause as Partial<ApiFailure> | string;
      const message =
        typeof failure === "string"
          ? failure
          : (failure.message ?? "检索暂时不可用，请稍后再试。");
      businessCode =
        typeof failure === "string" ? undefined : failure.businessCode;
      errorMessage = message;
      error =
        typeof failure === "string" ||
        !failure.code ||
        failure.code === "NETWORK_ERROR"
          ? message
          : `${resultSource.name}：${message}`;
      results = [];
      total = 0;
    } finally {
      loading = false;
    }
  }
</script>

<svelte:head
  ><title
    >{hasSearched
      ? `${query} · ${selectedProvider.shortName}`
      : "Patchouli · 图书检索"}</title
  ></svelte:head
>

<main class:searched={hasSearched}>
  <header>
    <a class="brand" href="/" aria-label="Patchouli 首页">
      <span>Patchouli</span>
    </a>
  </header>

  <section class="hero" aria-labelledby="page-title">
    <div class="eyebrow"><span></span>在千万册书中，找到那一页</div>
    <h1 id="page-title">从一处开始，<em>寻遍群书。</em></h1>
    <p class="intro">
      聚合豆瓣、首都图书馆与国家图书馆馆藏，一次检索，少一点辗转。
    </p>

    <form
      class="search-box"
      onsubmit={(event) => {
        event.preventDefault();
        search();
      }}
    >
      <EngineSelect value={provider} onchange={(id) => (provider = id)} />
      <input
        bind:value={query}
        aria-label="搜索关键词"
        placeholder={selectedProvider.placeholder}
        autocomplete="off"
      />
      <button
        class="search-button"
        type="submit"
        disabled={!query.trim() || loading}
        aria-label="搜索"
      >
        {#if loading}<span class="spinner"></span>{:else}<svg
            viewBox="0 0 24 24"
            aria-hidden="true"
            ><circle cx="11" cy="11" r="6.5" /><path d="m16 16 4 4" /></svg
          >{/if}
        <span>检索</span>
      </button>
    </form>
  </section>

  {#if hasSearched}
    <section class="results" aria-live="polite">
      {#if loading}
        <div class="status">
          <span class="large-spinner"></span>
          <p>少女祈祷中…</p>
        </div>
      {:else if error}
        <div class="status error-state">
          <span>!</span>
          <h2>这一页暂时翻不开</h2>
          {#if businessCode !== undefined}<p>错误码：{businessCode}</p>{/if}
          <p>错误消息：{errorMessage || error}</p>
          <button type="button" onclick={search}>重新检索</button>
        </div>
      {:else if results.length === 0}
        <div class="status">
          <span class="empty-glyph">○</span>
          <h2>没有找到相关书目</h2>
          <p>试试更短的书名、作者名，或切换另一处馆藏。</p>
        </div>
      {:else}
        <div class="result-heading">
          <p>
            在<span>{resultSource.shortName}</span>找到 {total.toLocaleString(
              "zh-CN",
            )} 条结果
          </p>
          <small>按相关度排序</small>
        </div>
        <div class="book-list">
          {#each results as book, index (book.source + book.id)}<BookCard
              {book}
              {index}
            />{/each}
        </div>
      {/if}
    </section>
  {/if}

  <footer><span>Patchouli</span><span>跨馆检索，不止于搜索</span></footer>
</main>
