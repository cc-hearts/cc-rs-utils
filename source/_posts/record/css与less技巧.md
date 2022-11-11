---
title: css与less技巧
url: https://www.yuque.com/u21419265/dlfa41/kqlleq
---

父元素用display:flex 子元素可以使用margin:auto居中
[点击查看【codepen】](https://codepen.io/xiaochen2001/embed/yLzpqGb)

# clip-path的使用技巧



## Inset

inset( <shape-arg>{1,4} \[round <border-radius>]? )
shape-arg 分别为矩形的上右下左顶点到被剪裁元素边缘的距离（和margin、padding参数类似），border-radius 为可选参数，用于定义 border 的圆角。



## circle

circle() 用于定义一个圆。
参数类型：circle( \[<shape-radius>]? \[at <position>]? )
其中 shape-radius 为圆形的半径，position 为圆心的位置。
如果 shape-radius 为百分比，则 100% 相当于：

```less
sqrt(width^2+height^2)/sqrt(2)
```

DEMO:



## Ellipse

ellipse() 用于定义一个椭圆。
参数类型：ellipse( \[<shape-radius>{2}]? \[at <position>]? )
其中 shape-radius 为椭圆x、y轴的半径，position 为椭圆中心的位置。



## polygon

polygon() 用于定义一个多边形。

> shape-arg 分别为矩形的上右下左顶点到被剪裁元素边缘的距离（和margin、padding参数类似）

参数类型：polygon( \[<fill-rule>,]? \[<shape-arg> <shape-arg>]# )
其中 fill-rule 为填充规则，即通过一系列点去定义多边形的边界。
[点击查看【codepen】](https://codepen.io/xiaochen2001/embed/porYVOv)
svg暂时没学 不怎么会
[svg cli-path](https://segmentfault.com/a/1190000023301221)

# Grid布局

> grid布局在线工具

默认情况下 容器元素都是块级元素，但也可以设成行内元素。

```less
div {
  display: inline-grid; //display:grid;
}
```

![image.png]\(../../assets/record/kqlleq/1640702624417-9283f495-27f7-4c3a-b0a4-dec0de293ccc.png) grid容器变成了一个行内块元素

> 注意，设为网格布局以后，容器子元素（项目）的float、display: inline-block、display: table-cell、vertical-align和column-*等设置都将失效。

grid-template-columns属性定义每一列的列宽，
grid-template-rows属性定义每一行的行高。

```less
grid-template-columns: 100px 100px 100px;
grid-template-rows: 100px 100px 100px;

  grid-template-columns: 33.33% 33.33% 33.33%;
  grid-template-rows: 33.33% 33.33% 33.33%;
```



## repeat()

repeat()接受两个参数，第一个参数是重复的次数（上例是3），第二个参数是所要重复的值。

```less
grid-template-columns:repeat(3,33.3%);//
```

repeat()重复某种模式也是可以的。

> 重复n列为100px n+1列20px n+2列80px

![image.png]\(../../assets/record/kqlleq/1640703467691-904e7bc7-e60d-4a57-928c-eb7e2f194b44.png)



## auto-fill 关键字

单元格的大小是固定的，但是容器的大小不确定。
如果希望每一行（或每一列）容纳尽可能多的单元格，这时可以使用auto-fill关键字表示自动填充。

> 为什么对 grid-template-rows指定一列 后续列无效

[点击查看【codepen】](https://codepen.io/xiaochen2001/embed/YzrYdRw)

## fr关键字

为了方便表示比例关系，网格布局提供了fr关键字（fraction 的缩写，意为"片段"）。如果两列的宽度分别为1fr和2fr， 则表示后者的宽度是前者的两倍

> fr可以与绝对长度的单位结合使用

```less
// 第一列的宽度为150像素，第二列的宽度是第三列的一半
.container {
  display: grid;
  grid-template-columns: 150px 1fr 2fr;
}
```



## minmax()

minmax()函数产生一个长度范围，表示长度就在这个范围之中。它接受两个参数，分别为最小值和最大值。

## auto

auto关键字表示由浏览器自己决定长度。

> 第二列的宽度，基本上等于该列单元格的最大宽度，除非单元格内容设置了min-width，且这个值大于最大宽度。

![image.png]\(../../assets/record/kqlleq/1640705240141-3fd4d0a6-afca-4a60-8823-c4c5f4367d7a.png)

## gap



### row-gap 属性，&#xA;column-gap 属性，&#xA;gap 属性

row-gap属性设置行与行的间隔（行间距），column-gap属性设置列与列的间隔（列间距）。
gap属性是column-gap和row-gap的合并简写形式

> gap省略了第二个值，浏览器认为第二个值等于第一个值

```less
gap: <row-gap> <column-gap>;


gap: 20px 20px;
```

![image.png]\(../../assets/record/kqlleq/1640705631159-133e0a23-702a-42b2-9326-468327201d0d.png)



## grid-template-areas

指定区域

## grid-auto-flow 属性

划分网格以后，容器的子元素会按照顺序，自动放置在每一个网格。默认的放置顺序是"先行后列"，即先填满第一行，再开始放入第二行。
grid-auto-flow属性决定，默认值是row，即"先行后列"。也可以将它设成column，变成"先列后行"

> column dense 表示先列后行 并且尽可能填满 不出现空格

> row dense，表示"先行后列"，并且尽可能紧密填满，尽量不出现空格

![image.png]\(../../assets/record/kqlleq/1640707578263-eb33fc15-963e-405e-801d-42ba376a57aa.png)
此时的单元格是被空出来的
使用column dense 可以尽可能填充空格
![image.png]\(../../assets/record/kqlleq/1640707623862-363ceecb-d80e-47dd-8744-28e2becfd482.png)

## 单元格内容属性



### justify-items 属性，&#xA;align-items 属性，&#xA;place-items 属性

> 设置了justify-items设置了之后  宽度会变为单元格内容(也就是各个.items的内容宽度)
> 设置了align-items 之后 高度会变为单元格内容(也就是各个.items的内容高度)

```less
justify-items: start | end | center | stretch;
align-items: start | end | center | stretch;


start：对齐单元格的起始边缘。
end：对齐单元格的结束边缘。
center：单元格内部居中。
stretch：拉伸，占满单元格的整个宽度（默认值）。
```

```less
 justify-items:center;
  align-items:center;
```

![image.png]\(../../assets/record/kqlleq/1640708457292-11886a4e-a9ec-450f-94f5-ba195ee15156.png)
place-items属性是align-items属性和justify-items属性的合并简写形式。

```less
place-items: <align-items> <justify-items>;
```

如果省略第二个值，则浏览器认为与第一个值相等。



### justify-content 属性，&#xA;align-content 属性，&#xA;place-content 属性

justify-content属性是整个内容区域在容器里面的水平位置（左中右），
align-content属性是整个内容区域的垂直位置（上中下）。

```less
 justify-content: start | end | center | stretch | space-around | space-between | space-evenly;
 align-content: start | end | center | stretch | space-around | space-between | space-evenly;

start - 对齐容器的起始边框
stretch - 项目大小没有指定时，拉伸占据整个网格容器。
space-around - 每个项目两侧的间隔相等。所以，项目之间的间隔比项目与容器边框的间隔大一倍。
space-between - 项目与项目的间隔相等，项目与容器边框之间没有间隔
space-evenly - 项目与项目的间隔相等，项目与容器边框之间也是同样长度的间隔
```

![image.png]\(../../assets/record/kqlleq/1640708942826-9d063c8d-afec-455a-b43b-5d7a0c0a147c.png)place-content属性是align-content属性和justify-content属性的合并简写形式。

```less
 place-content: <align-content> <justify-content>
```



### grid-auto-columns 属性，&#xA;grid-auto-rows 属性

有时候，一些项目的指定位置，在现有网格的外部。比如网格只有3列，但是某一个项目指定在第5行。这时，浏览器会自动生成多余的网格，以便放置项目。
grid-auto-columns属性和grid-auto-rows属性用来设置，浏览器自动创建的多余网格的列宽和行高。它们的写法与grid-template-columns和grid-template-rows完全相同。如果不指定这两个属性，浏览器完全根据单元格内容的大小，决定新增网格的列宽和行高。
[点击查看【codepen】](https://codepen.io/xiaochen2001/embed/eYGyXNz)



### grid-template 属性，&#xA;grid 属性

grid-template属性是grid-template-columns、grid-template-rows和grid-template-areas这三个属性的合并简写形式。
grid属性是grid-template-rows、grid-template-columns、grid-template-areas、 grid-auto-rows、grid-auto-columns、grid-auto-flow这六个属性的合并简写形式



### grid-column-start 属性，&#xA;grid-column-end 属性，&#xA;grid-row-start 属性，&#xA;grid-row-end 属性

项目的位置是可以指定的，具体方法就是指定项目的四个边框，分别定位在哪根网格线。

```less
grid-column-start属性：左边框所在的垂直网格线
grid-column-end属性：右边框所在的垂直网格线
grid-row-start属性：上边框所在的水平网格线
grid-row-end属性：下边框所在的水平网格线
```

除了指定为第几个网格线，还可以指定为网格线的名字

> grid-template-areas会改变网格线名字 或者手动改变网格线名字 grid-template-column grid-template-row

```less
.item-1 {
  grid-column-start: header-start;
  grid-column-end: header-end;
}
```

这四个属性的值还可以使用span关键字，表示"跨越"，即左右边框（上下边框）之间跨越多少个网格。
![image.png]\(../../assets/record/kqlleq/1640709960646-7b327e40-560f-4f13-b4d1-5900d47f6455.png)
使用这四个属性，如果产生了项目的重叠，则使用z-index属性指定项目的重叠顺序。



### grid-column 属性，&#xA;grid-row 属性

grid-column属性是grid-column-start和grid-column-end的合并简写形式，grid-row属性是grid-row-start属性和grid-row-end的合并简写形式。


```less
 grid-column: <start-line> / <end-line>;
  grid-row: <start-line> / <end-line>;


.item1{
 grid-column: 1 / 3;
  grid-row: 1 / 2;
}
```



## grid-area 属性

grid-area属性指定项目放在哪一个区域。

```less
.item-1 {
  grid-area: e;
}

```

![image.png]\(../../assets/record/kqlleq/1640710224384-80fde9f6-3a08-4ba0-9a09-d1b28be7b001.png)

grid-area属性还可用作grid-row-start、grid-column-start、grid-row-end、grid-column-end的合并简写形式，直接指定项目的位置。

```less

.item {
  grid-area: <row-start> / <column-start> / <row-end> / <column-end>;
}
```



### justify-self 属性，&#xA;align-self 属性，&#xA;place-self 属性

justify-self属性设置单元格内容的水平位置（左中右），跟justify-items属性的用法完全一致，但只作用于单个项目。
align-self属性设置单元格内容的垂直位置（上中下），跟align-items属性的用法完全一致，也是只作用于单个项目。

```less
 justify-self: start | end | center | stretch;
  align-self: start | end | center | stretch;

start：对齐单元格的起始边缘。
end：对齐单元格的结束边缘。
center：单元格内部居中。
stretch：拉伸，占满单元格的整个宽度（默认值）。
```

![image.png]\(../../assets/record/kqlleq/1640710398018-3e6288d9-76ed-4e19-aa2a-2879ee28b734.png)
place-self属性是align-self属性和justify-self属性的合并简写形式。

```less
place-self: <align-self> <justify-self>;
```

<https://www.ruanyifeng.com/blog/2019/03/grid-layout-tutorial.html>

## grid瀑布流写法

[点击查看【codepen】](https://codepen.io/xiaochen2001/embed/gOGvYax?editors=1100)

# less技巧

```less
  @color:"color";
  @color-1:#3c5f9c;
  @color-2:#cc6aba;
  @color-3:#83c360;
  @color-4:#bf7254;
  @color-5:#ba848c;
  @color-6:#b6c960;
  @color-7:#6d3bc0;
  @color-8:#9a50c3;
#grid {
  display:grid;
  height:100vh;
  grid-template-columns:repeat(4,1fr);
  grid-template-rows:repeat(2,1fr);
  & > div {
    position:relative;
     .contents(8,1);
  }
}
.contents(@n,@i) when (@i<=@n) {
  &:nth-child(@{i}) {
    background-color:e("@{@{color}-@{i}}");
  }
 &:nth-child(@{i})::after {
   content:"@{i}";
   position:absolute;
   top:50%;
   left:50%;
   transform:translate(-50%,-50%)
  }
  .contents(@n,@i + 1)
}
```



# flex瀑布流写法

[点击查看【codepen】](https://codepen.io/xiaochen2001/embed/PoJQoLY?editors=0100)

> less文档



# :not()

[CSS](https://developer.mozilla.org/zh-CN/docs/Web/CSS) [伪类](https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-classes) **:not()** 用来匹配不符合一组选择器的元素。由于它的作用是防止特定的元素被选中，它也被称为\_反选伪类\_（*negation pseudo-class*）

# gradients

线性渐变（Linear Gradients）- 向下/向上/向左/向右/对角方向。
径向渐变（Radial Gradients）- 由它们的中心向四周发散。

## 线性渐变

**background-image: linear-gradient(direction, color-stop1, color-stop2, …);**
**线性渐变默认从上到下**
[点击查看【codepen】](https://codepen.io/xiaochen2001/embed/BawxKwR)
**direction:**
**to top 从下到上**
**to bottom 从上到下**
**to left 从右到左**
**to right 从左到右**
// 对角渐变
[点击查看【codepen】](https://codepen.io/xiaochen2001/embed/wvrjGmr)

## 角度渐变

> 从顶部开始 沿顺时针

[点击查看【codepen】](https://codepen.io/xiaochen2001/embed/MWEGyPz)

## 重复的线性渐变

[点击查看【codepen】](https://codepen.io/xiaochen2001/embed/MWEGyPz)

## 颜色终止位置

```less
#gradient {
  height:100px;
  // 这里的10% 77% 表示的是颜色的停止点
  // 中间的有些位置你没有明确设置，
  // 那么它将会被自动计算(例如10% - 77% 之间的停止点)
  background: linear-gradient(to right, green 10%, red 77%, yellow 77%);;
}
```

[点击查看【codepen】](https://codepen.io/xiaochen2001/embed/RwLyQaN)

## 渐变中心点

渐变会平滑地从一种颜色过渡到另一种颜色 可以设置渐变的中心点控制渐变的平滑过渡

> 10% 就是渐变的中心点 默认值是50%

[点击查看【codepen】](https://codepen.io/xiaochen2001/embed/PoJeQOb)

## 堆叠渐变

[点击查看【codepen】](https://codepen.io/xiaochen2001/embed/dyVedjo)

# radial-gradient径向渐变

```less
.radial-gradient {
    position: relative;
    width: 262px; height: 262px;
    border-radius: 50%;
    background: linear-gradient(30deg, transparent 40%, rgba(42, 41, 40, .85) 40%) no-repeat 100% 0, linear-gradient(60deg, rgba(42, 41, 40, .85) 60%, transparent 60%) no-repeat 0 100%, repeating-radial-gradient(#2a2928, #2a2928 4px, #ada9a0 5px, #2a2928 6px);
    background-size: 50% 100%, 100% 50%, 100% 100%;
}
.radial-gradient:after {
    position: absolute;
    top: 50%; left: 50%;
    margin: -35px;
    border: solid 1px #d9a388;
    width: 68px; height: 68px;
    border-radius: 50%;
    box-shadow: 0 0 0 4px #da5b33, inset 0 0 0 27px #da5b33;
    background: #b5ac9a;
    content: '';
}
```

```html
<div class="radial-gradient"></div>
```
