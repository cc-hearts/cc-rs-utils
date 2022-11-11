---
title: offset、client、scroll、Page
categories: JavaScript
---



# offsetHeight、offsetWidth

**只读属性**
**HTMLElement.offsetHeight** 是一个只读属性，它返回该元素的像素高度

> 高度包含该元素的垂直内边距和边框，且是一个整数。
> 如果元素被隐藏（例如 元素或者元素的祖先之一的元素的style.display被设置为none），则返回0

```javascript

document.getElementById("app").offsetHeight // 190
document.getElementById("app").offsetWidth // 190

```

```css
#app {
  width: 150px;
  height: 150px;
  background-color: red;
  border: 20px solid #fff;
}
```



# clientHeight、clientWidth

**只读属性**
元素内部的高度(单位像素)，包含内边距，但不包括水平滚动条、边框和外边距。

> clientHeight 可以通过 CSS height + CSS padding - 水平滚动条高度 (如果存在)来计算.

```javascript
document.getElementById("app").clientHeight // 150
document.getElementById("app").clientWidth // 150
```



# scrollHeight、scrollWidth

scrollHeight 的值等于该元素在不使用滚动条的情况下为了适应视口中所用内容所需的最小高度。** 没有垂直滚动条的情况下，scrollHeight值与元素视图填充所有内容所需要的最小值clientHeight相同**。**包括元素的padding，但不包括元素的border和margin。**scrollHeight也包括 ::before 和 ::after这样的伪元素。

# offsetTop、offsetLeft

**HTMLElement.offsetTop** 为只读属性，它返回当前元素相对于其 offsetParent元素的顶部内边距的距离。

> 如果当前元素设置了position:fixed  则 当前元素的offsetParent 为 null 如果设置为style.display: none;  当前元素的offsetParent 也为 null

```javascript
const box = document.getElementById("box");
console.log(box.offsetParent); // null
```

```css
#box {
  width: 70px;
  height: 70px;
  background-color: gray;
  margin-top: 15px;
  overflow: hidden;
  position: fixed;
}
```

***

```javascript
const app = document.getElementById("app");
const box = document.getElementById("box");
console.log(app.offsetTop); //8px
console.log(box.offsetTop); //43 app的border 20px box margin 15px body 自带的padding 8px
```

```css
#app {
  width: 150px;
  height: 750px;
  background-color: red;
  border: 20px solid #fff;
}
#box {
  width: 70px;
  height: 70px;
  background-color: gray;
  margin-top: 15px;
  overflow: hidden;
}
```

```html
   <div id="app">
      <div id="box"></div>
    </div>
```

如果给父元素设计position

> 父元素设计的position 为 relative absolute fixed sticky 则子元素的offsetParent则是当前的父元素



# scrollTop、scrollLeft

在Chrome中 scroll是html的 可以通过 document.documentElement 获取html

> 当一个元素的内容没有产生垂直方向的滚动条，那么它的 scrollTop 值为0

```javascript
document.getElementById("btn").onclick = function () {
  console.log(document.documentElement.scrollHeight);
  console.log(document.documentElement.scrollTop);
  console.log(document.documentElement.clientHeight);
};
```

```css
#app {
  width: 150px;
  height: 2500px;
  background-color: red;
}
#btn {
  width: 40px;
  height: 20px;
  cursor: pointer;
  position: fixed;
  text-align: center;
  top: 0;
  left: 50%;
  background-color: indianred;
}
```

```html
    <div id="app"></div>
    <div id="btn">btn</div>
```

> 当scrollHeight -scrollTop == clientHeight时候 垂直滚动条滚到底部
> 当scrollWidth -scrollLeft == clientWidth时候 水平条滚到右侧



# pageX，pageY

pageX/Y：相对于文档边缘,包含滚动条距离
clientX/Y：相对于当前页面且不包含滚动条距离
