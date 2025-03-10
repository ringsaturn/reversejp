# Japan Geo Reverse Lookup

A Rust library for reverse geocoding in Japan. Given a longitude and latitude,
this library returns information about the region(s) the coordinates are located
in, including code, name, and English name.

## Installation

### Rust

```bash
cargo add reversejp
```

```rust
// reversejp-rust/examples/demo.rs
use reversejp::ReverseJp;

let reverse_jp = ReverseJp::with_embedded_data().unwrap();
let props = reverse_jp.find_properties(139.7670, 35.6812);

for prop in props {
    println!("Code: {}, Name: {}, English Name: {}", prop.code, prop.name, prop.en_name);
}
```

Example output:

```text
Code: 130010, Name: 東京都, English Name: Tokyo
Code: 1310100, Name: 千代田区, English Name: Chiyoda City
```

### Python

```bash
pip install reversejp
```

```py
# reversejp-python/examples/demo.py
import reversejp

props = reversejp.find_properties(139.7670, 35.6812)

for prop in props:
    print(prop.code, prop.name, prop.en_name)
```

Example output:

```text
130010 東京都 Tokyo
1310100 千代田区 Chiyoda City
```

## License

MIT

Data is sourced from the Japan Meteorological Agency website.
