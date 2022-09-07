---
title: Linux 基础
categories: Linux
---

## 目录结构

> 在 linux 中 一切皆为文件
> cd ：change directory

```shell
lrwxrwxrwx.  1 root root     7 Jul 11  2019 bin -> usr/bin

dr-xr-xr-x.  5 root root  4096 Sep  2 09:06 boot

drwxr-xr-x  19 root root  2980 Aug 22 12:32 dev

drwxr-xr-x. 77 root root  4096 Sep  2 09:04 etc

drwxr-xr-x.  3 root root  4096 Aug 20 12:58 home

lrwxrwxrwx.  1 root root     7 Jul 11  2019 lib -> usr/lib

lrwxrwxrwx.  1 root root     9 Jul 11  2019 lib64 -> usr/lib64

drwx------.  2 root root 16384 Jul 11  2019 lost+found

drwxr-xr-x.  2 root root  4096 Apr 11  2018 media

drwxr-xr-x.  2 root root  4096 Apr 11  2018 mnt

drwxrwxr-x.  4 root root  4096 Aug 22 13:08 opt

dr-xr-xr-x  89 root root     0 Aug 22 12:03 proc

dr-xr-x---. 10 root root  4096 Aug 22 13:17 root

drwxr-xr-x  25 root root   680 Sep  2 09:09 run

lrwxrwxrwx.  1 root root     8 Jul 11  2019 sbin -> usr/sbin

drwxr-xr-x.  2 root root  4096 Apr 11  2018 srv

dr-xr-xr-x  13 root root     0 Aug 22 20:03 sys

drwxrwxrwt.  8 root root  4096 Sep  2 09:09 tmp

drwxr-xr-x. 13 root root  4096 Jul 11  2019 usr

drwxr-xr-x. 19 root root  4096 Jul 11  2019 var
```

1. `bin` 目录 指令集管理目录
2. `dev` 设备管理目录(硬件管理)
3. `lib lib64` 都是库管理目录
4. `mdia` dvd、光驱、u 盘等管理目录文件
5. `opt` 用于需要安装的软件 管理目录
6. `proc src sys` 内核文件
7. `root` root 用户文件
8. `sbin` 高权限指令目录文件
9. `sys` 系统文件
10. `tmp` 临时文件夹
11. `usr` 用户文件夹
12. `var` 变量、日志文件夹
13. `lost+found` 系统非法关机后 这里就存放了一些文件
14. `etc` 所有系统管理所需要的配置文件和子目录
15. `boot` 启动 Linux 时的使用的一些核心文件

## vi 和 vim

> 所有的 Linux 系统都会内建 vi 文本编辑器
> `vim` 可以理解为 vi 的增强版

### 三种模式

1. 正常模式
2. 编辑模式
   按下`i` 进入编辑模式
3. 命令行模式
   按下`esc` 进入 命令行模式 `w` 代表写入 `q`代表退出 `q!`强制退出

## 常用快捷键

1. 复制：yy 复制当前行 p 粘贴 (3yy 复制当前行往下的 3 行 包括当前行)
2. 删除：dd 删除当前行 5dd 删除当前行下的 5 行
3. 查找： 按下 `/` 输入关键字 按下 回车查找
4. 快速到文档的最末行[G] 快速到文档的首行[gg]
5. 设置行号：`set nu` 显示行号 `set nonu`取消行号
6. 撤销上一个动作 按下`u`
   > 更多资料:

## 关机重启指令

```shell
shutdown -h now #立即关机
shutdown -h 1 #一分钟后关机
shutdown -r now #现在重新启动计算机
halt #关机
reboot # 重启计算机
sync #把内存的数据同步到磁盘 建议在关机和重启的时候优先执行
```

## 用户操作命令

```shell
	logout #退出用户登录 在运行级别3下有效
```

### 添加用户

```shell
useradd <用户名> # 如果没有指定组 则会创建一个与用户名同名的组 并且会将用户添加到改组下

# cd /home
# ll 便可以看到有用户名的一个文件夹


user -d <path> <用户名> # 创建用户名的时候指定home的目录 可以在登录的时候
# 如果先创建了文件夹 在使用该命令创建用户 则用户不会绑定用户组

useradd -g <用户组> <用户名> 添加用户到用户组内
```

