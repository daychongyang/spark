# Spark

## 配置开发环境

### Rust 工具链

- [rustup](https://rustup.rs/)

#### 安装 Rust

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Espressif 工具链

- [cargo-espflash](https://github.com/esp-rs/espflash/tree/main/cargo-espflash) - 上传固件到微控制器，打开串口监视器，Cargo 集成
- [espflash](https://github.com/esp-rs/espflash/tree/main/cargo-espflash) - 上传固件到微控制器，打开串口监视器
- [ldproxy](https://github.com/esp-rs/espflash/tree/main/cargo-espflash) - Espressif 构建工具链的依赖

```bash
$ cargo install cargo-espflash espflash ldproxy --locked
```

#### 安装 Espressif Rust 必要工具链

- [espup](https://github.com/esp-rs/espup)

```bash
$ cargo install espup --locked
$ espup install
```

##### espup 安装了什么？

- 安装 Espressif Rust 分支，支持 Espressif 目标
- 安装 nightly 工具链，支持 RISC-V 目标
- 安装 [LLVM 分支](https://github.com/espressif/crosstool-NG/)，支持 Xtensa 目标
- 安装 [GCC 工具链](https://github.com/espressif/llvm-project)，用于链接最终的二进制文件

### 工具链依赖项（RISC-V）

#### macOS

```bash
$ brew install llvm
```

## 运行代码

```bash
#配置环境变量
$ source ~/export-esp.sh
```

```bash
$ cargo run
```

## Reference

- [Embedded Rust on Espressif](https://narukara.github.io/std-training-zh-cn/01_intro.html)
- [The Rust on ESP Book](https://narukara.github.io/rust-on-esp-book-zh-cn/introduction.html)
