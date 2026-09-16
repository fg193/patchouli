# 页间（Patchouli）

一个 Svelte 5 + Tauri 2 的跨馆藏图书检索桌面应用。

目前支持的网站：

- 豆瓣读书
- 首都图书馆
- 国家图书馆

## 开发

先安装 [Tauri 2 的系统依赖和 Rust](https://v2.tauri.app/start/prerequisites/)，再运行：

```bash
npm install
npm run tauri dev
```

仅调试前端可运行 `npm run dev`。浏览器中没有 Tauri IPC，因此实际检索必须在 Tauri 窗口中测试。

## 架构

- `src/lib/api/contracts.ts`：前后端共享概念的 TypeScript 版本。
- `src/lib/api/client.ts`：UI 依赖的 API 端口；组件不直接依赖任何外部服务。
- `src/lib/api/engines.ts`：搜索引擎注册表与展示元数据。
- `src-tauri/src/api/`：Rust 应用层与各服务适配器。每个适配器独立负责请求和统一书目模型转换。

新增搜索引擎时，在两端的 `EngineId` 中注册，并在 `src-tauri/src/api/` 添加适配器。新增非搜索 API 时，在 `api` 下建立新的领域模块并注册独立 Tauri command，避免把不相关能力塞进搜索接口。
