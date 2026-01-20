# INSTALL GUIDE
1.1. Install LLVM (https://github.com/vovkos/llvm-package-windows/releases/tag/llvm-18.1.8) for your computer (for Windows)
1.2. Install LLVM `brew install llvm@18` (Mac)
2. Add it to your PATH
3. Run `cargo build` or `cargo check` in the project root to check for any errors
4. If it thorws an error on Windows:
- Run `rustup target add x86_64-pc-windows-msvc`, this installs MSVC toolchain
- Then run `cargo build --target x86_64-pc-windows-msvc`
