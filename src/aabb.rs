use std::marker::PhantomData;

use rand::seq::IndexedRandom;

const PARTICLES_PER_BOX: usize = 5;

use crate::{Particle, V3};

pub struct AABBTree {
    root: TreeNode,
}

pub struct TreeNode {
    //the center point of the box (this will be the origin(0,0,0) for all children)
    b_center: V3,
    //the distance from the center to the box boundary
    b_size: f32,
    //we split each region into it's 8 quadrants
    children: [Option<Box<TreeNode>>; 8],
    particles: Vec<Particle>,
}

impl From<Vec<Particle>> for AABBTree {
    fn from(value: Vec<Particle>) -> Self {
        let mut r = TreeNode {
            b_center: V3::new(0., 0., 0.),
            b_size: f32::MAX,
            children: [const { None }; 8],
            particles: value,
        };
        r.construct();
        Self { root: r }
    }
}

impl TreeNode {
    pub fn construct(&mut self) {
        if self.particles.len() <= PARTICLES_PER_BOX {
            return;
        }
        let half_size = self.b_size / 2.;
        let hsv = V3::new(half_size, half_size, half_size);
        let n1c = self.b_center + hsv;
        let n1 = TreeNode {
            b_center: n1c,
            b_size: half_size,
            children: [const { None }; 8],
            particles: self
                .particles
                .iter()
                .filter(|p| p.get_pos() < n1c + hsv && p.get_pos() > n1c - hsv)
                .collect(),
        };
    }
}
