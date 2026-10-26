# [![Documentation](https://docs.rs/v_escape_codegen/badge.svg)](https://docs.rs/v_escape_codegen/) [![Latest version](https://img.shields.io/crates/v/v_escape_codegen.svg)](https://crates.io/crates/v_escape_codegen) [![codecov](https://codecov.io/github/zzau13/v_escape/graph/badge.svg?token=AFus7ytGT9)](https://codecov.io/github/zzau13/v_escape)

# v_escape_codegen

The simd optimized escape code

## XML text escaping

`v_xmlescape` provides SIMD-accelerated XML text escaping. It replaces `&`, `<`,
and `>` with XML entities while preserving quotes, slashes, and other characters.
It exposes the same `escape_string`, `escape_fmt`, and `escape_bytes` APIs as
`v_htmlescape`.

```rust
use v_xmlescape::escape_fmt;

assert_eq!(escape_fmt("a\"b/c & <tag>").to_string(), "a\"b/c &amp; &lt;tag&gt;");
```

## Documentation

- Minimum supported Rust version: 1.93.1 or later

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
