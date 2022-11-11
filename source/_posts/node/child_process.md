---
title: child_process
categories: Node
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
