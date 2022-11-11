---
title: 在网站中使用react
url: https://www.yuque.com/u21419265/poi9nc/aogeg7
---

test\_button.js:

```javascript
'use strict';

class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      value: 0
    }
  }
  componentDidMount() {

  }
  toggleClick(e, field) {
    this.setState({
      value: ++this.state.value
    })
  }
  render() {
    return (
      React.createElement(
        "div", {},
        React.createElement(
          "span", {},
          `${this.state.value}`
        ),
        React.createElement(
          "button", {
            onClick: (e) => this.toggleClick.bind(this, e, "click")()
          },
          `add`
        ),
      )
    )
  }
}

const not_like_button_container = document.querySelector("#not_like_button_container")
ReactDOM.render(React.createElement(App), not_like_button_container)
```

html中

```html
<!-- 加载 React。-->
  <!-- 注意: 部署时，将 "development.js" 替换为 "production.min.js"。-->
  <script src="https://unpkg.com/react@17/umd/react.development.js" crossorigin></script>
  <script src="https://unpkg.com/react-dom@17/umd/react-dom.development.js" crossorigin></script>
  <!-- 加载自己的组件 -->
 <script src="test_button.js"></script>
```



# 使用jsx在网页中编写react

1. 先引入script标签

```html
<script src="https://unpkg.com/babel-standalone@6/babel.min.js"></script>
```

现在可以在任何script标签内使用JSX 方法是在为其添加 type="text/babel" 属性。
式例:
jsx\_button.jsx

```javascript
'use strict';

class Apps extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      count: 0
    }
  }
  toggleCount() {
    this.setState({ count: ++this.state.count })
  }
  render() {
    return (
      <div>
        <span>
          {this.state.count}
        </span>
        <button onClick={(e) => this.toggleCount.bind(this, e, "click")()}>
          add
        </button>
      </div>
    )
  }
}

const box = document.querySelector('#box');

ReactDOM.render(React.createElement(Apps), box)
```

html:

```html
 <div id="box"></div>

// ...
<script src="jsx_button.jsx" type="text/babel"></script>
```



# 将jsx添加到项目中

将 JSX 添加到项目中并不需要诸如打包工具或开发服务器那样复杂的工具。本质上，添加 JSX **就像添加 CSS 预处理器一样**。
安装babel插件

```javascript
npm init -y
npm install babel-cli@6 babel-preset-react-app@3
```

**在这里使用 npm 只是用来安装 JSX 预处理器**

## 运行预处理器

```javascript
npx babel --watch src --out-dir src/dist --presets react-app/prod
```

此时会监听src下的jsx文件 并且会将他编译成js文件生成在src/dist文件中 引入的话可以直接引入dist下的js文件使用

### crossorigin

CDN 的方式引入 React，建议设置 crossorigin属性
