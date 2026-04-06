// lib.rs - 六边形网格库，提供轴向坐标系统和基本操作
// 这个库实现了一个简单的六边形网格数据结构，支持插入、查询、删除等基本操作
// 以及坐标转换和距离计算功能。通过 wasm_bindgen 导出函数，
// copyright (c) 2026 by yaxuan97, licensed under MIT

use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use std::cell::RefCell;
use std::thread_local;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_wasm_bindgen;
use js_sys::JSON;

/// 轴向坐标结构体，用于六边形网格
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AxialCoord {
    pub q: i32,
    pub r: i32,
}

impl AxialCoord {
    /// 创建新的轴向坐标
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    /// 计算相邻格子的轴向坐标
    /// direction: 0-5，对应6个方向
    pub fn neighbor(&self, direction: u8) -> AxialCoord {
        const DIRECTIONS: [(i32, i32); 6] = [
            (1, 0),   // 0: 东
            (0, 1),   // 1: 东北
            (-1, 1),  // 2: 西北
            (-1, 0),  // 3: 西
            (0, -1),  // 4: 西南
            (1, -1),  // 5: 东南
        ];
        let (dq, dr) = DIRECTIONS[direction as usize % 6];
        AxialCoord::new(self.q + dq, self.r + dr)
    }

    /// 从直角坐标计算轴向坐标
    /// size: 六边形的大小（边长）
    pub fn from_pixel(x: f64, y: f64, size: f64) -> AxialCoord {
        let frac_q = (2.0 / 3.0 * x) / size;
        let frac_r = (-1.0 / 3.0 * x + 3.0_f64.sqrt() / 3.0 * y) / size;
        let (q, r) = Self::axial_round(frac_q, frac_r);
        AxialCoord::new(q, r)
    }

    /// 计算轴向坐标对应的直角坐标（中点）
    /// size: 六边形的大小（边长）
    pub fn to_pixel(&self, size: f64) -> (f64, f64) {
        let x = size * (3.0 / 2.0 * self.q as f64);
        let y = size * (3.0_f64.sqrt() / 2.0 * self.q as f64 + 3.0_f64.sqrt() * self.r as f64);
        (x, y)
    }

    /// 计算两个六边形格子的网格距离（步数）
    /// 这是沿着网格路径的最短步数
    pub fn hex_distance(&self, other: &AxialCoord) -> i32 {
        let dq = (self.q - other.q).abs();
        let dr = (self.r - other.r).abs();
        let ds = (-self.q - self.r + other.q + other.r).abs();
        dq.max(dr).max(ds)
    }

    /// 计算两个六边形格子中心点的直线距离
    /// size: 六边形的大小（边长）
    pub fn euclidean_distance(&self, other: &AxialCoord, size: f64) -> f64 {
        let (x1, y1) = self.to_pixel(size);
        let (x2, y2) = other.to_pixel(size);
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }

    /// 辅助函数：舍入立方坐标到轴向坐标
    fn axial_round(frac_q: f64, frac_r: f64) -> (i32, i32) {
        let frac_s = -frac_q - frac_r;
        let q = frac_q.round() as i32;
        let r = frac_r.round() as i32;
        let s = frac_s.round() as i32;

        let q_diff = (q as f64 - frac_q).abs();
        let r_diff = (r as f64 - frac_r).abs();
        let s_diff = (s as f64 - frac_s).abs();

        if q_diff > r_diff && q_diff > s_diff {
            (-r - s, r)
        } else if r_diff > s_diff {
            (q, -q - s)
        } else {
            (q, -q - s) // for equal, adjust r
        }
    }
}

/// 六边形网格数据结构
#[derive(Clone, Serialize, Deserialize)]
pub struct HexGrid<T> {
    data: HashMap<AxialCoord, T>,
}

impl<T> HexGrid<T> {
    /// 创建一个新的空六边形网格
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// 在指定坐标插入数据
    pub fn insert(&mut self, coord: AxialCoord, value: T) -> Option<T> {
        self.data.insert(coord, value)
    }

    /// 获取指定坐标的数据
    pub fn get(&self, coord: &AxialCoord) -> Option<&T> {
        self.data.get(coord)
    }

    /// 获取指定坐标的可变引用
    pub fn get_mut(&mut self, coord: &AxialCoord) -> Option<&mut T> {
        self.data.get_mut(coord)
    }

    /// 移除指定坐标的数据
    pub fn remove(&mut self, coord: &AxialCoord) -> Option<T> {
        self.data.remove(coord)
    }

