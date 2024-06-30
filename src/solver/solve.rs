use super::{graph::{AdjMat, Point, PointRole}, order::Order};

pub struct DelvSolver {
    adj_mat: AdjMat,
    pt_role: PointRole,
}

impl DelvSolver {
    pub fn new(data: Vec<(i32, i32)>, s_ids: Vec<usize>, r_ids: Vec<usize>) -> Self {
        let points = Point::new_from(data);
        let adj_mat = AdjMat::new(points);
        let pt_role = PointRole::new(s_ids, r_ids);

        Self { adj_mat: adj_mat, pt_role: pt_role }
    }

    pub fn prog_per_orders(orders: Vec<Order>) {  //directly solve the 1st pris' orders

    }

    
}
