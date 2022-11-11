---
title: nvm安装
categories: Record
---



# mac版本

```shell
curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.8/install.sh | bash
```

之后在

```shell
vi ~/.zshrc

加入
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion
```

1,nvm nvm list 是查找本电脑上所有的node版本

- nvm list 查看已经安装的版本
- nvm list installed 查看已经安装的版本
- nvm list available 查看网络可以安装的版本

2,nvm install 安装最新版本nvm

3,nvm use  ## 切换使用指定的版本node

4,nvm ls 列出所有版本

5,nvm current显示当前版本

6,nvm alias   ## 给不同的版本号添加别名

7,nvm unalias  ## 删除已定义的别名

8,nvm reinstall-packages  ## 在当前版本node环境下，重新全局安装指定版本号的npm包

9,nvm on 打开nodejs控制

10,nvm off 关闭nodejs控制

11,nvm proxy 查看设置与代理

12,nvm node\_mirror \[url] 设置或者查看setting.txt中的node\_mirror，如果不设置的默认是 <https://nodejs.org/dist/>
　　nvm npm\_mirror \[url] 设置或者查看setting.txt中的npm\_mirror,如果不设置的话默认的是： <https://github.com/npm/npm/archive/>.

13,nvm uninstall  卸载制定的版本

14,nvm use \[version] \[arch] 切换制定的node版本和位数

15,nvm root \[path] 设置和查看root路径

16,nvm version 查看当前的版本

## 解决nvm每次启动终端都要设置nvm use

终端运行 nvm alias default stable   default 是一个默认的别名，stable也是默认的别名 把nvm 启动后默认的版本号变为了指向stable的版本号     可以通过 nvm ls ，来看这些别名指向的版本号...

Vscode安装quokka找不到node 在setting.json中指定

```json
"quokka.node":"/Users/xchen/.nvm/versions/node/v16.13.0/bin/node" // which node
```



# node yarn 查看全局安装的包

```javascript
npm list -g --depth=0

yarn global list --depth=0
```

> nvm安装成功后 将nvm的全局的安装的包要配置环境变量 这样全局的命令才可以用



# nvm 低版本不能安装问题

<https://blog.csdn.net/cydlzw/article/details/114163312>
