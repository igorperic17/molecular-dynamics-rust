use crate::math;
use crate::particle;
use crate::particle::SubatomicParticleType;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Atom {
    pub electrons: Vec<particle::Particle>,
    // nucleus is represented as a single particle with a charge
    // equal to the atomic number * charge of a proton
    pub nucleus: particle::Particle,
}

impl Atom {
    pub fn new(charge: f32) -> Self {
        Self {
            electrons: vec![],
            nucleus: particle::Particle::new(SubatomicParticleType::Neutron, 1e-30, 1e-16),
        }
    }

    pub fn create_hidrogen(location: math::Vec3) -> Self {
        let offset = math::Vec3 {
            x: 0.000000000000001,
            y: 0.0,
            z: 0.0,
        };
        let electron = particle::Particle::create_electron(location + offset);
        Self {
            nucleus: particle::Particle::create_proton(location),
            electrons: vec![electron],
        }
    }

    // pub fn setPosition()
}
