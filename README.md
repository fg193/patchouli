# Patchouli

跨图书馆文献检索应用

目前支持的网站：

- 豆瓣读书
- 首都图书馆
- 国家图书馆


## 架构

项目采用 Svelte 5 + Tauri 2 框架。

- `src/lib/api/contracts.ts`：前后端共享概念的 TypeScript 版本。
- `src/lib/api/client.ts`：UI 依赖的 API 端口；组件不直接依赖任何外部服务。
- `src/lib/api/providers.ts`：provider 注册表与展示元数据。
- `src-tauri/`：Tauri 应用 crate `patchouli-tauri`，负责启动应用、注册 IPC command，以及组装 OPAC provider。
- `src-opac/`：独立的 Rust crate `patchouli-opac-api`，包含 OPAC 接口、领域模型、provider 注册表和各服务适配器。
- `src-opac/src/providers/`：豆瓣、首图、国图等适配器。

依赖方向保持单向：
- `src-tauri` 依赖 `src-opac`，provider 在 Tauri 应用的组合根中注册。
- OPAC API 通过 `Provider` trait 和 `Opac::register` 接收 provider，不依赖 Tauri，可以脱离 Tauri 运行和测试。

新增适配器时，在前端和 `src-opac/src/models.rs` 的 `ProviderId` 中注册，在 `src-opac/src/providers/` 添加实现，并在 `providers::all()` 中加入 provider。新增非搜索 API 时，在 `src-opac` 下建立新的领域模块并注册独立 Tauri command，避免把不相关能力塞进搜索接口。


## 开发

OPAC API 模块仅依赖 Rust Stable：

```bash
cargo test --manifest-path src-opac/Cargo.toml
```

若要开发整个 Tauri 应用，需先安装 [Tauri 2 的各项系统依赖及 Node.js](https://v2.tauri.app/start/prerequisites/)，再运行：

```bash
npm install
npm run tauri dev
```

构建 Tauri 应用，以 Android 平台为例：

```bash
npm run tauri android init
npm run tauri android build -- --apk --split-per-abi --target aarch64
```
