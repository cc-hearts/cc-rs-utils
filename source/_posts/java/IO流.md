---
title: IO流
url: https://www.yuque.com/u21419265/bo8cge/pr43g1
---

1. public File(String pathname) 以pathname为路径的建File对象，可以是绝对路径或者相对路径，如果 pathname是相对路径，则默认的当前路径在系统属性user.dir中存储。
   1. 绝对路径：是一个固定的路径,从盘符开始
   2. 相对路径：是相对于某个位置开始
2. public File(String parent,String child)

以parent为父路径，child为子路径创建File对象。

3. public File(File parent,String child)

根据一个父File对象和子文件路径创建File对象

![image.png](../../assets/java/pr43g1/1658661390343-56164763-033d-42de-9ef2-950f2ad030ec.png) <a name="upI5c"></a>

### 分类

流向: 输入流 输出流
数据单位 字节流 字符流
流的角色 节点流 处理流
