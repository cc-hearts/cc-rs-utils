---
title: z-index失效
url: https://www.yuque.com/u21419265/rceu41/dwpll5
---

1、一般z-index的使用是在有两个重叠的标签，在一定的情况下控制其中一个在另一个的上方出现。
2、z-index值越大就越是在上层。z-index：9999；z-index元素的position属性要是relative，absolute或是fixed。
3、z-index在一定的情况下会失效。
①、父元素position为relative时，子元素的z-index失效。
解决：父元素position改为absolute或static
②、该元素没有设置position属性为非static属性。
解决：设置该元素的position属性为relative，absolute或是fixed中的一种。
③、该标签在设置z-index的同时还设置了float浮动
解决：float去除，改为display：inline-block；
