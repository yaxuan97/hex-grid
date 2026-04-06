# HexGrid - Hexagonal Grid Data Structure Library

A Rust library for efficiently storing and managing hexagonal grid data in WebAssembly, using axial coordinate system, supporting arbitrary data type storage.

[中文版](README_zhCN.md)

## Features

- **Axial Coordinate System**: Uses standard hexagonal axial coordinates (q, r), supports negative coordinates
- **Efficient Storage**: HashMap-based sparse storage, only stores cells with data
- **Generic Support**: Supports storing arbitrary data types
- **Coordinate Conversion**: Provides mutual conversion between pixel coordinates and axial coordinates
- **Neighbor Calculation**: Quickly calculate 6 neighboring cells of any cell
- **WebAssembly Compatible**: Optimized for WASM environment, supports use in browsers
- **Multi-instance Management**: Supports creating and managing multiple grid instances

## Installation

### Rust Dependencies

Add to your `Cargo.toml`:

```toml
[dependencies]
hex-grid = "0.1.0"
```

### WebAssembly Usage

1. Compile to WASM:

```bash
wasm-pack build --target web
```

1. Include in HTML:

```html
<script type="module">
import init, * as wasm from './pkg/hex_grid.js';
await init();
</script>
```

## Quick Start

### Using in Rust

```rust
use hex_grid::{HexGrid, AxialCoord};

// Create grid
let mut grid: HexGrid<String> = HexGrid::new();

// Create coordinate
let coord = AxialCoord::new(0, 0);

// Insert data
grid.insert(coord, "Center cell".to_string());

// Get data
if let Some(value) = grid.get(&coord) {
    println!("Value: {}", value);
}

// Calculate neighbor
let neighbor = coord.neighbor(0); // East direction neighbor
println!("Neighbor coordinate: ({}, {})", neighbor.q, neighbor.r);
```

### Using in JavaScript

```javascript
import init, {
    create_grid,
    grid_insert,
    grid_get,
    neighbor,
    from_pixel,
    to_pixel
} from './pkg/hex_grid.js';

await init();

// Create grid
const gridId = create_grid();

// Insert data
grid_insert(gridId, 0, 0, { terrain: 'grass', height: 1 });

// Get data
const value = grid_get(gridId, 0, 0);
console.log(value); // { terrain: 'grass', height: 1 }

// Can also store arrays or primitive values
grid_insert(gridId, 1, 0, ['a', 'b', 'c']);

// Calculate neighbor
const neighborCoord = neighbor(0, 0, 0);
console.log(neighborCoord); // "(1, 0)"

// Coordinate conversion
const pixelCoord = to_pixel(1, 0, 1.0);
console.log(pixelCoord); // "(1.50, 0.87)"

const axialCoord = from_pixel(1.5, 0.0, 1.0);
console.log(axialCoord); // "(1, 0)"

// Serialize grid to binary data
const binaryData = grid_serialize(gridId);
console.log(binaryData); // Uint8Array containing serialized data

// Deserialize binary data into a new grid
const newGridId = create_grid();
const result = grid_deserialize(newGridId, binaryData);
console.log(result); // "网格 X 已从二进制数据恢复"

// Verify data is restored
const restoredValue = grid_get(newGridId, 0, 0);
console.log(restoredValue); // { terrain: 'grass', height: 1 }
```

## API Documentation

### AxialCoord

Axial coordinate struct.

#### Utility Methods

- `AxialCoord::new(q: i32, r: i32) -> AxialCoord`
  - Create new axial coordinate

- `neighbor(&self, direction: u8) -> AxialCoord`
  - Calculate neighbor cell coordinate
  - direction: 0-5, corresponding to East, Northeast, Northwest, West, Southwest, Southeast directions

- `from_pixel(x: f64, y: f64, size: f64) -> AxialCoord`
  - Convert from pixel coordinates to axial coordinates
  - size: hexagon side length

- `to_pixel(&self, size: f64) -> (f64, f64)`
  - Convert from axial coordinates to pixel coordinates (returns center point coordinates)
  - size: hexagon side length

### `HexGrid<T>`

Hexagonal grid data structure.

#### Operation Methods

- `HexGrid::new() -> HexGrid<T>`
  - Create new empty grid

- `insert(&mut self, coord: AxialCoord, value: T) -> Option<T>`
  - Insert data, return previous value (if exists)

- `get(&self, coord: &AxialCoord) -> Option<&T>`
  - Get data at specified coordinate

