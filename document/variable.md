# 变量

[示例程序](../variables/src/main.rs)

## let 变量绑定

将内存对象的所有权赋予变量

### mut 可变变量

```rust
let mut x = 1;
```

## 变量解构

从复杂变量中匹配部分内容

```rust
    let (a, mut b): (bool, bool) = (true, false);
    // a = true, b = false
    b = true;
    // a = true, b = true 

```

## const 常量

始终不可变，不可被声明为`mut`
使用`const`而非`let`声明，必须声明类型

## shadowing 变量遮蔽

1. 作用域
2. `let`覆盖

## 特殊命名规范

### 忽略未使用变量

```rust
let _x = 1;
```
