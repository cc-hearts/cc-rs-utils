---
title: ES6新特性
categories: JavaScript
---



# Object.is

**Object.is 是 ES2015 新特性，与 === 不一样的是：它可以正确分辨 正负零 及 NaN。**

- 抽象相等比较 (==)：将执行类型转换再进行比较，特殊地：Null 与 undefined 返回 true, 任何类型与 NaN 返回 false, +0 与-0 为 true.
- 严格等于运算符（===）:不会执行类型转换，类型不一致返回 false,特殊地：NaN 与 NaN 为 false,+0 与-0 为 true.
- Object.is: 基本与全等（===）相同，特殊地：NaN 与 NaN 为 true,+0 与-0 为 false.

**在除法中，1/+0 为 +Infinity, 1/-0 为 -Infinity，而 +Infinity === -Infinity 返回 false.**
