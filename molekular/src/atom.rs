use crate::particle;
use crate::particle::SubatomicParticleType;
use kiss3d::nalgebra::Vector3;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Atom {
    pub electrons: Vec<particle::Particle>,
    // nucleus is represented as a single particle with a charge
    // equal to the atomic number * charge of a proton
    pub nucleus: particle::Particle,
}

impl Atom {
    pub fn create_hidrogen(location: Vector3<f64>) -> Self {
        // According to: https://www.sciencefocus.com/science/whats-the-distance-from-a-nucleus-to-an-electron/
        // the electron (if it was a particle, hehe) orbits the nucleus at a distance of 1/20 nanometers
        let offset = Vector3::new(0.05e-9, 0.0, 0.0);
        let electron = particle::Particle::create_electron(location + offset);
        Self {
            nucleus: particle::Particle::create_proton(location),
            electrons: vec![electron],
        }
    }

    // pub fn setPosition()
}
