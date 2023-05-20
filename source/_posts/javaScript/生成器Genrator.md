---
title: 生成器Genrator
categories: JavaScript
---



# 生成器函数

> 生成器函数不能当作构造函数使用

**function*** 这种声明方式(function关键字后跟一个星号）会定义一个***生成器函数* (***generator function***)**，它返回一个  [Generator](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Generator)  对象。并且它符合[可迭代协议](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Iteration_protocols#iterable)和[迭代器协议](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Iteration_protocols#iterator)。

> [生成器函数](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Statements/function*)

生成器函数可以由构造函数生成或者直接function*(){} 生成

## 由构造函数生成

生成器函数的构造器GeneratorFunction 会生成一个新的**生成器函数**对象
但是GeneratorFunction 并不是一个全局对象 需要获取

```javascript
const GeneratorFunction = Object.getPrototypeOf(function*(){}).constructor


console.log(Object.prototype.toString.call(new GeneratorFunction("a","b", "yield a + b"))); // [object GeneratorFunction]
```

由构造函数生成的效率会比function*{}的效率低
如果生成器函数中有显式的return 则最后的next带出的是return的值 且生成器对象的done 为true

```javascript
function* g() {

  yield 2
  return "1"
}
const gObj:Generator =  g(); // Generator {}


console.log(gObj.next());
console.log(gObj.next(2)); // 向生成器发送一个值 2 并且返回下一个生成器表达式的值

```

> [Generator.prototype.return()](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Generator/return) 返回给定的值并结束生成器。

```javascript
function* g() {

  yield 2
  return "1"
}
const gObj:Generator =  g(); // Generator {}

console.log(
  gObj.return("1")
); // 1
```



# yield*

或者如果用的是 [yield*](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Operators/yield*)（多了个星号），则表示将执行权移交给另一个生成器函数（当前生成器暂停执行）。



# 迭代generator 对象

```javascript
function* iterator() {
  yield 1
  yield 2
  yield 3
}
const i = iterator();
for (const iterator of i) {
    console.log(iterator);
    // 1 2 3
}
```



## 多维数组变为一维数组

```javascript
// 多维数组变为一维数组

function* isArr(arr: any[]) {
  if (Array.isArray(arr)) {
    for (const iterator of arr) {
      yield* isArr(iterator)
    }
  } else {
    yield arr
  }
}

const arr = ['a', ['b', 'c'], ['d', ['e', 'f']],];

const i = isArr(arr);

let result: any[] = [];
for (const iterator of i) {
  result = [...result, iterator]
}
console.log(result);
```
