use std::{collections::HashMap, ops::Deref};

use ordered_float::NotNan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeID,
    pub data: NodeData,
}

///         ^UP
///         |
/// LEFT <-   -> RIGHT
///         |
///         vDOWN
pub mod navi {
    pub type Direction = (i16, i16);
    pub const SITU: (i16, i16) = (0, 0);
    pub const UP: (i16, i16) = (0, 1);
    pub const DOWN: (i16, i16) = (0, -1);
    pub const LEFT: (i16, i16) = (-1, 0);
    pub const RIGHT: (i16, i16) = (1, 0);

    pub const UP_LEFT: (i16, i16) = (-1, 1);
    pub const UP_RIGHT: (i16, i16) = (1, 1);
    pub const DOWN_LEFT: (i16, i16) = (-1, -1);
    pub const DOWN_RIGHT: (i16, i16) = (1, -1);
}

/// y
/// ^ 0,1,2
/// | 3,4,5,
/// | 6,7,8
/// |------> x
pub const INDEXED_NAVI: [(i16, i16); 9] = [
    (-1, 1),
    (0, 1),
    (1, 1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

pub const ALLOWED_NAVI: [(i16, i16); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct NodeID(pub i16, pub i16);
impl NodeID {
    pub const UP_LEFT: Self = NodeID(i16::MIN, i16::MAX);
    pub const UP_MIDDLE: Self = NodeID(0, i16::MAX);
    pub const UP_RIGHT: Self = NodeID(i16::MAX, i16::MAX);
    pub const LEFT_MIDDLE: Self = NodeID(i16::MIN, 0);
    pub const SITU: Self = NodeID(0, 0);
    pub const ORIGIN: Self = NodeID(0, 0);
    pub const RIGHT_MIDDLE: Self = NodeID(i16::MAX, 0);
    pub const DOWN_LEFT: Self = NodeID(i16::MIN, i16::MIN);
    pub const DOWN_MIDDLE: Self = NodeID(0, i16::MIN);
    pub const DOWN_RIGHT: Self = NodeID(i16::MAX, i16::MIN);
}

impl From<(i16, i16)> for NodeID {
    fn from(value: (i16, i16)) -> Self {
        NodeID(value.0, value.1)
    }
}
impl Into<(i16, i16)> for NodeID {
    fn into(self) -> (i16, i16) {
        (self.0, self.1)
    }
}

///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Cell(i8);
impl Cell {
    pub fn new(value: i8) -> Self {
        Cell(value)
    }
    pub fn from_abslute(value: u8) -> Self {
        if value > 127u8 {
            Cell((value - 127u8) as i8)
        } else {
            Cell(value as i8 - 127i8)
        }
    }
}
impl Cell {
    pub fn temperature(&self) -> i8 {
        self.0
    }

    /// # Example
    /// ```
    /// use entropy_sdk::grid::Cell;
    ///
    /// let t = Cell::new(-127);
    /// let k = Cell::from_abslute(0);
    ///
    /// assert_eq!(t, k);
    /// ```
    pub fn abslute(&self) -> u8 {
        if self.0 < 0 {
            (self.0 - i8::MIN) as u8
        } else {
            self.0 as u8 + 128u8
        }
    }

    pub fn carnot_efficiency(self, other: Cell) -> f32 {
        let one = self.abslute();
        let other = other.abslute();
        let one = unsafe { NotNan::new_unchecked(one as f32) };
        let other = unsafe { NotNan::new_unchecked(other as f32) };
        let (h, c) = if one > other {
            (*one, *other)
        } else {
            (*other, *one)
        };
        1f32 - c / h
    }
}
impl Deref for Cell {
    type Target = i8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeData(pub Vec<Cell>);
impl NodeData {
    pub fn from_vec(value: Vec<i8>) -> Self {
        NodeData(value.into_iter().map(|v| Cell(v)).collect())
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn hotest(&self) -> (usize, Cell) {
        let (mut i_max, mut c_max) = (0, self.0[0]);
        self.0.iter().enumerate().for_each(|(i, c)| {
            if *c > c_max {
                i_max = i;
                c_max = c.clone();
            }
        });

        (i_max, c_max)
    }

    pub fn coldest(&self) -> (usize, Cell) {
        let (mut i_min, mut c_min) = (0, self.0[0]);
        self.0.iter().enumerate().for_each(|(i, c)| {
            if *c < c_min {
                i_min = i;
                c_min = c.clone();
            }
        });

        (i_min, c_min)
    }

    /// ```
    /// use entropy_sdk::grid::NodeData;
    /// assert_eq!(NodeData::from_vec(vec![1i8,2,3,4,5]).entropy(), 2.321928094887362);
    /// assert_eq!(NodeData::from_vec(vec![1i8,1,1,1,1]).entropy(), 0.0);
    /// ```
    pub fn entropy(&self) -> f64 {
        let data = &self.0;
        let mut frequency = HashMap::new();
        let len = data.len() as f64;

        // 统计每个字节出现的次数
        data.iter().for_each(|c| {
            *frequency.entry(*c).or_insert(0) += 1;
        });

        // 计算信息熵
        let mut entropy = 0.0;
        for (_, &count) in &frequency {
            let probability = count as f64 / len;
            if probability > 0.0 {
                entropy -= probability * (probability.ln() / std::f64::consts::LN_2);
            }
        }

        entropy
    }
}
