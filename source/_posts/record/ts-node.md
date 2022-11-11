---
title: ts-node
categories: Record
---

ts-node 需要指定`tsconfig`需要使用的指令是`-P`

```javascript
 "deploy": "ts-node -P tsconfig.build.json ./scripts/ci.build.ts"
```
