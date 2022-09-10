---
title: fs模块常用功能
categories: Node
---

## 常用 API

### 判断文件是否存在

```js
if (fs.existsSync(fileName)) {
  console.log('file exist')
}
```

### 同步写入

```js
// 同步写入api

// https://nodejs.org/api/fs.html#fswritefilesyncfile-data-options

fs.writeFileSync(`./${fileName}`, `# ${getTodayFormatDate()} 每日计划`, {
  encoding: 'utf-8',
})
```
