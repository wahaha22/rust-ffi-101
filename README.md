# Rust FFI 101
Rust 使用 FFI 调用 C库 libbzip2 样例.

## Steps
1. 编写 shim 代码, 将要暴露的接口暴露出来.(wrapper.h)
2. 使用 [bindgen](https://github.com/rust-lang/rust-bindgen) 生成 shim 对应的 Rust FFI 代码.(bindings.rs)
3. 链接 C/C++ 库, [build.rs](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

## Env
Linux, libbzip2

## Test
```
cargo test -- --nocapture
```

## Ref
- https://fitzgeraldnick.com/2016/12/14/using-libbindgen-in-build-rs.html
- geektime: https://time.geekbang.org/column/article/437566
