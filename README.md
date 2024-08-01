# int_math

`int_math` is a Rust crate providing mathematical abstractions for 2D vectors and rectangles. It includes:

- **`VectorU`**: A 2D vector with unsigned integer coordinates.
- **`VectorI`**: A 2D vector with signed integer coordinates.
- **`RectU`**: A rectangle with unsigned integer coordinates for position and size.
- **`RectI`**: A rectangle with signed integer coordinates for position and unsigned integer dimensions.

## Features

- **Vector Operations**: Supports basic arithmetic operations for `VectorU` and `VectorI`.
- **Rectangles**: Provides methods to create and manipulate rectangles, including calculating centers and applying offsets.

## Usage

Add `int_math` to your `Cargo.toml`:

```toml
[dependencies]
int_math = "0.0.1"
```

Then, use it in your code:

```rust
use int_math::{RectU, VectorU};

let rect = RectU::new(10, 20, 30, 40);
let center = rect.center();
println!("Center: {:?}", center);
```

## License

Licensed under the MIT License. See the [LICENSE](LICENSE) file for details.