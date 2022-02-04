# RWR GFL 存档管理服务

## 快速上手

> 该项目依赖前端运行，对应前端：https://github.com/Kreedzt/rwr-profile-web

下载前端[构建后代码](https://github.com/Kreedzt/rwr-profile-server/releases)，然后下载本项目[构建后代码](https://github.com/Kreedzt/rwr-profile-server/releases)，配合 nginx 托管，并放置 `config.json` 文件和 `users.json` 文件，最后从命令行启动该项目即可

`config.json`(需要与该项目同目录):
```json5
{
  // rwr 存档目录，建议使用相对路径
  "rwr_profile_folder_path": "temp/profiles",
  // 服务器数据目录，不能为空，路径必须存在 users.json
  "server_data_folder_path": "temp/data",
  // 服务器日志目录
  "server_log_folder_path": "temp/logs"
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

## 构建

该项目采用 Rust 语言编写，需要 [Rust](https://www.rust-lang.org/) 开发环境

编译需执行以下命令：
```bash
cargo build --release
```

编译后在根目录的 `target/release` 内生成二进制文件（exe），该文件可用终端直接运行。

编译后可用 [upx](https://github.com/upx/upx) 二次缩小体积，通常缩小到 700k 左右

```bash
upx --best --lzma target/release/rwr-profile-server
```

## 特性

- 用户接口
  + 提供基本的注册与登录
- 经验管理接口
  + 经验重置到5星人形
- 仓库管理接口
  + 仓库更新
- 改造管理接口
  + 改造更换
- 背包管理接口
  + 背包更新

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
  "server_log_folder_path": "temp/logs"
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


## 协议

- [GPLv3](https://opensource.org/licenses/GPL-3.0)