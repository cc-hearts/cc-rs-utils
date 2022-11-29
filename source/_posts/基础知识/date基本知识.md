---
title: date基本知识
categories: Basic
---

## ISO

> ISO 标准有助于消除各种日-日惯例、文化和时区对全球业务产生的影响。它提供了一种显示日期和时间的方式，这种方式是明确定义的，对人和机器都是可以理解的

时间是参照**格林威治时间**（也就是本初子午线的时间）

```js
new Date().toISOString()
;('2022-11-05T12:17:23.944Z')
```

## UTC

> GMT 是前世界标准时，UTC 是现世界标准时。

```js
new Date().toUTCString()
;('Sat, 05 Nov 2022 12:20:48 GMT')
```

## GMT

格林威治平时（也称格林威治时间）

```js
new Date().toGMTString()
;('Mon, 05 Dec 2022 15:28:41 GMT')
```

## 参考资料
