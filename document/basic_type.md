# 基本类型

[示例程序](../type/num_type/src/main.rs)

## 数据类型

1. 数值类型

- 有符号整`i8, i16, i32, i64, isize`
- 无符号整`u8, u16, u32, u64, usize`
- 浮点`f32, f64`
- 复数` num::complex::Complex::new(10, 20) `
- `size`取决于CPU64/32位 
- 在数字中可以任意插入_,仅用于视觉分隔, 如1_2_3(很好奇这有啥用)

1. 字符串

- &str

3. 布尔类型

- true, false

4. 字符类型
- 表示单个 Unicode 字符，存储为 4 个字节

5. 单元类型
- () ，其唯一的值也是 ()

## 浮点精度

float系列类型的比较运算使用`std::cmp::PartialEq`而非`std::cmp::Eq`，因为
1. 精度: 浮点精度
2. `NaN`: 根据[IEEE-754](https://wikipedia.org/wiki/IEEE_754)标准，`NaN`不满足自反性`(a == a) == true`

- 解决方法
`epsilon`
```Rust
 (0.1_f64 + 0.2 - 0.3).abs() < 0.00001
```

## 整型溢出

- `--release` 下不检测溢出，按照补码循环溢出规则处理

- 使用 `wrapping_*` 方法在所有模式下都按照补码循环溢出规则处理，例如 `wrapping_add`
- 如果使用`checked_*` 方法时发生溢出，则返回 None 值
- 使用 `overflowing_*` 方法返回该值和一个指示是否存在溢出的布尔值
- 使用 `saturating_*` 方法，可以限定计算后的结果不超过目标类型的最大值或低于最小值

## 类型推导与标注

类似C++

整型默认i32

`u8`可以使用字节赋值, 如`b'A'`

## 位运算

同C++

## Range 序列

### 连续序列

只允许字符和数字

```Rust
    for i in 1..=5 {
        println!("{}", i);
    }
```

## 复数

[示例程序](../type/complex-num/src/main.rs)

需要引入`num`库

```Rust
use num::complex::Complex;
```

需要在`Cargo.toml`中`[dependencies]` 下添加一行 `num = "0.4.0"`