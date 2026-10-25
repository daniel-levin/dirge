# dirge

[<img alt="crates.io" src="https://img.shields.io/crates/v/dirge.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/dirge)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-dirge?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/dirge)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/daniel-levin/dirge/rust.yml?branch=main&style=for-the-badge" height="20">](https://github.com/daniel-levin/dirge/actions)

**Strongly-typed** path types.

Enable `camino` to use UTF-8 paths throughout the API. The wrappers then store
`camino::Utf8PathBuf` and dereference to `camino::Utf8Path`; constructors accept
strings and Camino paths. They also implement `AsRef<std::path::Path>` for use
with standard filesystem APIs. Absolute path resolution returns an error if the
result is not valid UTF-8.

The optional `serde` feature works with either path backend.
