---
title: mysql 启动
categories: Docker
---

## mysql启动

```shell
docker run -itd --name mysql-test -p 3306:3306 -e MYSQL_ROOT_PASSWORD=123456 mysql
```
