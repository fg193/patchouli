## Compose 配置与路由修复

### 为什么主路由表正常，redroid 仍然无法访问网关

Docker 会把地址、直连路由和默认路由放入 `main` 表；
Android 的策略路由规则却可能把未标记流量导向 `local_network` 表。
后者为空时，即使下面的命令能在 `main` 表看到默认路由，Android 流量仍无法到达 Docker gateway：

```bash
docker exec redroid ip route show table main
docker exec redroid ip rule
docker exec redroid ip route show table local_network
```

healthcheck 会把 `main` 表路由复制到 `local_network`。
必须先复制直连路由，再复制默认路由；否则内核在添加默认路由时还不知道 gateway 可通过哪个直连网络到达，可能返回：

```text
RTNETLINK answers: Network is unreachable
```

### 为什么修完路由后执行 `exit 1`

运行检测命令：

```bash
docker exec redroid ping -c 1 1.1.1.1
ip route get 1.1.1.1
```

预期路由包含 `table local_network`，例如：

```text
1.1.1.1 via 172.18.0.1 dev eth0 table local_network src 172.18.0.2
```

healthcheck 分成两次检查：

1. 第一次发现 `ip route get 1.1.1.1` 失败，写入路由，然后以 `1` 退出。
2. Docker 按 healthcheck 调度再次执行同一命令；这次快速检查成功，以 `0` 退出，容器变为 `healthy`。

`exit 1` 不会终止后续 healthcheck。循环由 Docker healthcheck 调度器负责，不是 shell 中的 `continue`。

`start_interval: 6s` 让启动期第一次检查推迟约 6 秒，给 Android 网络栈时间创建策略路由表。
这里不再检查 gateway 地址，也不使用 `getprop sys.boot_completed` 作为修路由条件。
路由本身是否可写、`ip route get` 是否成功就是该 healthcheck 的直接判定信号。

### 不要使用 host network

不要给 privileged redroid 配置 `network_mode: host` 或 `--net=host`。
Android 会管理自己的网络栈和路由规则，与宿主机共享 network namespace 时可能修改或破坏宿主机网络。

若想区分 Docker 宿主 bridge 故障和 redroid 内部路由故障，可以临时运行最小 Alpine 容器测试：

```bash
docker run --rm alpine ping -c 1 1.1.1.1
```

如果普通 Alpine 能联网，而 redroid 连 gateway 都不通，应优先检查 redroid 的 `ip rule` 和 `local_network` 表。
