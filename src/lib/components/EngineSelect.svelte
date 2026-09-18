<script lang="ts">
  import { getProvider, PROVIDERS, type Provider } from "../api/providers";
  import type { ProviderId } from "../api/contracts";
  import { ChevronDown } from "@lucide/svelte";

  let {
    value,
    onchange,
  }: { value: ProviderId; onchange: (id: ProviderId) => void } = $props();
  let open = $state(false);
  let container: HTMLDivElement;

  const current = $derived(getProvider(value));

  $effect(() => {
    const closeOnOutsidePointer = (event: PointerEvent) => {
      if (open && container && !container.contains(event.target as Node)) {
        open = false;
      }
    };

    document.addEventListener("pointerdown", closeOnOutsidePointer);
    return () =>
      document.removeEventListener("pointerdown", closeOnOutsidePointer);
  });

  function select(provider: Provider) {
    onchange(provider.id);
    open = false;
  }
</script>

<div class="engine-select" bind:this={container}>
  <button
    class="engine-trigger"
    type="button"
    aria-label={`当前搜索引擎：${current.name}，点击切换`}
    aria-haspopup="listbox"
    aria-expanded={open}
    onclick={() => (open = !open)}
  >
    <span class="engine-mark">{current.shortName}</span>
    <ChevronDown aria-hidden="true" />
  </button>

  {#if open}
    <div class="engine-menu" role="listbox" aria-label="选择搜索引擎">
      <p class="menu-label">检索来源</p>
      {#each PROVIDERS as provider}
        <button
          type="button"
          role="option"
          aria-selected={provider.id === value}
          class:active={provider.id === value}
          onclick={() => select(provider)}
        >
          <span class="option-mark" style={`--engine-color:${provider.accent}`}
            >{provider.shortName.slice(0, 1)}</span
          >
          <span>
            <strong>{provider.name}</strong>
            <small>{provider.description}</small>
          </span>
          {#if provider.id === value}<span class="check">✓</span>{/if}
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
    flex: 0 0 auto;
  }
  .engine-trigger {
    width: 68px;
    flex: 0 0 68px;
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
  .engine-trigger :global(svg) {
    width: 14px;
    fill: none;
    stroke: #77766e;
    stroke-width: 1.8;
  }
  .engine-mark,
  .option-mark {
    display: grid;
    place-items: center;
    font-family: var(--serif);
  }
  .engine-mark {
    color: var(--ink);
    font-size: 13px;
    white-space: nowrap;
  }
  .option-mark {
    color: white;
    background: var(--engine-color);
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
