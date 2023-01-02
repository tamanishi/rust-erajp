# rust_erajp

Rust library for japanese era.  
This is a sample inspired from [go-erajp](https://github.com/mattn/go-erajp).

## Usage

```rust
println!("{}", rust_erajp::to_era(2023));
```

## License

MIT

## Todo
- 寛永への改元日(1624/2/30)は何とかなるのか
    - [日本では明治6年にグレゴリオ暦が採用された](https://www1.kaiho.mlit.go.jp/KOHO/faq/reki/shinreki.html)とのこと