    /// 检查坐标是否包含数据
    pub fn contains(&self, coord: &AxialCoord) -> bool {
        self.data.contains_key(coord)
    }

    /// 获取所有坐标的迭代器
    pub fn keys(&self) -> impl Iterator<Item = &AxialCoord> {
        self.data.keys()
    }

    /// 获取所有数据的迭代器
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.data.values()
    }

    /// 获取所有坐标和数据的迭代器
    pub fn iter(&self) -> impl Iterator<Item = (&AxialCoord, &T)> {
        self.data.iter()
    }

    /// 清空网格
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// 获取网格中的元素数量
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 检查网格是否为空
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 将网格序列化为二进制数据
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        bincode::serialize(self).map_err(Into::into)
    }

    /// 从二进制数据反序列化网格
    pub fn deserialize(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
    {
        bincode::deserialize(data).map_err(Into::into)
    }
}

impl HexGrid<JsValue> {
    /// 将网格序列化为二进制数据（针对 JsValue）
    pub fn serialize_js(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let serialized_data: Vec<(AxialCoord, String)> = self.data.iter()
            .map(|(k, v)| -> Result<_, Box<dyn std::error::Error>> {
                let json_str = JSON::stringify(v)
                    .map(|js_str| js_str.as_string().unwrap_or("null".to_string()))
                    .unwrap_or("null".to_string());
                Ok((*k, json_str))
            })
            .collect::<Result<_, _>>()?;
        bincode::serialize(&serialized_data).map_err(Into::into)
    }

    /// 从二进制数据反序列化网格（针对 JsValue）
    pub fn deserialize_js(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let serialized_data: Vec<(AxialCoord, String)> = bincode::deserialize(data)?;
        let data = serialized_data.into_iter()
            .map(|(k, json_str)| -> Result<_, Box<dyn std::error::Error>> {
                let js_value = JSON::parse(&json_str)
                    .map_err(|_| "JSON parse error".to_string())?;
                Ok((k, js_value))
            })
            .collect::<Result<_, _>>()?;
        Ok(HexGrid { data })
    }
}

impl<T> Default for HexGrid<T> {
    fn default() -> Self {
        Self::new()
    }
}

thread_local! {
    static GRIDS: RefCell<HashMap<u32, HexGrid<JsValue>>> = RefCell::new(HashMap::new());
    static NEXT_ID: RefCell<u32> = RefCell::new(1);
}

#[wasm_bindgen]
pub fn create_grid() -> u32 {
    GRIDS.with(|grids| {
        NEXT_ID.with(|next_id| {
            let mut id = next_id.borrow_mut();
            let grid_id = *id;
            *id += 1;

            let mut g = grids.borrow_mut();
            g.insert(grid_id, HexGrid::new());

            grid_id
        })
    })
}

#[wasm_bindgen]
pub fn destroy_grid(id: u32) -> bool {
    GRIDS.with(|grids| {
        let mut g = grids.borrow_mut();
        g.remove(&id).is_some()
    })
}

#[wasm_bindgen]
pub fn grid_clear(id: u32) -> String {
    GRIDS.with(|grids| {
        let mut grids = grids.borrow_mut();
        if let Some(grid) = grids.get_mut(&id) {
            grid.clear();
            format!("网格 {} 已清空", id)
        } else {
            format!("网格 {} 不存在", id)
        }
    })
}

#[wasm_bindgen]
pub fn grid_insert(id: u32, q: i32, r: i32, value: JsValue) -> String {
    GRIDS.with(|grids| {
        let mut grids = grids.borrow_mut();
        if let Some(grid) = grids.get_mut(&id) {
            let coord = AxialCoord::new(q, r);
            match grid.insert(coord, value) {
                Some(_old) => "插入成功，已覆盖旧值".to_string(),
                None => "插入成功".to_string(),
            }
        } else {
            format!("网格 {} 不存在", id)
        }
    })
}

#[wasm_bindgen]
pub fn grid_get(id: u32, q: i32, r: i32) -> JsValue {
    GRIDS.with(|grids| {
        let grids = grids.borrow();
        if let Some(grid) = grids.get(&id) {
            let coord = AxialCoord::new(q, r);
            grid.get(&coord).cloned().unwrap_or(JsValue::NULL)
        } else {
            JsValue::NULL
        }
    })
}

#[wasm_bindgen]
pub fn grid_remove(id: u32, q: i32, r: i32) -> JsValue {
    GRIDS.with(|grids| {
        let mut grids = grids.borrow_mut();
        if let Some(grid) = grids.get_mut(&id) {
            let coord = AxialCoord::new(q, r);
            grid.remove(&coord).unwrap_or(JsValue::NULL)
        } else {
            JsValue::NULL
        }
    })
}

