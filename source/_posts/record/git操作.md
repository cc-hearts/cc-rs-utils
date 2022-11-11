---
title: git操作
url: https://www.yuque.com/u21419265/dlfa41/fvmlp4
---

git branch -d local

git push origin --delete remoteBranchName

git push origin 本地分支名:远程分支名

<https://www.runoob.com/git/git-tutorial.html>

1、查看本地属性

| git config --local --list |
| --- |

2、修改全局的用户名和邮箱

| git config  --global user.name 你的目标用户名； |
| --- |

git config  --global user.email 你的目标邮箱名
3、修改当前project的用户名和邮箱

| git config user.name 你的目标用户名; |
| --- |

git config user.email 你的目标邮箱名;
4、也可以直接修改.gitconfig文件

| vi ~/.gitconfig;   然后在文件中直接修改即可. |
| --- |

<a name="nqNEq"></a>

# 创建一个分支

git branch <分支名>

查看分支是否创建成功

git branch  查看本地分支

git branch -a 查看所有的分支

git branch -d 分支名 删除本地分支

git remote -v 查看远程仓库地址

**git fetch origin main (main为远程分支名字） 拉取远程分支** <a name="GNEbe"></a>

# 切换分支

git checkout <切换的分支名>

<a name="csmo0"></a>

# 合并分支内容

git merge <需要合并的分支(是将其他分支合并到你当前的分支)>

合并后的内容有问题，你可以通过撤销合并恢复到以前状态
git reset --hard HEAD
代码已经提交，撤销的方法是
git reset --hard ORIG\_HEAD

<a name="Ocvrp"></a>

# 记Git报错-refusing to merge unrelated histories

> 拒绝合并不相关的历史
> git branch -b 创建的远程分支

解决

```shell
git merge <branch> --allow-unrelated-history // 该选项可以合并两个独立启动仓库的历史
```

```git

新建本地分支指定远程分支，该命令可以将远程git仓库里的指定分支拉取到本地
git checkout -b 本地分支名 origin/远程分支名
 
//获取远程
git fetch origin
 
//查看远程分支
git branch -r
//创建本地分支并关联
git checkout -b 本地分支 origin/远程分支
 
//已有本地分支创建关联
git branch --set-upstream-to origin/远程分支名  本地分支名
//拉取
```

> <https://blog.csdn.net/qq_32615575/article/details/108361235>

1.查看当前远程仓库路径
[git](https://so.csdn.net/so/search?q=git\&spm=1001.2101.3001.7020) remote -v
2.设置新的远程仓库路径
git [remote](https://so.csdn.net/so/search?q=remote\&spm=1001.2101.3001.7020) set-url origin [git@](mailto:git@git.taient.com:web/tie-frontend.git)xxx.git

git branch -vv

git remote rm origin 删除远程分支

git remote add origin // 添加远程分支

\-------
git push origin 本地分支名:远程分支名

<a name="EH2Ea"></a>

## 回滚

git reset --hard commit:id
git push -f 强制推送

// 删除本地分支 git branch -d localBranchName // 删除远程分支 git push origin --delete remoteBranchName
