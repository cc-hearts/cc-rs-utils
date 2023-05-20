1. 预先需要安装的包
```shell
pnpm add @swc/core @swc/jest identity-obj-proxy @testing-library/dom @testing-library/jest-dom @testing-library/react @testing-library/user-event @types/testing-library__jest-dom jest jest-environment-jsdom
```
> `@types/testing-library__jest-dom`  是 `jest-dom` 的 `tds`声明
2. 生成jest的配置文件
```shell
pnpm jest --init
```

修改几项配置文件
> 路径匹配哪些文件作为test文件
```js
  testMatch: [
    '<rootDir>/src/**/__tests__/**/*.{spec,test}.{js,jsx,ts,tsx}',
    '<rootDir>/src/**/*.{spec,test}.{js,jsx,ts,tsx}',
    '<rootDir>/__test__/**/*.{spec,test}.{js,jsx,ts,tsx}',
  ],
```
> 配置文件的类型配置 如果是ts类型的单测 则将ts放在前面

```js
  moduleFileExtensions: [
    'ts', 'tsx', 'js', 'jsx'
   ],
```

> 处理静态资源的映射
```js
  moduleNameMapper: {
    '^.+\\.module\\.(css|sass|scss|less)$': 'identity-obj-proxy',
    '\\.svg$': 'identity-obj-proxy',
    '\\.(css|sass|scss|less)$': 'identity-obj-proxy'
  },
```

> 对 ts tsx 做编译转换(babel的功能 这里使用了swc代替)
```js
transform: {
  '^.+\\.(js|jsx|ts|tsx)$': ["@swc/jest"],
},
```

> 转换需要忽略的文件
```js
transformIgnorePatterns: [
  '[/\\\\]node_modules/(?!(antd)/)[/\\\\].+\\.(js|jsx|ts|tsx)$',
],
```

> 测试的环境 为 `jsdom` 后 (在version 28 之后 需要在安装` jest-environment-jsdom`
```js
testEnvironment: 'jsdom'
```

> 每次单测文件加载之后都会加载的文件 一般用来加载环境变量
> 由于react 的组件测试都需要依赖`js-dom` 因此可以将`js-dom` 放在这个文件中导入
```js
 setupFilesAfterEnv: [
  '<rootDir>/jest/setupJestDom.ts'
 ],

// ./jest/setupJestDom.ts
import '@testing-library/jest-dom'
```

> 每次执行完毕后，不自动清理单测缓存，这样执行效率会快
```js
setupFiles: [
  '@testing-library/react/dont-cleanup-after-each'
],
```


## 参考资料
[react + vite + testing-library单测环境构建](https://segmentfault.com/a/1190000041989123)
