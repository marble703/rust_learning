# cargo 编译器

## 编译运行

### run 一键编译运行

```shell
cargo run
```

### build 编译

可执行文件生成在./target/{编译类型}/(项目名)下

```shell
cargo build
```

### check 快速检查

```shell
cargo check
```

## 参数

### --vcs 版本控制

不让cargo自动生成git库

```shell
cargo new my_project --vcs=none
```

### 编译选项

--release/debug
