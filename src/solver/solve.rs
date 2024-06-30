use std::collections::HashMap;
use itertools::Itertools;
use ordered_float::{self, OrderedFloat};

use super::{graph::{AdjMat, Point, PointRole}, order::Order};

pub struct DeliverySolver {
    pub adj_mat: AdjMat,
    pub pt_role: PointRole,

    pub nearest_sender: HashMap<usize, usize>,
}

impl DeliverySolver {
    pub fn new(data: Vec<(i32, i32)>, s_ids: Vec<usize>, r_ids: Vec<usize>) -> Self {
        let points = Point::new_from(data);
        let adj_mat = AdjMat::new(points);
        let pt_role = PointRole::new(s_ids, r_ids);
        let nearest_sender = DeliverySolver::init_near_map(&adj_mat, &pt_role);

        Self { adj_mat: adj_mat, pt_role: pt_role, nearest_sender: nearest_sender }
    }

    fn init_near_map(adj_mat: &AdjMat, pt_role: &PointRole) -> HashMap<usize, usize> {
        let mut nearest_sender: HashMap<usize, usize> = HashMap::new();

        pt_role.recvers.iter().for_each(|&recver| {
            let recver_dist = &adj_mat
                .dist[recver];

            let dist_from_sender = pt_role
                .senders
                .iter()
                .map(|&sender| (
                    sender,
                    recver_dist[sender],
                )).collect::<HashMap<_, _>>();

            nearest_sender.insert(
                recver, 
                dist_from_sender
                    .into_iter()
                    .min_by_key(|(_, v)|
                        OrderedFloat(*v)
                    ).unwrap().0
            );
        });

        nearest_sender
    }

    pub fn prog_per_orders(&self, orders: Vec<Order>) {  //directly solve the 1st pris' orders
        let sorted_order = orders
            .into_iter()
            .sorted_by(|x, y| {
                let x_fit = OrderedFloat(
                    self
                        .adj_mat
                        .get_dist(
                            x.owned,
                            self
                                .nearest_sender[&x.owned]
                        )
                );

                let y_fit = OrderedFloat(
                    self
                        .adj_mat
                        .get_dist(
                            y.owned,
                            self
                                .nearest_sender[&y.owned]
                        )
                );

                x_fit.cmp(&y_fit)
            }).collect::<Vec<_>>();

        

        
    }

    
}
