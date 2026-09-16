<script lang="ts">
  import BookCard from "./lib/components/BookCard.svelte";
  import EngineSelect from "./lib/components/EngineSelect.svelte";
  import { tauriSearchApi } from "./lib/api/client";
  import { getEngine } from "./lib/api/engines";
  import type { ApiFailure, BookResult, EngineId } from "./lib/api/contracts";

  let engine = $state<EngineId>("douban");
  let query = $state("");
  let results = $state<BookResult[]>([]);
  let total = $state(0);
  let loading = $state(false);
  let error = $state("");
  let hasSearched = $state(false);

  const selectedEngine = $derived(getEngine(engine));

  async function search() {
    const normalized = query.trim();
    if (!normalized || loading) return;

    loading = true;
    error = "";
    hasSearched = true;
    try {
      const response = await tauriSearchApi.search({
        engine,
        query: normalized,
        page: 1,
        pageSize: 20,
      });
      results = response.items;
      total = response.total;
    } catch (cause) {
      const failure = cause as Partial<ApiFailure> | string;
      error =
        typeof failure === "string"
          ? failure
          : (failure.message ?? "检索暂时不可用，请稍后再试。");
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
      ? `${query} · ${selectedEngine.shortName}`
      : "页间 · 图书检索"}</title
  ></svelte:head
>

<main class:searched={hasSearched}>
  <header>
    <a class="brand" href="/" aria-label="页间首页">
      <span class="brand-mark">頁</span>
      <span>页间</span>
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
      <EngineSelect value={engine} onchange={(id) => (engine = id)} />
      <input
        bind:value={query}
        aria-label="搜索关键词"
        placeholder={engine === "clcn"
          ? "在首图按作者检索"
          : `在${selectedEngine.shortName}搜索书名、作者或 ISBN`}
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
          <p>正在翻阅 {selectedEngine.name}…</p>
        </div>
      {:else if error}
        <div class="status error-state">
          <span>!</span>
          <h2>这一页暂时翻不开</h2>
          <p>{error}</p>
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
            <span>{selectedEngine.shortName}</span> 找到约 {total.toLocaleString(
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

  <footer>
    <span>页间 · PATCHOULI</span><span>跨馆检索，不止于搜索</span>
  </footer>
</main>
