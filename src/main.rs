use std::error::Error;
use crate::solver::graph::{Point, AdjMat, PointRole};

mod solver;

fn main() -> Result<(), Box<dyn Error>> {
    let point_datas = vec![
        (1, 1), (5, 3), (2, 6), (7, 2), (4, 8), (8, 4), (3, 7), (6, 5), (9, 9),
    ];
    let pt_role = PointRole::new(vec![1, 4, 7], vec![0, 2, 3, 5, 6, 8]);

    let points = Point::new_from(point_datas);
    let adj_mat = AdjMat::new(points);

    adj_mat.print_dist();
    println!("{}, {}", pt_role.is_sender(1), pt_role.is_recver(2));

    Ok(())
}