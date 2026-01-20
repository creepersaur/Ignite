# INSTALL GUIDE

1.1. Install LLVM (https://github.com/vovkos/llvm-package-windows/releases/tag/llvm-18.1.8) for your computer (for Windows) \n
1.2. Install LLVM `brew install llvm@18` (Mac) \n
2. Add it to your PATH \n
3. Run `cargo build` or `cargo check` in the project root to check for any errors \n
4. If it thorws an error on Windows: \n
- Run `rustup target add x86_64-pc-windows-msvc`, this installs MSVC toolchain \n
- Then run `cargo build --target x86_64-pc-windows-msvc` \n
