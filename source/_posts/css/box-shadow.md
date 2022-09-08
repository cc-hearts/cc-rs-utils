---
title: box-shadow 详细用法
categories: CSS
---

## 基本用法

```css
 {
      `box-shadow: ` `none` `| [``inset``? && [ <offset-x> <offset-y>
    <blur-radius>? <spread-radius>? <color>? ] ];
}
```

## 多层指定 box-shadow

```html
<div></div>
```

```css
div {
  width: 300px;
  height: 80px;
  border: 1px solid #333;
  border-bottom: none;
  box-sizing: border-box;
  box-shadow: 0 1px 1px rgb(0 0 0 / 20%), 0 7px 0 -3px #f6f6f6,
    0 8px 2px -3px rgb(0 0 0 / 20%), 0 13px 0 -6px #f6f6f6,
    0 14px 2px -6px rgb(0 0 0 / 20%);
}
```

<p class="codepen" data-height="300" data-default-tab="html,result" data-slug-hash="oNdbMaR" data-user="hearto_o" style="height: 300px; box-sizing: border-box; display: flex; align-items: center; justify-content: center; border: 2px solid; margin: 1em 0; padding: 1em;">
  <span>See the Pen <a href="https://codepen.io/hearto_o/pen/oNdbMaR">
  Untitled</a> by hearto_o (<a href="https://codepen.io/hearto_o">@hearto_o</a>)
  on <a href="https://codepen.io">CodePen</a>.</span>
</p>
<script async src="https://cpwebassets.codepen.io/assets/embed/ei.js"></script>

## 参考资料

- https://www.cnblogs.com/coco1s/p/9913885.html
