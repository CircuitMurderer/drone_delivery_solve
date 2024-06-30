use std::error::Error;
use crate::solver::{order::OrderGener, solve::{Config, DeliverySolver}, order::GenWay};

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
            d_longest: 20.,
            d_m_carry: 3,
            gen_duration: 30,
            gen_orders: 3
        }
    );

    solver.adj_mat.print_mat();

    let mut order_gener = OrderGener::new(solver.pt_role.clone());
    for _ in 0..6 {
        let orders = order_gener.gen(6, GenWay::OnlyHigh);
        let sols = solver.prog_per_orders(orders);
        for sol in sols.iter() {
            println!("Min cost: {}", sol.all_dist);
            println!("Route: {:?}", sol.route);
        }
    }

    Ok(())
}