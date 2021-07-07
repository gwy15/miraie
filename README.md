# miraie 未来へ

[![Build](https://github.com/gwy15/miraie/actions/workflows/build.yml/badge.svg)](https://github.com/gwy15/miraie/actions/workflows/build.yml)

Miraie 是一个基于 [mirai](https://github.com/mamoe/mirai) 和 [mirai-api-http](https://github.com/project-mirai/mirai-api-http) 的 Rust 机器人框架。

# 环境变量
- `MIRAIE_RESOURCE_ROOT`：资源的根目录，这个需要是 mirai 运行时的目录，如果 mirai 运行在机器 A 上，rust bot 运行在机器 B 上，这个需要是机器 A 上的路径。