> `home`目录会存在用户组

### 给用户修改密码

```shell
passwd <用户名> # 修改用户名的密码
```

### 删除用户

```shell
userdel <用户名> # 删除用户 但是不会删除用户名

userdel -r <用户名> # 删除用户的时候 并且删除用户的家目录

## cd /home
## ll # 可以查看到用户的家目录

# 尽量删除用的时候 保留家目录
```

### 查询用户

```shell
id <用户名>

# id xm
# uid=1001(xm) gid=1001(xm) groups=1001(xm)
# uid 用户id gid 组id groups 组的名次

```

#3# 切换用户

```shell
su - <用户名> # 高权限用户切换到低权限的用户 不需要密码

# 如果切换过去了 则可以使用exit 退出当前用户 返回到原来的用户
```

### 查看当前的用户

```
whoami # 查看当前的用户名
```

## 用户组

类似于角色 系统可以对有共性的多个用户进行统一的管理

### 新增用户组

```shell
groupadd <用户组名称>

# 添加用户时直接加上组

# 1.先创建组 groudadd tmp-group
# 2. 添加用户到组内
# useradd -g tmp-group <用户名>
# 给新增的用户分配了用户组名之后 家目录是根据用户组来的 而不是用户
```

### 删除用户组

```shell
	groupdel <用户组名称>
```

### 修改用户组

```shell
groupmod -g <用户组> <用户名>
```

## 用户和组的配置文件

```shell
/etc/passwd #用户配置文件 记录用户的信心

# admin:x:1000:1000::/home/admin:/bin/bash
# 用户名:<密码>:uid:groupId::<家目录>:<用户对应的shell>
```

```shell
/etc/shadow # 口令的配置文件
# xh:$6$.jpaLSwV$FbHHY80WEtQB.PmWC0OIUYErEqMRW8yo5pof5pcAahybxIKIpAGA1q0sNhPT2rQ/ajGXXgYYso1osxEIGBktg/:19238:0:99999:7:::


# xx:$6$u9PDyMSZ$2naZywjtQyVQvUtawf/2OaBYEp6zcEN5BfGYES.B/pPGBrOxSZDSTPk5X4H6tLhqEFcF7gnCvspaUEvjw.HXy1:19238:0:99999:7:::
```

```shell
/ect/group # 组配置文件

# 组名:口令(加密):组的标识符(id):
```

## 指令

系统的运行级别配置文件: `/etc/inittab`
切换到指定运行级别的指令:

```shell
init [0 | 1 | 2 | 3 | 5 | 6]
```

> 运行级别
> 0 关机
> 1 单用户（找回丢失密码）
> 2 多用户状态没有网络服务
> 3 多用户状态有网络服务
> 4 系统未使用保留给用户
> 5 图形界面
> 6 系统重启

### 帮助指令

```shell
# man [命令或配置文件] # 获得帮助信息

man ls

man help
```

```shell
help [命令] # 获取shell 内置的命令的信息
```

### 文件目录指令

```shell
pwd #显示当前工作目录的绝对路径
```

#### ls 指令

```shell
ls [选项] [目录或者文件]

# -a 显示当前目录所有文件和目录 包括隐藏的
# -l 以列表的方式显示信息

ls -l
ls -a
ls -la
```

#### cd 指令

```shell
cd
cd ~
# 都是回到家目录下面
```

#### mkdir 指令

```shell
mkdir /home/tmpFile # 在home目录下面创建 tmpFile 目录

mkdir -p /home/animal/snake # -p 可以创建多级目录

# mkdir -p /home/animal/snake 在 home的下创建了多级目录 animal/snake
```

> 删除目录:

```shell
rmdir /home/animal/snake #rmdir 删除的是空目录 如果目录下面有内容 是无法删除的

# 强删除
rm -rf ./[文件目录/文件名]
```

#### touch 指令

```shell
touch [文件名] [文件名] ... # 可以一次性创建多个文件
```

#### cp 指令
