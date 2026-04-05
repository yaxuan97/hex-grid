# HexGrid - 六边形网格数据结构库

一个用于在WebAssembly中高效存储和管理六边形网格数据的Rust库，使用轴向坐标系统，支持任意数据类型存储。

[English](README.md)

## 功能特性

- **轴向坐标系统**: 使用标准的六边形轴向坐标(q, r)，支持负数坐标
- **高效存储**: 基于HashMap的稀疏存储，只存储有数据的格子
- **泛型支持**: 支持存储任意数据类型
- **坐标转换**: 提供像素坐标与轴向坐标的相互转换
- **相邻格子计算**: 快速计算任意格子的6个相邻格子
- **WebAssembly兼容**: 专为WASM环境优化，支持在浏览器中使用
- **多实例管理**: 支持创建和管理多个网格实例

## 安装

### Rust依赖

在你的`Cargo.toml`中添加：

```toml
[dependencies]
hex-grid = "0.1.0"
```

### WebAssembly使用

1. 编译为WASM：

```bash
wasm-pack build --target web
```

1. 在HTML中引入：

```html
<script type="module">
import init, * as wasm from './pkg/hex_grid.js';
await init();
</script>
```

## 快速开始

### Rust中使用

```rust
use hex_grid::{HexGrid, AxialCoord};

// 创建网格
let mut grid: HexGrid<String> = HexGrid::new();

// 创建坐标
let coord = AxialCoord::new(0, 0);

// 插入数据
grid.insert(coord, "中心格子".to_string());

// 获取数据
if let Some(value) = grid.get(&coord) {
    println!("值: {}", value);
}

// 计算相邻格子
let neighbor = coord.neighbor(0); // 东方向相邻格子
println!("相邻坐标: ({}, {})", neighbor.q, neighbor.r);
```

### JavaScript中使用

```javascript
import init, {
    create_grid,
    grid_insert,
    grid_get,
    neighbor,
    from_pixel,
    to_pixel
} from './pkg/hex_table.js';

await init();

// 创建网格
const gridId = create_grid();

// 插入数据
grid_insert(gridId, 0, 0, { terrain: 'grass', height: 1 });

// 获取数据
const value = grid_get(gridId, 0, 0);
console.log(value); // { terrain: 'grass', height: 1 }

// 也可以存储数组或原始值
grid_insert(gridId, 1, 0, ['a', 'b', 'c']);

// 计算相邻格子
const neighborCoord = neighbor(0, 0, 0);
console.log(neighborCoord); // "(1, 0)"

// 坐标转换
const pixelCoord = to_pixel(1, 0, 1.0);
console.log(pixelCoord); // "(1.50, 0.87)"

const axialCoord = from_pixel(1.5, 0.0, 1.0);
console.log(axialCoord); // "(1, 0)"
```

## API文档

### AxialCoord

轴向坐标结构体。

#### 工具方法

- `AxialCoord::new(q: i32, r: i32) -> AxialCoord`
  - 创建新的轴向坐标

- `neighbor(&self, direction: u8) -> AxialCoord`
  - 计算相邻格子坐标
  - direction: 0-5，分别对应东、东北、西北、西、西南、东南方向

- `from_pixel(x: f64, y: f64, size: f64) -> AxialCoord`
  - 从像素坐标转换为轴向坐标
  - size: 六边形边长

- `to_pixel(&self, size: f64) -> (f64, f64)`
  - 从轴向坐标转换为像素坐标（返回中心点坐标）
  - size: 六边形边长

### `HexGrid<T>`

六边形网格数据结构。

#### 操作方法

- `HexGrid::new() -> HexGrid<T>`
  - 创建新的空网格

- `insert(&mut self, coord: AxialCoord, value: T) -> Option<T>`
  - 插入数据，返回之前的值（如果存在）

- `get(&self, coord: &AxialCoord) -> Option<&T>`
  - 获取指定坐标的数据

- `get_mut(&mut self, coord: &AxialCoord) -> Option<&mut T>`
  - 获取指定坐标的可变引用

- `remove(&mut self, coord: &AxialCoord) -> Option<T>`
  - 删除指定坐标的数据

