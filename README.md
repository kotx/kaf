# Kaf

[![Crates.io](https://img.shields.io/crates/v/kaf?style=flat-square)](https://crates.io/crates/kaf)
[![docs.rs](https://img.shields.io/docsrs/kaf?style=flat-square)](https://docs.rs/kaf)
![Crates.io](https://img.shields.io/crates/d/kaf?style=flat-square)

Cursed and pretty logging with `Fn` filters

```rust
kaf::with_filter(
    Box::new(|target, _level| {
        (target == "mycrate" || target.starts_with("mycrate::"))
    }),
    log::LevelFilter::Info,
);
```

## FAQ

**This is unoptimized and slow!**

Please PR a fix or open an issue, I don't know anything about optimization.

**What the heck is *X*?**

I don't know either, but please open an issue.
