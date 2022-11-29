---
title: 'Git 提交规范'
categories: Config
---

- `feat` -新功能 feature
- `fix` -修复 bug
- `docs` -文档注释
- `stye` -代码格式（不影响代码运行的变动）
- `refactor` -重构、优化（既不增加新功能，也不是修复 bug)
- `perf` -性能优化
- `test` -增加测试
- `chore` -构建过程或辅助工具的变动
- `revert` -回退
- `build` -打包

## git 拉取远程分支

```shell
git checkout -b <本地分支名>  origin/<远程分支名>
```

## git 提交记修改

最后一次提交记录修改：

```shell
git commit --amend -m "修补最后一次提交记录的信息"
```

修改多次提交记录:
使用 `git rebase -i`表示进行交互式变基

> startPoint 是需要更改的节点的前一个节点 endPoint 不指定则默认为当前的节点

```shell
git rebase -i [startPoint] [endPoint]
```

之后将提交的`commit`的`pick` 改成

`rewrite` 并且修改`commit` 信息即可

## git rebase 分支合并

> 变基只适合在本地操作，如果记录已经推到了远程仓库，那么就不能再执行变基操作 否则 多人开发代码结构会混乱不堪

```shell
git checkout top # 切换到要进行变基的分支
git rebase master # 将当前的分支 变基到 master分支

# 之后 切换到 master 合并这个分支即可
git merge top # 此时的分支就会是一条线
```

## git 删除缓存

```shell
git rm -r --cached .
git add .
git commit -m 'new message'
```

## git 中断合并

```shell
git merge --abort # 在合并的时候，出现了冲突，但是还没有解决冲突，没有进行提交的时候，放弃合并
```

> 如果已经 commit：
>
> - git revert -m 1 HEAD 新建一个 commit，并且回到合并之前的状态
> - git reset --hard commit_id 回退到指定的 commit 节点