#[wasm_bindgen]
pub fn grid_contains(id: u32, q: i32, r: i32) -> String {
    GRIDS.with(|grids| {
        let grids = grids.borrow();
        if let Some(grid) = grids.get(&id) {
            let coord = AxialCoord::new(q, r);
            if grid.contains(&coord) {
                "包含".to_string()
            } else {
                "不包含".to_string()
            }
        } else {
            format!("网格 {} 不存在", id)
        }
    })
}

#[wasm_bindgen]
pub fn grid_len(id: u32) -> String {
    GRIDS.with(|grids| {
        let g = grids.borrow();
        if let Some(grid) = g.get(&id) {
            format!("长度: {}", grid.len())
        } else {
            format!("网格 {} 不存在", id)
        }
    })
}

#[wasm_bindgen]
pub fn grid_keys(id: u32) -> String {
    GRIDS.with(|grids| {
        let g = grids.borrow();
        if let Some(grid) = g.get(&id) {
            let keys: Vec<String> = grid.keys().map(|coord| format!("({}, {})", coord.q, coord.r)).collect();
            format!("坐标: {}", keys.join(", "))
        } else {
            format!("网格 {} 不存在", id)
        }
    })
}

#[wasm_bindgen]
pub fn list_grids() -> String {
    GRIDS.with(|grids| {
        let g = grids.borrow();
        let ids: Vec<String> = g.keys().map(|id| id.to_string()).collect();
        format!("活跃网格: {}", ids.join(", "))
    })
}

#[wasm_bindgen]
pub fn neighbor(q: i32, r: i32, direction: u8) -> String {
    let coord = AxialCoord::new(q, r);
    let neighbor = coord.neighbor(direction);
    format!("({}, {})", neighbor.q, neighbor.r)
}

#[wasm_bindgen]
pub fn from_pixel(x: f64, y: f64, size: f64) -> String {
    let coord = AxialCoord::from_pixel(x, y, size);
    format!("({}, {})", coord.q, coord.r)
}

#[wasm_bindgen]
pub fn to_pixel(q: i32, r: i32, size: f64) -> String {
    let coord = AxialCoord::new(q, r);
    let (x, y) = coord.to_pixel(size);
    format!("({:.3}, {:.3})", x, y)
}

#[wasm_bindgen]
pub fn hex_distance(q1: i32, r1: i32, q2: i32, r2: i32) -> i32 {
    let coord1 = AxialCoord::new(q1, r1);
    let coord2 = AxialCoord::new(q2, r2);
    coord1.hex_distance(&coord2)
}

#[wasm_bindgen]
pub fn euclidean_distance(q1: i32, r1: i32, q2: i32, r2: i32, size: f64) -> f64 {
    let coord1 = AxialCoord::new(q1, r1);
    let coord2 = AxialCoord::new(q2, r2);
    coord1.euclidean_distance(&coord2, size)
}

#[wasm_bindgen]
pub fn grid_serialize(id: u32) -> Vec<u8> {
    GRIDS.with(|grids| {
        let grids = grids.borrow();
        if let Some(grid) = grids.get(&id) {
            grid.serialize_js().unwrap_or_default()
        } else {
            Vec::new()
        }
    })
}

