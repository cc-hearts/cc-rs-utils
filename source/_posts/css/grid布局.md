---
title: grid布局基本使用指南
categories: CSS
---

## 基本属性介绍

```html
<div class="grid">
  <div class="container">1</div>
  <div class="container">2</div>
  <div class="container">3</div>
  <div class="container">4</div>
  <div class="container">5</div>
</div>
```

```css
.grid {
  display: grid;
}

.container {
  height: 50px;
  border: 1px solid #eee;
}
```

```css
.grid {
  display: grid;
  /* grid-template-columns: 100px 100px 100px; */
  /* fr 单位的灵活网格 */
  /* 定义每一列的宽度*/
  grid-template-columns: 100px 100px 100px;
  /* 定义每一个行的高度*/
  grid-template-rows: repeat(3, 100px);

  /* 定义行列之间的间隔*/
  /* 各个行之间的空隙 */
  row-gap: 20px;
  /* 各个列之间的空间 */
  column-gap: 20px;
  /* 简介写法：*/
  /* gap: 20px */
}
```

```css
/**  列开始位置 但是不会超过该列 */
.container:nth-of-type(1) {
  grid-column-start: 2;
}
```

如果 `grid-column-start` 的 列超过了`grid-template-columns` 定义的列的宽度 则会自动分配剩余空间的宽度进行计算

```css
.container:nth-of-type(1) {
  grid-column-start: 9;
}
```

<p class="codepen" data-height="300" data-default-tab="html,result" data-slug-hash="eYrNgBP" data-user="hearto_o" style="height: 300px; box-sizing: border-box; display: flex; align-items: center; justify-content: center; border: 2px solid; margin: 1em 0; padding: 1em;">
  <span>See the Pen <a href="https://codepen.io/hearto_o/pen/eYrNgBP">
  Untitled</a> by hearto_o (<a href="https://codepen.io/hearto_o">@hearto_o</a>)
  on <a href="https://codepen.io">CodePen</a>.</span>
</p>
<script async src="https://cpwebassets.codepen.io/assets/embed/ei.js"></script>

## 自动使用多列填充

> 关键`repeat(auto-fill, minmax(200px, 1fr));`

```css
.container {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  grid-auto-rows: minmax(100px, auto);
  grid-gap: 20px;
}
```

## 参考资料

> grid 布局在线网址: https://grid.layoutit.com/

- https://www.ruanyifeng.com/blog/2019/03/grid-layout-tutorial.html
- https://developer.mozilla.org/zh-CN/docs/Web/CSS/CSS_Grid_Layout
