# int_math

`int_math` is a Rust crate providing mathematical abstractions for 2D vectors and rectangles. It includes:

- **`UVec2`**: A 2D vector with unsigned integer coordinates.
- **`Vec2`**: A 2D vector with signed integer coordinates.
- **`Vec3`**: A 3D vector with signed integer coordinates.
- **`URect`**: A rectangle with unsigned integer coordinates for position and size.
- **`Rect`**: A rectangle with signed integer coordinates for position and unsigned integer dimensions.

## Features

- **Vector Operations**: Supports basic arithmetic operations for `UVec2` and `Vec2`.
- **Rectangles**: Provides methods to create and manipulate rectangles, including calculating centers and applying offsets.

## Usage

Add `int_math` to your `Cargo.toml`:

```toml
[dependencies]
int_math = "0.0.1"
```

Then, use it in your code:

```rust
use int_math::{URect, UVec2};

let rect = URect::new(10, 20, 30, 40);
let center = rect.center();
println!("Center: {:?}", center);
```

## License

Licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
