<script lang="ts">
  import { SEARCH_ENGINES, type SearchEngine } from "../api/engines";
  import type { EngineId } from "../api/contracts";

  let {
    value,
    onchange,
  }: { value: EngineId; onchange: (id: EngineId) => void } = $props();
  let open = $state(false);

  const current = $derived(
    SEARCH_ENGINES.find((engine) => engine.id === value) ?? SEARCH_ENGINES[0],
  );

  function select(engine: SearchEngine) {
    onchange(engine.id);
    open = false;
  }
</script>

<div class="engine-select">
  <button
    class="engine-trigger"
    type="button"
    aria-label={`当前搜索引擎：${current.name}，点击切换`}
    aria-haspopup="listbox"
    aria-expanded={open}
    onclick={() => (open = !open)}
  >
    <span class="engine-mark" style={`--engine-color:${current.accent}`}
      >{current.shortName.slice(0, 1)}</span
    >
    <svg viewBox="0 0 16 16" aria-hidden="true"><path d="m4 6 4 4 4-4" /></svg>
  </button>

  {#if open}
    <button
      class="dismiss"
      type="button"
      aria-label="关闭搜索引擎菜单"
      onclick={() => (open = false)}
    ></button>
    <div class="engine-menu" role="listbox" aria-label="选择搜索引擎">
      <p class="menu-label">检索来源</p>
      {#each SEARCH_ENGINES as engine}
        <button
          type="button"
          role="option"
          aria-selected={engine.id === value}
          class:active={engine.id === value}
          onclick={() => select(engine)}
        >
          <span class="option-mark" style={`--engine-color:${engine.accent}`}
            >{engine.shortName.slice(0, 1)}</span
          >
          <span>
            <strong>{engine.name}</strong>
            <small>{engine.description}</small>
          </span>
          {#if engine.id === value}<span class="check">✓</span>{/if}
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .engine-select {
    position: relative;
    align-self: stretch;
    display: flex;
  }
  .engine-trigger {
    width: 84px;
    border: 0;
    border-right: 1px solid var(--line);
    background: transparent;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    cursor: pointer;
  }
  .engine-trigger:hover {
    background: rgba(43, 48, 43, 0.04);
  }
  .engine-trigger svg {
    width: 14px;
    fill: none;
    stroke: #77766e;
    stroke-width: 1.8;
  }
  .engine-mark,
  .option-mark {
    display: grid;
    place-items: center;
    color: white;
    background: var(--engine-color);
    font-family: var(--serif);
  }
  .engine-mark {
    width: 30px;
    height: 30px;
    border-radius: 9px;
    font-size: 14px;
  }
  .dismiss {
    position: fixed;
    inset: 0;
    z-index: 4;
    border: 0;
    background: transparent;
  }
  .engine-menu {
    position: absolute;
    z-index: 5;
    top: calc(100% + 12px);
    left: 0;
    width: 288px;
    padding: 10px;
    border: 1px solid var(--line);
    border-radius: 16px;
    background: rgba(255, 255, 252, 0.98);
    box-shadow: 0 18px 55px rgba(40, 39, 31, 0.15);
  }
  .menu-label {
    margin: 5px 8px 8px;
    color: var(--muted);
    font-size: 11px;
    letter-spacing: 0.16em;
  }
  .engine-menu button {
    width: 100%;
    padding: 10px;
    border: 0;
    border-radius: 11px;
    background: transparent;
    display: grid;
    grid-template-columns: 34px 1fr 20px;
    gap: 10px;
    align-items: center;
    text-align: left;
    cursor: pointer;
  }
  .engine-menu button:hover,
  .engine-menu button.active {
    background: var(--paper-dark);
  }
  .option-mark {
    width: 32px;
    height: 32px;
    border-radius: 50%;
  }
  strong,
  small {
    display: block;
  }
  strong {
    font-size: 14px;
    font-weight: 600;
    color: var(--ink);
  }
  small {
    margin-top: 2px;
    font-size: 11px;
    color: var(--muted);
  }
  .check {
    justify-self: end;
    color: var(--green);
  }
</style>
