---
title: 自定义属性 data-*
---

## 概述

**data-*** [全局属性](https://developer.mozilla.org/zh-CN/docs/Web/HTML/Global_attributes) 是一类被称为**自定义数据属性**的属性，它赋予我们在所有 HTML 元素上嵌入自定义数据属性的能力，并可以通过脚本在 [HTML](https://developer.mozilla.org/zh-CN/docs/Web/HTML) 与 [DOM](https://developer.mozilla.org/zh-CN/docs/Web/API/Document_Object_Model) 表现之间进行专有数据的交换

通过js获取 DOM元素上的自定义属性：

```js
 // <div id="app" data-value="app">
 const app = document.getElementById('app')
 // 获取DOM节点上的自定义属性
 console.log(app.dataset)
```

## 参考资料

- <https://developer.mozilla.org/zh-CN/docs/Web/HTML/Global_attributes/data>-*
