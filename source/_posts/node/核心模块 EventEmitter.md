---
title: 核心模块 EventEmitter
url: https://www.yuque.com/heart-ma72y/ve7v1k/gi2p3n
---

node 中所有的异步I/O 操作都会在完成的时候发送一个事件队列

```javascript
const { EventEmitter } = require('events')

const events = new EventEmitter();

events.on('connection', function() {
  console.log('connection events invoke');
})


events.emit('connection', 123)
```

大多数时候我们不会直接使用 EventEmitter，而是在对象中继承它
