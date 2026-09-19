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
npx tauri dev
```

在移动端热部署 Tauri 应用进行开发和预览，以 Android 平台为例：

```bash
npx tauri android init
adb reverse tcp:1420 tcp:1420
npx tauri android dev
```

构建 Tauri 应用，以 Android 平台为例，更详细的测试流程可参考 [docs/redroid-testing.md](docs/redroid-testing.md)：

```bash
npx tauri android init
npx tauri android build -- --apk --split-per-abi --target aarch64
```

修改代码后，需保证代码正确格式化：

```bash
npm run format
```


## 致谢

- [上海 Alice 幻乐团](http://www16.big.or.jp/~zun/)，[项目命名参考](https://thwiki.cc/Patchouli)
- [Stellarium](https://stellarium-web.org)，[主图标设计参考](https://github.com/Stellarium/stellarium/blob/master/data/stellarium.icns)
- [汪磊同学](https://github.com/zce)，[豆瓣 API 文档](https://fg193.github.io/douban-api-docs/docs/book.html)
- [Paseo](https://paseo.sh)，主要工作在该开发环境中完成
- Oracle Cloud, 大部分开发及测试在[龟壳免费安培服务器](https://docs.oracle.com/en-us/iaas/Content/FreeTier/freetier_topic-Always_Free_Resources.htm)中进行
- Livid，部分代码开发工作使用了 [V2EX AI Chat 免费提供的 DeepSeek-V4-Flash 0731 模型](https://www.v2ex.com/t/1231448)
- [SnowNF](https://github.com/SnowNF/ndk-aarch64-linux) 及 [zhuwanghong](https://github.com/zhuwanhong/android-sdk-linux-arm64)，提供了在 aarch64 运行 NDK 的文档
- [Lucide 图标包](https://lucide.dev/icons/)，以及[成百上千的其他开源依赖](https://github.com/fg193/patchouli/network/dependencies)
