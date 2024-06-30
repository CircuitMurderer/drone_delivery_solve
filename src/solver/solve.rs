use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use itertools::Itertools;
use ordered_float::{self, OrderedFloat};

use super::{graph::{AdjMat, Edge, Point, PointRole}, order::Order, tree::Tree};

pub struct Config {
    pub d_speed: i32,       // km per min: 1
    pub d_longest: i32,     // longest fly dist: 20
    pub d_max_carry: i32,       // max carry, I set it to 3

    pub gen_duration: i32,  // duration between gen, I set it to 30(min)
    pub gen_orders: i32,    // gen orders' number, I set it to 3
}

pub struct SolveInfo {
    pub d_dist: OrderedFloat<f64>,
    pub d_carry: i32,
}

pub struct DeliverySolver {
    pub adj_mat: AdjMat,
    pub pt_role: PointRole,

    pub config: Config,
    pub nearest_sender: HashMap<usize, usize>,
}

impl DeliverySolver {
    pub fn new(data: Vec<(i32, i32)>, s_ids: Vec<usize>, r_ids: Vec<usize>, conf: Config) -> Self {
        let points = Point::new_from(data);
        let adj_mat = AdjMat::new(points);
        let pt_role = PointRole::new(s_ids, r_ids);
        let nearest_sender = DeliverySolver::init_near_map(&adj_mat, &pt_role);

        Self { adj_mat: adj_mat, pt_role: pt_role, config: conf, nearest_sender: nearest_sender }
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

    pub fn get_sorted_orders(&self, orders: Vec<Order>) -> Vec<Order> {
        orders
            .into_iter()
            .sorted_by(|x, y| {
                let x_fit = OrderedFloat(
                    self
                        .adj_mat
                        .get_dist(
                            x.owned,
                            self.nearest_sender[&x.owned]
                        )
                );

                let y_fit = OrderedFloat(
                    self
                        .adj_mat
                        .get_dist(
                            y.owned,
                            self.nearest_sender[&y.owned]
                        )
                );

                x_fit.cmp(&y_fit)
            }).collect::<Vec<_>>()
    }

    pub fn build_limited_mst(&self, root: usize, recvers: HashSet<usize>, conf: Config) {
        let tree = Tree::new_with_root(root);
        let left_recvs = recvers.clone();

        let root_edges = self.adj_mat.get_edges(root, &left_recvs);
        let mut edge_queue: BinaryHeap<Edge> = BinaryHeap::from(root_edges);

        let mut info = SolveInfo { d_dist: OrderedFloat(0.), d_carry: 0 };
        while !left_recvs.is_empty() {
            let edge = edge_queue.pop();
            
        }
    }

    pub fn prog_per_orders(&self, orders: Vec<Order>) {  //directly solve the 1st pris' orders
        let sorted_orders = self.get_sorted_orders(orders);
        let recvers = sorted_orders
            .iter()
            .map(|it| it.owned)
            .collect::<HashSet<_>>();
        let mut edges: BinaryHeap<Edge> = BinaryHeap::new();

        let orders_deque = VecDeque::from(sorted_orders);
        while !orders_deque.is_empty() {

        }

        
    }

    
}
