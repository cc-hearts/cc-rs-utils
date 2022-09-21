---
title:v-on的基本使用
categories: Vue
---

## on 监听多个方法

```Vue
  <button
    type="default"
    style="
      background: #ffffff;
      color: black;
      border-radius: 10rpx;
      border: 1rpx black solid;
    "
    @click="
      resetTaskFilterData()
      getData(true)
      handleClosePopup('popupProcessed')
    "
  >
    重置
  </button>
```
