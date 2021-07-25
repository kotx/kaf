# Kaf

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
