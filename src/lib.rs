use zeus::v3;
mod aabb;

const TOTAL_PARTICLES: usize = 100;
//fps
const ITERATIONS_PER_SECOND: f32 = 120.;
const MAX_PARTICLE_SIZE: f32 = 1.;
const MAX_PARTICLE_MASS: f32 = 50.;
const BOX_SIZE: f32 = 10_000.;
const MAX_START_SPEED: f32 = 40.;

pub struct Container {
    pub particles: [Particle; TOTAL_PARTICLES],
}

impl Container {
    fn collision(&mut self) {
        for p in 0..self.particles.len() {
            for q in 0..self.particles.len() {
                if !(p == q) {
                    if self.check_collision(p, q) {
                        self.collide(p, q);
                    }
                }
            }
            self.particles[p].pos += self.particles[p].velocity * 1. / ITERATIONS_PER_SECOND;
            if self.particles[p].pos.x >= BOX_SIZE {
                self.particles[p].pos.x = BOX_SIZE;
                self.particles[p].velocity.x = -self.particles[p].velocity.x
            }
            if self.particles[p].pos.y >= BOX_SIZE {
                self.particles[p].pos.y = BOX_SIZE;
                self.particles[p].velocity.y = -self.particles[p].velocity.y
            }
            if self.particles[p].pos.z >= BOX_SIZE {
                self.particles[p].pos.z = BOX_SIZE;
                self.particles[p].velocity.z = -self.particles[p].velocity.z
            }
        }
    }

    #[cfg(feature = "random")]
    pub fn new_random() -> Self {
        let mut c = Container {
            particles: [Particle::default(); TOTAL_PARTICLES],
        };
        for i in 0..TOTAL_PARTICLES {
            c.particles[i] = Particle::new_random();
        }
        c
    }

    //a naive collision resolver
    fn collide(&mut self, p: usize, q: usize) {
        //vector orthogonal to surfaces
        let n = self.particles[p].pos - self.particles[q].pos;

        //projection of p's momentum onto the normal vector
        let dp = (self.particles[p].velocity * self.particles[p].mass).proj(&n);

        //projection of q's momentum onto the normal vector
        let dq = (self.particles[q].velocity * self.particles[q].mass).proj(&n);

        self.particles[q].apply_momentum(dp);
        //each action causes an opposite and equal reaction
        self.particles[q].apply_momentum(-dq);

        self.particles[p].apply_momentum(dq);
        //each action causes an opposite and equal reaction
        self.particles[p].apply_momentum(-dp);

        //fix positions
        let ds = self.particles[p].radius + self.particles[q].radius - n.len();
        self.particles[p].pos += n.normalized() * ds;
        self.particles[q].pos -= n.normalized() * ds;
    }

    fn check_collision(&mut self, p: usize, q: usize) -> bool {
        (self.particles[p].pos - self.particles[q].pos).len2()
            < ((self.particles[p].radius + self.particles[q].radius).powi(2))
    }

    pub fn run(&mut self, t: f32) {
        for _ in 0..(t * ITERATIONS_PER_SECOND) as u64 {
            self.collision();
        }
    }
}

type V3 = v3::Vector3<f32>;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Particle {
    pos: V3,
    velocity: V3,
    radius: f32,
    mass: f32,
}

impl Particle {
    pub fn new(pos: V3, velocity: V3, radius: f32, mass: f32) -> Self {
        Self {
            pos,
            velocity,
            radius,
            mass,
        }
    }

    fn apply_momentum(&mut self, force: V3) {
        self.velocity += force / self.mass
    }
    pub fn get_pos(&self) -> V3 {
        return self.pos;
    }
    pub fn get_radius(&self) -> f32 {
        return self.radius;
    }

    #[cfg(feature = "random")]
    fn new_random() -> Self {
        use rand;
        Particle {
            pos: V3::new_random(BOX_SIZE, 0.),
            velocity: V3::new_random(MAX_START_SPEED, 0.),
            radius: rand::random_range(0.0..MAX_PARTICLE_SIZE),
            mass: rand::random_range(0.0..MAX_PARTICLE_MASS),
        }
    }
}
