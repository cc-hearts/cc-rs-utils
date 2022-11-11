---
title: cross-env
categories: Record
---

> 运行跨平台设置和使用环境变量(Node中的环境变量)的脚本

在自定义配置环境变量的时候，由于在不同的环境下，配置方式也是不同的。

> 当我们使用 NODE\_ENV = production 来设置环境变量的时候，大多数windows命令会提示将会阻塞或者异常，或者，windows不支持NODE\_ENV=development的这样的设置方式，会报错。因此 cross-env 出现了。我们就可以使用 cross-env命令，这样我们就不必担心平台设置或使用环境变量了。也就是说 cross-env 能够提供一个设置环境变量的scripts，这样我们就能够以unix方式设置环境变量，然而在windows上也能够兼容的

package.json中：

```json
    "start": "cross-env process.env.NODE_ENV=development webpack-dev-server  --config development.config.js",
    "build": "cross-env NODE_ENV=production  webpack --config production.config.js",
    "test": "cross-env NODE_ENV=test jest"
```
