use std::error::Error;
use solve_drone::solver::solve::{Config, DeliverySolver};
use solver::tree::Tree;

use crate::solver::graph::{Point, AdjMat, PointRole};

mod solver;

fn main() -> Result<(), Box<dyn Error>> {
    let point_datas = vec![
        (1, 1), (5, 3), (2, 6), (7, 2), (4, 8), (8, 4), (3, 7), (6, 5), (9, 9),
    ];
    let s_ids = vec![1, 4, 7];
    let r_ids = vec![0, 2, 3, 5, 6, 8];

    let solver = DeliverySolver::new(
        point_datas, 
        s_ids, 
        r_ids, 
        Config {
            d_speed: 1,
            d_longest: 20,
            d_max_carry: 3,
            gen_duration: 30,
            gen_orders: 3
        }
    );

    solver.adj_mat.print_mat();
    println!("{}, {}", solver.pt_role.is_sender(1), solver.pt_role.is_recver(2));

    let mut tree = Tree::new_with_root(1);
    tree.insert_node_by_data(&1, 2);
    tree.insert_node_by_data(&1, 3);
    tree.insert_node_by_data(&2, 4);
    tree.insert_node_by_data(&2, 5);
    tree.insert_node_by_data(&3, 6);
    tree.insert_node_by_data(&5, 7);

    let path = tree.get_path();
    println!("{:?}", path);
    println!("{:?}", solver.nearest_sender);

    Ok(())
}