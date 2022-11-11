---
title: child_process
url: https://www.yuque.com/heart-ma72y/ve7v1k/rhz0at
---



# exec 调用shell程序

```javascript
const childProcess = require('child_process')
// sto的值为 shell的输出
childProcess.exec('echo hello', (err, sto) => {
  if (err) {
    console.log(err)
    return
  }
  console.log(sto)
})
```
