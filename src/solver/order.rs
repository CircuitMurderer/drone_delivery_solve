use super::graph::PointRole;
use rand::{rngs::ThreadRng, seq::IteratorRandom, thread_rng};

#[derive(Clone, Copy)]
pub enum Priority {
    High,
    Mid,
    Low,
}

pub enum GenWay {
    Every,
    OnlyHigh,
    OnlyMid,
    OnlyLow,
}

#[derive(Clone, Copy)]
pub struct Order {
    pub pri: Priority,       // unit: minute, 30: 1st | 90: 2nd | 180: 3rd
    pub owned: usize,   // the order hold by which point
}

pub struct OrderGener {
    pub pt_role: PointRole,
    pub rng: ThreadRng,
}

pub fn pri_to_num(pri: Priority) -> i32 {
    match pri {
        Priority::High => 30,
        Priority::Mid => 90,
        Priority::Low => 180,
    }
}

impl OrderGener {
    pub fn new(pt_role: PointRole) -> Self {
        Self { pt_role: pt_role, rng: thread_rng() }
    }

    pub fn gen(&mut self, n: usize, way: GenWay) -> Vec<Order> {
        let chosen = self
            .pt_role
            .recvers
            .clone()
            .into_iter()
            .choose_multiple(&mut self.rng, n);
        
        let enums = [
            Priority::High,
            Priority::Mid,
            Priority::Low,
        ];

        match way {
            GenWay::Every => chosen.into_iter().map(|it| {
                Order {
                    owned: it,
                    pri: enums
                        .iter()
                        .choose(&mut self.rng)
                        .unwrap()
                        .to_owned()
                }
            }).collect::<Vec<_>>(),

            GenWay::OnlyHigh => chosen.into_iter().map(|it| {
                Order {
                    owned: it,
                    pri: Priority::High,
                }
            }).collect::<Vec<_>>(),

            GenWay::OnlyMid => chosen.into_iter().map(|it| {
                Order {
                    owned: it,
                    pri: Priority::Mid,
                }
            }).collect::<Vec<_>>(),

            GenWay::OnlyLow => chosen.into_iter().map(|it| {
                Order {
                    owned: it,
                    pri: Priority::Low,
                }
            }).collect::<Vec<_>>(),
        }
    }
}

