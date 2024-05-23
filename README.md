# RWR GFL 存档管理服务

![license](https://badgen.net/github/license/Kreedzt/rwr-profile-server)
![latest release](https://badgen.net/github/release/Kreedzt/rwr-profile-server)
[![Maintainability Rating](https://sonarcloud.io/api/project_badges/measure?project=Kreedzt_rwr-profile-server&metric=sqale_rating)](https://sonarcloud.io/summary/new_code?id=Kreedzt_rwr-profile-server)
[![Docker Image Size](https://badgen.net/docker/size/zhaozisong0/rwr-profile-server?icon=docker&label=image%20size)](https://hub.docker.com/r/zhaozisong0/rwr-profile-server)
![rwr_version](https://badgen.net/badge/RWR/1.97/orange)

## 开发
> 该项目提供了前端界面, 可用前端界面来进行开发调试: https://github.com/Kreedzt/rwr-profile-web

该项目采用 Rust 语言编写，需要 [Rust](https://www.rust-lang.org/) 开发环境

启动开发环境的命令:

``` sh
cargo run
```

会绑定在 `8080` 端口工作

## 构建

该项目采用 Rust 语言编写，需要 [Rust](https://www.rust-lang.org/) 开发环境

编译需执行以下命令：
```bash
cargo build --release
```

编译后在根目录的 `target/release` 内生成二进制文件（exe），该文件可用终端直接运行。

编译后可用 [upx](https://github.com/upx/upx) 二次缩小体积，通常缩小到 800k 左右

```bash
upx --best --lzma target/release/rwr-profile-server
```

## 特性

- 用户接口（user）
  + 提供基本的注册与登录
- 玩家数据接口（person）
  + 查询单条玩家信息
  + 更新单条玩家信息
  + 查询所有玩家信息
  + 经验重置到 5 星
  + 经验重置到指定值
  + 更新背包
  + 更新仓库
  + 更新改造
  + 为所有玩家背包插入指定物品
  + 为选定玩家背包插入指定物品
  + 为所有玩家更改兵种
  + 为指定玩家更改兵种
  + 为所有玩家移除物品
  + 为指定玩家移除物品
  + 下载存档
  + 上传存档
- 玩家记录信息接口（profile）
  + 下载存档
  + 上传存档
  + 查询所有信息缓存

## 部署

见 [部署文档](https://github.com/Kreedzt/rwr-profile-server/blob/master/DEPLOYMENT.md)

## 其他项目

- [RWR GFL 存档数据可视化](https://github.com/Kreedzt/rwr-profile-visualization)
- [RWR GFL 存档管理系统](https://github.com/Kreedzt/rwr-profile-web)
- [RWR GFL 存档数据查询](https://github.com/Kreedzt/rwr-profile-stats)

## 协议

- [GPLv3](https://opensource.org/licenses/GPL-3.0)
