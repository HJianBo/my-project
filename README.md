# Simple Rust Tetris

这个版本保留 Rust + macroquad，实现浏览器运行。

## 本地桌面运行

```bash
cargo run
```

## 浏览器运行

1. 构建 Web 资源（默认 debug）：

```bash
./scripts/build_web.sh
```

或构建 release：

```bash
./scripts/build_web.sh release
```

2. 启动静态服务器：

```bash
./scripts/serve_web.sh
```

3. 浏览器打开：

```text
http://127.0.0.1:8000
```
