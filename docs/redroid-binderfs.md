# BinderFS 问题排查处理

正常情况下宿主机 `ls /dev/binderfs` 应看到：

```text
binder
binder-control
hwbinder
vndbinder
```

如果宿主机 `/dev/binderfs` 只是一个空目录，容器内虽然仍有这些符号链接：

```text
/dev/binder -> /dev/binderfs/binder
/dev/hwbinder -> /dev/binderfs/hwbinder
/dev/vndbinder -> /dev/binderfs/vndbinder
```

但链接目标不存在。此时 ADB 可能可以连接，`pm path` 等命令却会失败：

```text
Binder driver '/dev/binder' could not be opened. Terminating.
```

需要恢复 Binder 并重建现有服务，让设备节点重新映射进容器：

```bash
sudo mkdir -p /dev/binderfs
sudo modprobe binder_linux devices=binder,hwbinder,vndbinder
mountpoint -q /dev/binderfs || sudo mount -t binder binder /dev/binderfs
docker compose up -d --wait --wait-timeout 120 --force-recreate
```
