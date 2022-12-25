`#![feature(backtrace, exclusive_range_pattern, arc_unwrap_or_clone)]`
でエラーが発生していたので以下のコマンドで対応

```
rustup default nightly
rustup toolchain install nightly
rustup run nightly cargo bench
```

https://stackoverflow.com/questions/53136717/errore0554-feature-may-not-be-used-on-the-stable-release-channel-couldnt
