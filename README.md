# curl vs reqwest test

## Usage

Test with reqwest

```
cargo run -- 'https://catboost.ai'
```

Test with curl

```
# Test with curl
cargo run --no-default-features --features curl -- 'https://catboost.ai'
```

Test with reqwest-impersonate

```
cargo run --no-default-features --features reqwest_impersonate -- 'https://catboost.ai'
```

## Credits

Based on https://github.com/tectonic-typesetting/tectonic
