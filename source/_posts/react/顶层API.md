---
title: React 顶层API的使用
categories: React
---

## 顶层 API 的使用

```tsx
import React from 'react'

function App() {
  const render = (
    <div className="app">
      {/* 普通dom类型 */}
      <div>
        <div>123123123</div>
      </div>
      {/* 文本节点类型 */}
      文本节点类型
      {/* Fragment 类型 */}
      <React.Fragment>
        <div>fragment</div>
      </React.Fragment>
      {/* 数组节点类型 */}
      {[1, 2, 3].map((val) => (
        <div key={val}>{val}</div>
      ))}
    </div>
  )

  // 获取所有的子节点

  const { children } = render.props

  console.log(children)

  // 扁平化 扁平化第一层的数组

  let childrenList = React.Children.toArray(children)

  console.log(childrenList)

  // 除去文本节点类型

  const filterKeyList: React.ReactElement[] = []

  childrenList.forEach((node) => {
    // 是否是react类型 如果是的话 返回

    if (React.isValidElement(node)) filterKeyList.push(node)
  })

  console.log(filterKeyList)

  // 创建新的react元素

  const newNode = React.createElement(
    'div',

    { key: 'unique' },

    'stu react example'
  )

  // 修改元素

  console.log([...filterKeyList, newNode])

  // https://zh-hans.reactjs.org/docs/react-api.html#cloneelement

  return React.cloneElement(render, {}, [...filterKeyList, newNode])

  // return render
}

export default App
```

## react 自动引入

- @babel/plugin-syntax-jsx
  - @babel/plugin-syntax-jsx ： 使用这个插件，能够让 Babel 有效的解析 JSX 语法。
- @babel/plugin-transform-react-jsx - @babel/plugin-transform-react-jsx ：这个插件内部调用了 @babel/plugin-syntax-jsx，可以把 React JSX 转化成 JS 能够识别的 createElement 格式。
  > 需要在.babelrc 中设置  runtime: automatic

```js
"presets": [ ["@babel/preset-react",{ "runtime": "automatic" }] ],
```
