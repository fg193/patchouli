# 使用 Redroid 测试

本文记录在 Redroid 中测试本项目的完整流程，以及调试过程中遇到的问题、判断方法和解决方案。


## 一、构建 aarch64 release

在 amd64 环境中，使用 Google 官方 `sdkmanager` 安装 Android SDK 及 NDK 即可。
在 aarch64 环境中，官方目前没有提供预编译好的 NDK 工具链，请参考以下项目配置环境：

- 如何编译 NDK：https://github.com/SnowNF/ndk-aarch64-linux
- 编译好的 NDK：https://github.com/zhuwanhong/android-sdk-linux-arm64

首次构建前，建议准备一个 keystore 用于应用签名。
如果之前编译过其他应用，可以复用原有 `~/.android/debug.keystore`，否则可以创建一个：

```bash
keytool -genkey -v -keystore "/absolute/path/to/debug.keystore" -keyalg RSA -keysize 2048 -validity 10000 -alias androiddebugkey
```

基于下面的示例，创建文件 `src-tauri/gen/android/keystore.properties`，并按实际情况修改各个属性：

```properties
keyAlias=androiddebugkey
storePassword=android
keyPassword=android
storeFile=/absolute/path/to/debug.keystore
```

为了缩短构建时间，减小产物体积，测试中始终构建 split APK，而不是 universal APK：

```bash
npx tauri android init
npx tauri android build -- --target aarch64 --apk --split-per-abi --ci
```

成品路径：

```text
src-tauri/gen/android/app/build/outputs/apk/arm64/release/app-arm64-release.apk
```

初次构建后，检查签名是否已生效：

```bash
apksigner verify --print-certs \
  src-tauri/gen/android/app/build/outputs/apk/arm64/release/app-arm64-release.apk
```


## 二、启动并验证 Android 环境

[compose.yml](compose.yml) 已在 aarch64 宿主机上验证过：

```bash
docker compose up -d --wait --wait-timeout 120
adb connect localhost:5555
```

测试时必须分别确认以下三层健康状态:

1. **容器状态**：Redroid 进程正在运行，网络正常（已包含在 `--wait` healthcheck 中）。
2. **Android 状态**：Binder 设备可用，Zygote 正常运行，系统启动完成，`pm` 等系统服务可以调用。
3. **应用状态**：APK 可安装和启动，实际请求已提交，进程没有 panic、Java exception 或 native signal。

Redroid 依赖宿主机的 Binder 设备。当前 healthcheck 只负责验证容器网络路由和系统启动状态，
不能仅凭 Docker 的 `healthy` 判断 Android 已经可用，需补充 Binder 状态验证：

```
adb -s localhost:5555 shell pm path com.android.setting
```

预期结果：`pm path` 能正常返回 APK 路径。
如果遇到 Binder 错误，请按 [redroid-binderfs.md](redroid-binderfs.md) 提示，完成修复后再继续。


## 三、覆盖安装并启动

```bash
adb -s localhost:5555 install -r \
  src-tauri/gen/android/app/build/outputs/apk/arm64/release/app-arm64-release.apk

adb -s localhost:5555 shell am force-stop io.github.fg193.patchouli
adb -s localhost:5555 shell am start -W \
  -n io.github.fg193.patchouli/.MainActivity
adb -s localhost:5555 shell pidof io.github.fg193.patchouli
```

如果覆盖安装提示签名不一致，不要立即卸载应用；卸载会清除 `/data` 中的应用数据。


## 四、实际搜索与 logcat 回归测试

WebView 页面通常无法通过 `uiautomator dump` 暴露完整 DOM 文本，因此本次采用截图确认控件位置，再用 ADB 输入事件操作。

截图：

```bash
adb -s localhost:5555 exec-out screencap -p > /tmp/patchouli.png
```

输入中文时，`adb shell input text` 容易受编码和输入法影响。
自动化回归优先使用 ASCII 查询，例如搜索 `agent`，避免把输入故障误判成 provider 故障。

测试前清空日志并记录 PID：

```bash
before_pid=$(adb -s localhost:5555 shell pidof io.github.fg193.patchouli)
adb -s localhost:5555 logcat -c
```

在 UI 中选择“国家图书馆”或其他搜索引擎，输入查询，并明确点击搜索按钮。
不要仅凭键盘 Enter 判断表单已提交；应隐藏键盘，并通过截图确认结果区域已经更新。

请求完成后收集判定信号：

```bash
after_pid=$(adb -s localhost:5555 shell pidof io.github.fg193.patchouli)
echo "before=$before_pid after=$after_pid"

adb -s localhost:5555 shell dumpsys activity activities \
  | grep -m1 mResumedActivity

adb -s localhost:5555 logcat -d -v threadtime \
  > /tmp/patchouli-logcat.txt

grep -E \
  'FATAL EXCEPTION|Fatal signal|SIGSEGV|SIGABRT|AndroidRuntime|am_crash|Process io\.github\.fg193\.patchouli .* has died|panicked at' \
  /tmp/patchouli-logcat.txt
```

真正的 crash 至少会出现以下一种证据：

- 应用 PID 消失或改变；
- 前台 Activity 返回 Launcher；
- `FATAL EXCEPTION`；
- `Fatal signal 6 (SIGABRT)`、`SIGSEGV` 等 native signal；
- Rust `panicked at`；
- ActivityManager 报告应用进程死亡。

页面显示“网络请求失败”但 PID 仍在、Activity 仍在前台，不是 crash，而是应用捕获并展示了错误。必须区分这两类结果。


## 五、常见故障速查

| 现象 | 判定 | 解决方案 |
|------|------|----------|
| 容器 `exited` | redroid 未运行 | `docker logs redroid` 查看退出原因，排除问题后再用 Compose 启动容器 |
| 容器健康检查超时 | 容器网络连接问题 | 参考 [redroid-network.md](redroid-network.md) 进行排查修复 |
| 容器启动成功，无法执行 `pm path` | 缺少 binderfs 内核模块 | 参考 [redroid-binderfs.md](redroid-binderfs.md) 进行排查修复 |
| 页面显示网络错误但应用仍在 | 受控错误，不是 crash | 检查目标站 TCP/TLS 可达性及响应 |
| 搜索后直接回 Launcher | 应用进程崩溃 | 对比 PID，抓取 panic、AndroidRuntime 和 native signal |
