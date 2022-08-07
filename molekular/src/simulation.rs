use crate::atom;
use crate::math;
use crate::particle;

const COULUMB_CONSTANT: f32 = 9.0 * 10e9;

pub struct Simulation {
    delta_t: f32,
    particles: Vec<particle::Particle>,
    temperature: f32,
}

impl Simulation {
    // deltaT - simulation timestamp [fs]
    pub fn new(delta_t: f32, temperature: f32) -> Self {
        Simulation {
            delta_t: delta_t,
            particles: Vec::new(),
            temperature: temperature,
        }
    }
    pub fn run(self: &mut Self, n_steps: i32) {
        for i in 0..n_steps {
            println!("Step #{}...", i);
            self.step();
        }
    }

    pub fn step(self: &mut Self) {
        let p: &mut Vec<particle::Particle> = &mut self.particles;
        for i in 0..(p.len() - 1) {
            for j in (i + 1)..p.len() {
                let d_vec: math::Vec3 = p[i].pos - p[j].pos;

                let f_charge = COULUMB_CONSTANT * (p[i].q * p[j].q);

                let f_x = if d_vec.x == 0.0 {
                    0.0
                } else {
                    f_charge / d_vec.x
                };

                let f_y = if d_vec.y == 0.0 {
                    0.0
                } else {
                    f_charge / d_vec.y
                };

                let f_z = if d_vec.z == 0.0 {
                    0.0
                } else {
                    f_charge / d_vec.z
                };

                let q_force = math::Vec3::new(f_x, f_y, f_z);

                p[i].pos = p[i].pos + q_force;
                p[j].pos = p[j].pos + q_force;
                for k in 0..p.len() {
                    println!("{:?}", p[k]);
                }
            }
        }
    }

    pub fn add_atom(self: &mut Self, atom: &atom::Atom) {
        self.particles.push(atom.nucleus);
        for particle in &atom.electrons {
            self.particles.push(*particle);
        }
    }
}