- `get_mut(&mut self, coord: &AxialCoord) -> Option<&mut T>`
  - Get mutable reference at specified coordinate

- `remove(&mut self, coord: &AxialCoord) -> Option<T>`
  - Remove data at specified coordinate

- `contains(&self, coord: &AxialCoord) -> bool`
  - Check if coordinate contains data

- `keys(&self) -> impl Iterator<Item = &AxialCoord>`
  - Get iterator over all coordinates

- `values(&self) -> impl Iterator<Item = &T>`
  - Get iterator over all data

- `iter(&self) -> impl Iterator<Item = (&AxialCoord, &T)>`
  - Get iterator over coordinate and data pairs

- `clear(&mut self)`
  - Clear the grid

- `len(&self) -> usize`
  - Get number of elements in grid

- `is_empty(&self) -> bool`
  - Check if grid is empty

### WebAssembly Exported Functions

#### Grid Management

- `create_grid() -> u32`
  - Create new grid, return grid ID

- `destroy_grid(id: u32) -> bool`
  - Destroy specified grid, return success status

- `list_grids() -> String`
  - List all active grid IDs

#### Grid Operations

- `grid_insert(id: u32, q: i32, r: i32, value: JsValue) -> String`
  - Insert arbitrary JS value in specified grid, including objects, arrays, strings, numbers, booleans, or binary data

- `grid_get(id: u32, q: i32, r: i32) -> JsValue`
  - Get data from specified grid, returns `null` if not found

- `grid_remove(id: u32, q: i32, r: i32) -> JsValue`
  - Remove data from specified grid, returns removed JS value; returns `null` if not found

- `grid_contains(id: u32, q: i32, r: i32) -> String`
  - Check if specified grid contains coordinate

- `grid_len(id: u32) -> String`
  - Get length of specified grid

- `grid_keys(id: u32) -> String`
  - Get all coordinates of specified grid

- `grid_clear(id: u32) -> String`
  - Clear specified grid

- `grid_serialize(id: u32) -> Vec<u8>`
  - Serialize the specified grid to binary data for saving

- `grid_deserialize(id: u32, data: &[u8]) -> String`
  - Deserialize binary data into the specified grid, restoring saved data

#### Coordinate Tools

- `neighbor(q: i32, r: i32, direction: u8) -> String`
  - Test neighbor cell calculation

- `from_pixel(x: f64, y: f64, size: f64) -> String`
  - Test pixel to axial coordinate conversion

- `to_pixel(q: i32, r: i32, size: f64) -> String`
  - Test axial to pixel coordinate conversion

## Coordinate System Explanation

### Axial Coordinates

Uses (q, r) coordinate system, where:

- q: horizontal axis
- r: 60-degree tilted axis
- s = -q - r (implicit third axis)

### Direction Encoding

Neighbor directions encoding (starting from positive east, counterclockwise):

- 0: East (1, 0)
- 1: Northeast (0, 1)
- 2: Northwest (-1, 1)
- 3: West (-1, 0)
- 4: Southwest (0, -1)
- 5: Southeast (1, -1)

### Pixel Coordinate Conversion

Uses flat-top hexagon layout:

- From axial to pixel: `x = size * (3/2 * q), y = size * (√3/2 * q + √3 * r)`
- From pixel to axial: Uses cubic coordinate rounding algorithm for accuracy

## Example Applications

### Game Map

```javascript
// Create game map grid
const mapId = create_grid();

// Set terrain
grid_insert(mapId, 0, 0, { terrain: 'grass', height: 1 });
grid_insert(mapId, 1, 0, { terrain: 'water', depth: 2 });

// Query location
const terrain = grid_get(mapId, 0, 0);
console.log(terrain); // { terrain: 'grass', height: 1 }
```

## Building and Testing

### Build WASM

```bash
wasm-pack build --target web
```

### Run Tests

```bash
cargo test
```

### Local Development Server

```bash
python -m http.server 8000
```

Then visit `http://localhost:8000/test.html` to view the test page.

## Performance Features

- **Memory Efficient**: Only stores cells with data, suitable for sparse grids
- **Fast Lookup**: HashMap provides O(1) average lookup time
- **WASM Optimized**: Compiled for WebAssembly environment, compact size
- **Type Safe**: Rust's type system ensures data consistency

## License

[MIT License](LICENSE)

## Contributing

Issues and Pull Requests are welcome!

## Version History

- v0.1.1: Added serialization and deserialization functionality for saving and loading grid data as binary
- v0.1.0: Initial version, supports basic grid operations and coordinate conversion
