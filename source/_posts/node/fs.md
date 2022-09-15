---
title: fs Modules
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

-   fs.dir：操作目录的子模块，提供`dir.read`、`dir.readSync`等 API 来读取目录信息。
-   fs.createReadStream()：创建一个读文件流对象。
-   fs.createWriteSteam()：创建一个写文件流对象。
-   fs.stat、fs.statSync()()：读取文件信息，包括文件状态、权限、创建时间、修改时间等等信息。
-   fs.appendFile()、fs.appendFileSync()：追加内容到文件
-   fs.chmod()、fs.chown()：改变文件权限、权限组。
-   fs.copyFile()、fs.copyFileSync()：拷贝文件。
-   fs.mkdir()、fs.mkdirSync()：创建目录。
-   fs.rename()、fs.renameSync()：修改文件名。
-   fs.rmdir()、fs.rmdirSync()：删除目录。
-   fs.unlink()、fs.unlinkSync()：删除文件。
-   fs.watchFile()：这是用来监听文件内容变化的 API。
-   fs.writeFile()、fs.writeFileSync()：写入文件。