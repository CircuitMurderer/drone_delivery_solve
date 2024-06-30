use std::collections::HashSet;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct AdjMat {
    pub dist: Vec<Vec<f64>>,
    pub points: Vec<Point>,
}

pub struct PointRole {
    pub senders: HashSet<usize>,
    pub recvers: HashSet<usize>,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    pub fn new_from(data: Vec<(i32, i32)>) -> Vec<Self> {
        data.iter().map(|it|
            Point::new(it.0, it.1)
        ).collect::<Vec<_>>()
    }
}

impl std::ops::Sub for &Point {
    type Output = f64;

    fn sub(self, rhs: Self) -> Self::Output {
        (((rhs.x - self.x).pow(2) + (rhs.y - self.y).pow(2)) as f64).sqrt()
    }
}

impl AdjMat {
    pub fn new(points: Vec<Point>) -> Self {
        Self { dist: AdjMat::get_all_dist(&points), points: points }
    }

    fn get_all_dist(points: &Vec<Point>) -> Vec<Vec<f64>> {
        points.iter().map(|pt_out| 
            points.iter().map(|pt_in|
                pt_out - pt_in
            ).collect::<Vec<_>>()
        ).collect::<Vec<_>>()
    }

    pub fn print_dist(&self) {
        self.dist.iter().for_each(|its| {
            its.iter().for_each(|it| {
                print!("{:.4} ", it);
            });
            println!();
        });
    }
}

impl PointRole {
    pub fn new(s_ids: Vec<usize>, r_ids: Vec<usize>) -> Self {
        Self { 
            senders: s_ids.into_iter().collect::<HashSet<_>>(), 
            recvers: r_ids.into_iter().collect::<HashSet<_>>()
        }
    }

    pub fn is_sender(&self, id: usize) -> bool { 
        self.senders.contains(&id)
    }

    pub fn is_recver(&self, id: usize) -> bool { 
        self.recvers.contains(&id)
    }
}

