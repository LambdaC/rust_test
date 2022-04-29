# rust编译成asm
cargo rustc -- --emit asm
cargo rustc --release -- --emit asm  //优化版

# 查看编译支持的架构
rustc --print target-list

# 添加编译架构支持
rustup target add riscv64gc-unknown-none-elf


# 因为编译成的是无操作系统支持的asm，所以需要修改一下main.rs
#![no_std]
#![no_main]
#[panic_handler]

# 编译指定架构
cargo rustc -- --emit asm --target riscv64gc-unknown-none-elf
cargo rustc --release -- --emit asm --target riscv64gc-unknown-none-elf




