---
title: redis 启动
categories: Docker
---



## 拉取redis镜像

```shell
docker pull redis # 默认拉取latest的镜像
```

## redis启动并且设置密码

```shell
docker run -d --name <redis容器的名字> -p 6379:6379 redis --requirepass "password"
```

# redis.conf

- 允许外地连接

> 注释了 bind 127.0.0.1

- daemonize no

> daemonize yes注释起来或者daemonize no设置，因为该配置和docker run中-d参数冲突，会导致容器一直启动失败

开启redis持久化

> appendonly yes 持久化
>
> Provate-remote no # 外部链接redis 保护模式关闭



# 启动

``` 
docker run -p6379:6379 --name r3 --privileged=true
-v /app/redis/redis.conf:/ect/redis/redis.conf
-v /app/redisdata:/data
-d redis redis-server /ect/redis/redis.conf
```

> redis server 的启动配置读取指定的路径

```shell
redis-server /ect/redis/redis.conf
```

修改之后需要重新启动服务

## exec后的shell 测试
```shell
$ redis-cli

$ auth "password"

select 3
```