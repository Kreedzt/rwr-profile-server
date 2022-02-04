# RWR GFL 存档管理服务

## 快速上手

> 该项目依赖前端运行，对应前端：https://github.com/Kreedzt/rwr-profile-web

下载前端构建后代码，然后下载本项目[构建后代码](https://github.com/Kreedzt/rwr-profile-web/releases)，配合 nginx 托管，并从命令行启动该项目即可

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


## 协议

- GPL