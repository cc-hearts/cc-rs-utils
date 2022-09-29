---
title: useState
---

## useState 的变化

一个 `useState`会返回一个 callback 但是同一个`useState`每次返回的 callback 的值都是同一个

```ts
import { useState, useRef } from 'react'
export default function App() {
  const [s, setS] = useState(1)
  const refs = useRef<{ count: number; setCount: ((...rest: any[]) => void) | null }>({ count: -1, setCount: null })
  if (refs.current.count === -1) {
    console.log('执行次数')
    refs.current.count = s
    refs.current.setCount = setS
  }
  console.log(refs.current.setCount === setS)
  return (
    <React.Fragment>
      <button onClick={() => setS((state) => state + 1)}>click change state</button>
    </React.Fragment>
  )
}
```
