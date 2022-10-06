---
title: 守卫
Categories: Nest
---

守卫其实也是中间件其中的一种，只是在Nestjs中对中间件更加具体的划分，将不同职能的部件给了不同的称呼，一般守卫是用来做登录鉴权使用，权限类的

## jwt 和 token + redis的区别

- jwt 是将用户信息加密到一个字符串中 服务端可以解密
- token+redis方案
  I.使用一些算法生成一个token当做然ey来存储到redis中，值为当前用户信息（用户id,用户名等）
  2.这里简单使用uuid生成token
  3.根据前端传递的token(uuid)从redis中读取数据，获取用户信息

> token + redis 可以跨语言使用

## UseGuards(class)

> 局部守卫 可以添加到控制器上 也可以添加到指定的方法上

```ts
import { Injectable, UseGuards } from '@nestjs/common';
import { AuthGuard } from 'src/guard/auth.guard';
import { RedisService } from 'src/redis/redis.service';

@UseGuards(AuthGuard)
@Injectable()
export class LoginService {
  constructor(private readonly redisService: RedisService) {}
  async login(token: string) {
    const userInfo = await this.redisService.get(token);
    // 没有值
    return userInfo;
  }
}
```

全局守卫 `app.useGlobalGuards(new calss())`

```ts
 const redisService = app.get<RedisService>(RedisService);
  app.useGlobalGuards(new AuthGuard(redisService));
```

还可以在module中使用

```ts
// ...
  controllers: [AppController],
  providers: [
    AppService,
    {
      provide: APP_GUARD,
      useClass: AuthGuard,
    },
  ],
```
