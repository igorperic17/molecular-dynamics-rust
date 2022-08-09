use crate::atom;
use crate::particle;
use kiss3d::nalgebra::Vector3;

const COULUMB_CONSTANT: f32 = 9.0 * 10e9;

pub struct Simulation {
    delta_t: f32,
    pub particles: Vec<particle::Particle>,
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

    pub fn step(self: &mut Self) {
        let p: &mut Vec<particle::Particle> = &mut self.particles;
        for i in 0..(p.len() - 1) {
            for j in (i + 1)..p.len() {
                let d_vec: Vector3<f32> = (p[i].pos - p[j].pos).abs();

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

                let q_force = Vector3::new(f_x, f_y, f_z);

                // apply Newton's 2nd law of motion
                p[i].v += q_force;
                p[j].v += q_force;
                let d_i = (p[i].v / p[i].m) * self.delta_t;
                let d_j = (p[j].v / p[j].m) * self.delta_t;
                p[i].pos += d_i;
                p[j].pos += d_j;
                for k in 0..p.len() {
                    println!("{:?}", p[k]);
                }
            }
        }
    }

    pub fn add_atom(self: &mut Self, atom: &atom::Atom) {
        // println!("Adding nucleus to the simulation...");
        self.particles.push(atom.nucleus);
        for particle in &atom.electrons {
            // println!("Adding electron to the simulation...");
            self.particles.push(*particle);
        }
    }
}
