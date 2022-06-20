# RWR 存档管理系统部署流程 

## 文件准备(仅初次需要)

1. 建立相关文件夹:
  - data: 用于存放数据文件
  - logs: 用于存放服务端日志
  - upload_temp: 用于存放临时文件上传
  
2. data 文件夹中新建如下文件:
  - users.json
  - quick_items.json
  - ranks.json(可选, 查询系统依赖此文件)

3. 初始化文件内容
  - user.json 写入内容(参考 `users_example.json`):
    ```json
    {"user_list":[]}
    ```
  - quick_items.json 写入内容(参考 `quick_items_example.json`):
    ```json
    []
    ```
  - ranks.json 写入内容(参考 `ranks_example.json`):
    ```json
    []
    
    ```
    > 注意: ranks.json 为对查询系统提供等级查询进度条, 所以此项内容需视情况填充:
    单项格式如下:
    ```json
    [
      {
        "xp": 0.0,
        "name": "2 星人形"
      }
    ]
    ```
    + xp 对应军衔要求 xp
    + name 对应名称(为前端标识)

## 启动

目前支持 2 种启动方式:
- 二进制直接启动
- Docker 启动

### 二进制直接启动

先根据平台下载 [构建后代码](https://github.com/Kreedzt/rwr-profile-server/releases)

然后编写 `config.json`, 将 `config.json` 放置到与二进制文件 **同路径** 中

config.json 配置项参考如下:
```json
{
  "rwr_profile_folder_path": "../profiles",
  "server_data_folder_path": "../data",
  "server_log_folder_path": "../logs",
  "server_upload_temp_folder_path": "../upload_temp",
  "server_hourly_request": false,
  "port": 8080
}
```

- `rwr_profile_folder_path`: rwr 存档目录，建议使用相对路径
- `server_data_folder_path`: 服务器数据目录，不能为空，路径必须存在 users.json 与 quick_items.json
- `server_log_folder_path`: 服务器日志目录
- `server_upload_temp_folder_path`: 服务器上传存档临时目录
- `server_hourly_request`: 服务端是否每小时请求查询存档数据并缓存(用于查询系统)
- `port`: 服务绑定的 TCP 端口

项目结构参考:
```text

|-- profiles/
|-- logs/
|-- upload_temp/
|-- data/
|---- users.json
|---- quick_items.json
|---- ranks.json
|-- server/
|---- rwr-profile-server.exe
|---- config.json
```
  
以上操作完成后, 直接运行二进制文件即可启动

## Docker 启动

对外的暴露的端口为 8080

挂载的目录说明:
- /app/data: 对应文件准备中的 `data` 目录
- /app/logs: 对应文件准备中的  `logs` 目录
- /app/profiles: 对应 rwr 存档目录
- /app/upload_temp: 对应文件准备中的 `upload_temp`

启动命令参考:
```sh
docker run --name=rwr-profile-server-docker -d -p 8080:8080 -v $PWD/data:/app/data -v $PWD/logs:/app/logs -v $PWD/profiles:/app/profiles -v $PWD/upload_temp:/app/upload_temp -d zhaozisong0/rwr-profile-server:latest
```

### 额外说明

在初次用户注册后, 可在 `data/users.json` 中找到用户名, 将 `admin` 值修改为 1 来标识为管理员

例:
```json
{"user_list":[{"name":"KREEDZT","user_id":11111,"password":"====","admin":1}]}
```
