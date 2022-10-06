use crate::atom;
use crate::particle;
use kiss3d::nalgebra::Vector3;

const COULUMB_CONSTANT: f64 = 9.0 * 10e9;

pub enum ParticleHistoryEnum {
    Position,
    Velocity,
    Accelleration
}

pub struct Simulation {
    delta_t: f64,
    pub particles: Vec<particle::Particle>,
    temperature: f64,
}

impl Simulation {
    // deltaT - simulation timestamp [fs]
    pub fn new(delta_t: f64, temperature: f64) -> Self {
        Simulation {
            delta_t: delta_t,
            particles: Vec::new(),
            temperature: temperature,
        }
    }

    pub fn step(self: &mut Self) {
        let p: &mut Vec<particle::Particle> = &mut self.particles;
        for i in 0..(p.len() - 1) {
            // TODO: update sphere radius to be visible from the current camera position and the zoom level

            // update positions w.r.t. Coloumb forces
            for j in (i + 1)..p.len() {
                let d_vec: Vector3<f64> = p[i].pos - p[j].pos;

                let f_charge = -COULUMB_CONSTANT * (p[i].q * p[j].q);

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

                println!("Position before:");
                for k in 0..p.len() {
                    print!("{:?} ", p[k].pos[0]);
                }
                println!();
                println!("Force: {:?}", q_force[0]);

                // apply Newton's 2nd law of motion
                let mass_i = p[i].m;
                // let mass_j = p[j].m;
                p[i].a = q_force / mass_i;
                // p[j].a -= q_force / mass_j;
                // let d_i = p[i].a;
                // let d_j = p[j].a * self.delta_t / p[j].m;
                let accel_i = p[i].a;
                // let accel_j = p[j].a;
                p[i].v += accel_i * self.delta_t;
                // p[j].v += accel_j;

                let v_i = p[i].v;
                // let v_j = p[j].v;
                p[i].pos += v_i * self.delta_t;
                // p[j].pos += v_j;

                // println!("Position after:");
                // for k in 0..p.len() {
                //     println!("{:?}", p[k]);
                // }
            }
        }
    }

    pub fn add_atom(self: &mut Self, atom: &atom::Atom) {
        for particle in &atom.electrons {
            self.particles.push(particle.clone());
        }
        self.particles.push(atom.nucleus.clone());
    }
}
