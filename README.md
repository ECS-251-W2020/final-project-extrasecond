# RITOS

For proposal please refer to [here](https://github.com/ExtraSecond/proposal).

Install suitable rust toolchain:

`bash
curl https://sh.rustup.rs -sSf             \
    |                                      \
    sh -s --                               \
    --default-toolchain nightly-2019-12-20 \
    --component rust-src llvm-tools-preview rustfmt rls rust-analysis

source $HOME/.cargo/env
cargo install cargo-xbuild cargo-binutils
`
