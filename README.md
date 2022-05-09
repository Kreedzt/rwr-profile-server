# RWR GFL 存档管理服务

## 快速上手

> 该项目依赖前端运行，对应前端：https://github.com/Kreedzt/rwr-profile-web

下载前端[构建后代码](https://github.com/Kreedzt/rwr-profile-web/releases)，然后下载本项目[构建后代码](https://github.com/Kreedzt/rwr-profile-server/releases)，配合 nginx 托管，并放置 `config.json` 文件和 `users.json` 文件，最后从命令行启动该项目即可

`config.json`(需要与该项目同目录):
```json5
{
  // rwr 存档目录，建议使用相对路径
  "rwr_profile_folder_path": "temp/profiles",
  // 服务器数据目录，不能为空，路径必须存在 users.json
  "server_data_folder_path": "temp/data",
  // 服务器日志目录
  "server_log_folder_path": "temp/logs",
  // 服务器上传存档临时目录，目标路径必须存在
  "server_upload_temp_folder_path": "temp/upload_temp"
  // 服务端是否每小时请求查询存档数据并缓存(用于查询系统)
  "server_hourly_request": true,
  // 服务绑定的 TCP 端口
  "port": 8080
}
```

`users.json`（需要放在 `config.json` 中设置的 `server_data_folder_path` 路径中）
> user_list 内容可以为空, 即为空数组也行, 第一次可以通过 web 页面注册用户, 然后手动修改 admin 标识
```json5
{
  "user_list":[
    {
      // 用户名
      "name":"AAA",
      // 对应的存档用户 id
      "user_id":1432226718,
      // 密码(编码后)
      "password":"YWFh",
      // 是否管理员标识
      "admin":1
    }
  ]
}
```

项目结构参考:

``` text
|- temp/
|-- profiles/
|-- logs/
|-- upload_temp/
|-- data/
|--- users.json
|- rwr-profile-server.exe
|- config.json
```

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
  + 下载存档
  + 上传存档
- 玩家记录信息接口（profile）
  + 下载存档
  + 上传存档
  + 查询所有信息缓存

## 部署

该项目仅是后端，需要前端项目一起部署

可用 [nginx](http://nginx.org/en/download.html) 部署

`nginx.conf` 反向代理如下配置即可:

```conf
server {
        listen 9090;
        server_name localhost;

        # 前端页面
        location / {
            root /dist;
            try_files $uri $uri/ /index.html;
            index index.html;
        }

        # API 转发
        location /api/ {
         	proxy_set_header HOST $host;
        	proxy_set_header X-Forwarded-Proto $scheme;
       	 	proxy_set_header X-Real-IP $remote_addr;
        	proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_pass http://127.0.0.1:8080/;
        }
}
```

在此二进制同路径放置 `config.json` 文件

`config.json`(需要与该项目同目录):
```json5
{
  // rwr 存档目录，建议使用相对路径
  "rwr_profile_folder_path": "temp/profiles",
  // 服务器数据目录，不能为空，路径必须存在 users.json
  "server_data_folder_path": "temp/data",
  // 服务器日志目录
  "server_log_folder_path": "temp/logs",
  // 服务器上传存档临时目录，目标路径必须存在
  "server_upload_temp_folder_path": "temp/upload_temp"
  // 服务端是否每小时请求查询存档数据并缓存(用于查询系统)
  "server_hourly_request": true,
  // 服务绑定的 TCP 端口
  "port": 8080
}
```

在 `server_data_folder_path` 路径放置 `users.json`:
> user_list 内容可以为空, 即为空数组也行, 第一次可以通过 web 页面注册用户, 然后手动修改 admin 标识
```json5
{
  "user_list":[
    {
      // 用户名
      "name":"AAA",
      // 对应的存档用户 id
      "user_id":1432226718,
      // 密码(编码后)
      "password":"YWFh",
      // 是否管理员标识
      "admin":1
    }
  ]
}
```

## 其他项目

- [RWR GFL 存档数据可视化](https://github.com/Kreedzt/rwr-profile-visualization)
- [RWR GFL 存档管理系统](https://github.com/Kreedzt/rwr-profile-web)
- [RWR GFL 存档数据查询](https://github.com/Kreedzt/rwr-profile-stats)

## 协议

- [GPLv3](https://opensource.org/licenses/GPL-3.0)