- `contains(&self, coord: &AxialCoord) -> bool`
  - 检查坐标是否包含数据

- `keys(&self) -> impl Iterator<Item = &AxialCoord>`
  - 获取所有坐标的迭代器

- `values(&self) -> impl Iterator<Item = &T>`
  - 获取所有数据的迭代器

- `iter(&self) -> impl Iterator<Item = (&AxialCoord, &T)>`
  - 获取坐标和数据的迭代器

- `clear(&mut self)`
  - 清空网格

- `len(&self) -> usize`
  - 获取网格中的元素数量

- `is_empty(&self) -> bool`
  - 检查网格是否为空

### WebAssembly导出函数

#### 网格管理

- `create_grid() -> u32`
  - 创建新网格，返回网格ID

- `destroy_grid(id: u32) -> bool`
  - 销毁指定网格，返回是否成功

- `list_grids() -> String`
  - 列出所有活跃网格ID

#### 网格操作

- `grid_insert(id: u32, q: i32, r: i32, value: JsValue) -> String`
  - 在指定网格插入任意JS值，包括对象、数组、字串、数字、布尔或二进制数据

- `grid_get(id: u32, q: i32, r: i32) -> JsValue`
  - 从指定网格获取数据，返回 `null` 表示未找到

- `grid_remove(id: u32, q: i32, r: i32) -> JsValue`
  - 从指定网格删除数据，返回被删除的JS值；未找到返回 `null`

- `grid_contains(id: u32, q: i32, r: i32) -> String`
  - 检查指定网格是否包含坐标

- `grid_len(id: u32) -> String`
  - 获取指定网格长度

- `grid_keys(id: u32) -> String`
  - 获取指定网格的所有坐标

- `grid_clear(id: u32) -> String`
  - 清空指定网格

#### 坐标工具

- `neighbor(q: i32, r: i32, direction: u8) -> String`
  - 测试相邻格子计算

- `from_pixel(x: f64, y: f64, size: f64) -> String`
  - 测试像素转轴向坐标

- `to_pixel(q: i32, r: i32, size: f64) -> String`
  - 测试轴向转像素坐标

## 坐标系统说明

### 轴向坐标

使用(q, r)坐标系统，其中：

- q: 水平轴
- r: 60度倾斜轴
- s = -q - r（隐含第三轴）

### 方向编码

相邻格子的方向编码（从正东开始逆时针）：

- 0: 东 (1, 0)
- 1: 东北 (0, 1)
- 2: 西北 (-1, 1)
- 3: 西 (-1, 0)
- 4: 西南 (0, -1)
- 5: 东南 (1, -1)

### 像素坐标转换

使用平面顶点朝上的六边形（flat-top）布局：

- 从轴向到像素：`x = size * (3/2 * q), y = size * (√3/2 * q + √3 * r)`
- 从像素到轴向：使用立方坐标舍入算法确保准确性

## 示例应用

### 游戏地图

```javascript
// 创建游戏地图网格
const mapId = create_grid();

// 设置地形
grid_insert(mapId, 0, 0, { terrain: 'grass', height: 1 });
grid_insert(mapId, 1, 0, { terrain: 'water', depth: 2 });

// 查询位置
const terrain = grid_get(mapId, 0, 0);
console.log(terrain); // { terrain: 'grass', height: 1 }
```

## 构建和测试

### 构建WASM

```bash
wasm-pack build --target web
```

### 运行测试

```bash
cargo test
```

### 本地开发服务器

```bash
python -m http.server 8000
```

然后访问 `http://localhost:8000/test_zhCN.html` 查看测试页面。

## 性能特点

- **内存效率**: 只存储有数据的格子，适合稀疏网格
- **查找速度**: HashMap提供O(1)平均查找时间
- **WASM优化**: 针对WebAssembly环境编译，体积小巧
- **类型安全**: Rust的类型系统确保数据一致性

## 许可证

[MIT License](LICENSE)

## 贡献

欢迎提交Issue和Pull Request！

## 版本历史

- v0.1.0: 初始版本，支持基本网格操作和坐标转换
