---
title: transform渲染影响
categories: CSS
---

## transform限制position:fixed的跟随效果

如果 父元素使用了 transform 而子元素使用了fixed 则 会被父元素限制 降级成
`position:absolute` 的效果

参考资料:

- <https://www.zhangxinxu.com/wordpress/2015/05/css3-transform-affect/>
