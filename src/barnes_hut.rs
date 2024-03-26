use super::body::Body;
use super::gravity::calculate_gravity;
use glam::Vec2;
use crossbeam::thread;

pub struct BHTreeNode {
    pub theta: f32,

    pub is_external: bool,

    pub region_center: Vec2,
    pub region_width: f32,
    
    pub mass_center: Vec2,
    pub mass: f32,

    pub children: Vec<BHTreeNode>
}

impl BHTreeNode {
    pub fn new(theta: f32, region_center: Vec2, region_width: f32) -> Self {
        Self { 
            theta, 
            is_external: true,
            region_center, 
            region_width, 
            mass_center: region_center, 
            mass: 0.0, 
            children: vec![] 
        }
    }

    pub fn reset(&mut self) {
        self.is_external = true;
        self.mass_center = self.region_center;
        self.mass = 0.0;
        self.children.clear();
    }

    pub fn contains_position(&self, position: Vec2) -> bool {
        if (position.x - self.region_center.x).abs() >= self.region_width / 2.0 {
            return false;
        }

        if (position.y - self.region_center.y).abs() >= self.region_width / 2.0 {
            return false;
        }

        return true;
    }

    pub fn insert(&mut self, new_pos: Vec2, new_mass: f32) {
        if !self.contains_position(new_pos) {
            return;
        }

        if self.mass == 0.0 {
            self.mass = new_mass;
            self.mass_center = new_pos;
            return;
        }

        if !self.is_external {
            for child in self.children.iter_mut() {
                child.insert(new_pos, new_mass);
            }
        }
        else {
            let child_width = self.region_width / 2.0;
            let half_child_width = child_width / 2.0;

            let nw_center = self.region_center + Vec2::new(-half_child_width, -half_child_width);
            self.children.push(BHTreeNode::new(self.theta, nw_center, child_width));

            let ne_center = self.region_center + Vec2::new(half_child_width, -half_child_width);
            self.children.push(BHTreeNode::new(self.theta, ne_center, child_width));

            let se_center = self.region_center + Vec2::new(half_child_width, half_child_width);
            self.children.push(BHTreeNode::new(self.theta, se_center, child_width));

            let sw_center = self.region_center + Vec2::new(-half_child_width, half_child_width);
            self.children.push(BHTreeNode::new(self.theta, sw_center, child_width));

            for child in self.children.iter_mut() {
                child.insert(new_pos, new_mass);

                if new_pos != self.mass_center {
                    child.insert(self.mass_center, self.mass);
                }
            }

            self.is_external = false;
        }

        self.mass_center = (new_pos * new_mass + self.mass_center * self.mass) / (self.mass + new_mass);
        self.mass += new_mass;
    }


    pub fn calculate_force(&self, pos: Vec2, mass: f32) -> Vec2 {
        if self.is_external && self.mass > 0.0 && self.mass != mass {
            return calculate_gravity(self.mass_center, self.mass, pos, mass);
        }

        let mut force = Vec2::ZERO;

        let distance = pos.distance(self.mass_center);

        if self.region_width / distance < self.theta {
            return calculate_gravity(self.mass_center, self.mass, pos, mass);
        }
        
        for child in self.children.iter() {
            force += child.calculate_force(pos, mass);
        }

        force
    }
}

pub struct BarnesHut {
    pub tree_root: BHTreeNode,
    pub thread_count: i32,
}

impl BarnesHut {
    pub fn new(theta: f32, center: Vec2, width: f32, thread_count: i32) -> BarnesHut {
        Self {
            tree_root: BHTreeNode::new(theta, center, width),
            thread_count,
        }
    }

    pub fn apply_gravity(&mut self, bodies: &mut Vec<Body>) {
        self.tree_root.reset();

        for body in bodies.iter() {
            self.tree_root.insert(body.position, body.mass);
        }

        let body_count = bodies.len();
        
        let mut chunk_size: usize = body_count / self.thread_count as usize;

        if chunk_size == 0 {
            chunk_size = 1;
        }

        thread::scope(|s| {
            for chunk in bodies.chunks_mut(chunk_size) {
                s.spawn(|_| {
                    for body in chunk.iter_mut() {
                        let force = self.tree_root.calculate_force(body.position, body.mass);
                        body.apply_force(-force);
                    }
                });
            }
        })
        .unwrap();
    }
}
