use super::graph::Point;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Order {
    pub pri: i32,       // unit: minute, 30: 1st | 90: 2nd | 180: 3rd
    pub owned: usize,   // the order hold by which point
}

pub struct OrderGener {
    pub points: Vec<Point>,
}

pub struct Orders {
    pub orders: Vec<Order>,
    pub owners: Vec<usize>,
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.pri.cmp(&self.pri)
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl OrderGener {
    
}

