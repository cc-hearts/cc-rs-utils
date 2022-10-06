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
