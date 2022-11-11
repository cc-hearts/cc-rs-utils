---
title: 路由import懒加载不能识别变量解决办法
url: https://www.yuque.com/u21419265/dlfa41/nyy289
---

webpack 编译es6 动态引入 import() 时不能传入变量，例如import(dir) , 而要传入字符串 import('path/to/my/file.js')，

这是因为webpack的现在的实现方式不能实现完全动态。但一定要用变量的时候，可以通过字符串模板来提供部分信息给webpack；

例如import(`./path/${myFile}`), 这样编译时会编译所有./path下的模块，但运行时确定myFile的值才会加载，从而实现懒加载。
[
](https://blog.csdn.net/m0_48292596/article/details/116602874)
