---
title: this关键字
url: https://www.yuque.com/u21419265/bo8cge/uaieix
---

this可以用来修饰 属性 方法 构造器

this修饰属性和方法:
this理解为 当前的对象 或者当前正在创建的对象

- 在类的方法中 可以使用this.属性或者this.方法的方式去调用当前对象的属性或者方法 一般情况下省略了this 如果方法的形参和类的属性同名时候 需要显式调用this.变量的形式
- 在类的构造器中 可以使用this.属性或者this.方法的形式去调用当前对象的属性或者方法 一般情况下省略了this 如果构造器的形参和类的属性同名时候 需要显式调用this.变量的形式 <a name="xKgBG"></a>

## this调用构造器

在类的构造器中 可以显示shyingthis(形参列表)方式 调用本类中其他的构造器
构造器中不能通过this(形参列表) 方式调用自己
如果一个类中有n个构造器 则最多有n - 1 构造器使用了this(形参列表)
构造器内部最多只能声明this(形参列表) 用来调用其他的构造器

```java
 public triangle(){

    }
    public triangle (double base , double height) {
        this();
        this.base = base;
        this.height = height;
    }
```
