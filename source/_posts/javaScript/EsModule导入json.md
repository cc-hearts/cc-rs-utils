---
title: ES Module 导入 json 文件的方法
categories: JavaScript
---

1. 通过`fetch` 导入文件

```js
const data = await fetch('./src/article/data.json')
const list = await data.json() // 获取json数据
```

2. 通过 `import assert` 导入文件

```js
import data from '../article/data.json' assert { type: 'json' }
```