#[wasm_bindgen]
pub fn grid_deserialize(id: u32, data: &[u8]) -> String {
    GRIDS.with(|grids| {
        let mut grids = grids.borrow_mut();
        match HexGrid::<JsValue>::deserialize_js(data) {
            Ok(new_grid) => {
                grids.insert(id, new_grid);
                format!("网格 {} 已从二进制数据恢复", id)
            }
            Err(e) => format!("反序列化失败: {}", e),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_grid() {
        let mut grid: HexGrid<i32> = HexGrid::new();

        let coord1 = AxialCoord::new(0, 0);
        let coord2 = AxialCoord::new(-1, 1);
        let coord3 = AxialCoord::new(2, -3);

        // 插入数据
        assert!(grid.insert(coord1, 10).is_none());
        assert!(grid.insert(coord2, 20).is_none());
        assert!(grid.insert(coord3, 30).is_none());

        // 获取数据
        assert_eq!(grid.get(&coord1), Some(&10));
        assert_eq!(grid.get(&coord2), Some(&20));
        assert_eq!(grid.get(&coord3), Some(&30));

        // 检查不存在的坐标
        let coord4 = AxialCoord::new(1, 1);
        assert_eq!(grid.get(&coord4), None);

        // 移除数据
        assert_eq!(grid.remove(&coord2), Some(20));
        assert_eq!(grid.get(&coord2), None);

        // 检查长度
        assert_eq!(grid.len(), 2);
        assert!(!grid.is_empty());

        // 清空
        grid.clear();
        assert_eq!(grid.len(), 0);
        assert!(grid.is_empty());
    }

    #[test]
    fn test_neighbor() {
        let coord = AxialCoord::new(0, 0);
        assert_eq!(coord.neighbor(0), AxialCoord::new(1, 0));
        assert_eq!(coord.neighbor(1), AxialCoord::new(0, 1));
        assert_eq!(coord.neighbor(2), AxialCoord::new(-1, 1));
        assert_eq!(coord.neighbor(3), AxialCoord::new(-1, 0));
        assert_eq!(coord.neighbor(4), AxialCoord::new(0, -1));
        assert_eq!(coord.neighbor(5), AxialCoord::new(1, -1));
        // 测试方向循环
        assert_eq!(coord.neighbor(6), AxialCoord::new(1, 0));
    }

    #[test]
    fn test_pixel_to_hex() {
        let size = 1.0;
        // 原点
        let coord = AxialCoord::from_pixel(0.0, 0.0, size);
        assert_eq!(coord, AxialCoord::new(0, 0));

        // 一些已知点
        let coord = AxialCoord::from_pixel(1.5, 0.0, size);
        assert_eq!(coord, AxialCoord::new(1, 0));

        let coord = AxialCoord::from_pixel(0.0, 3.0_f64.sqrt(), size);
        assert_eq!(coord, AxialCoord::new(0, 1));
    }

    #[test]
    fn test_hex_to_pixel() {
        let size = 1.0;
        let coord = AxialCoord::new(0, 0);
        let (x, y) = coord.to_pixel(size);
        assert!((x - 0.0).abs() < 1e-6);
        assert!((y - 0.0).abs() < 1e-6);

        let coord = AxialCoord::new(1, 0);
        let (x, y) = coord.to_pixel(size);
        assert!((x - 1.5).abs() < 1e-6);
        assert!((y - (3.0_f64.sqrt() / 2.0)).abs() < 1e-6);

        let coord = AxialCoord::new(0, 1);
        let (x, y) = coord.to_pixel(size);
        assert!((x - 0.0).abs() < 1e-6);
        assert!((y - 3.0_f64.sqrt()).abs() < 1e-6);
    }

    #[test]
    fn test_hex_distance() {
        let coord1 = AxialCoord::new(0, 0);
        let coord2 = AxialCoord::new(0, 0);
        assert_eq!(coord1.hex_distance(&coord2), 0);

        let coord1 = AxialCoord::new(0, 0);
        let coord2 = AxialCoord::new(1, 0);
        assert_eq!(coord1.hex_distance(&coord2), 1);

        let coord1 = AxialCoord::new(0, 0);
        let coord2 = AxialCoord::new(1, 1);
        assert_eq!(coord1.hex_distance(&coord2), 2);

        let coord1 = AxialCoord::new(0, 0);
        let coord2 = AxialCoord::new(1, -1);
        assert_eq!(coord1.hex_distance(&coord2), 1);
    }

    #[test]
    fn test_euclidean_distance() {
        let size = 1.0;
        let coord1 = AxialCoord::new(0, 0);
        let coord2 = AxialCoord::new(0, 0);
        assert!((coord1.euclidean_distance(&coord2, size) - 0.0).abs() < 1e-6);

        let coord1 = AxialCoord::new(0, 0);
        let coord2 = AxialCoord::new(1, 0);
        let expected_distance = (3.0_f64).sqrt(); // sqrt(1.5^2 + (√3/2)^2) = sqrt(2.25 + 0.75) = sqrt(3)
        assert!((coord1.euclidean_distance(&coord2, size) - expected_distance).abs() < 1e-6);
    }

    #[test]
    fn test_round_trip() {
        let size = 1.0;
        let coords = vec![
            AxialCoord::new(0, 0),
            AxialCoord::new(1, 0),
            AxialCoord::new(0, 1),
            AxialCoord::new(-1, 1),
            AxialCoord::new(2, -1),
        ];

        for coord in coords {
            let (x, y) = coord.to_pixel(size);
            let back = AxialCoord::from_pixel(x, y, size);
            assert_eq!(coord, back);
        }
    }
}
