---
title: pnpm 的使用
categories: Config
---


## 基本命令

`-w`  作为所有的package的公共依赖安装到根目录

```shell
pnpm install react -w
```

如果是开发依赖 添加`-D`

```shell
pnpm install react -wD
```

如果给某个模块单独安装依赖

```ts
onom add axios --filter @packages/vue2
```
