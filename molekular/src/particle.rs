use crate::simulation;use kiss3d::nalgebra::{Vector3, Matrix3x1};
use std::clone::Clone;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::Copy;
use std::cell::RefCell;

// artificially scale the particles up, so they can be seen when far apart
const PARTICLE_RADIUS_FACTOR: f64 = 1e-2;

// According to: https://en.wikipedia.org/wiki/Proton
const PROTON_RENDER_RADIUS: f64 = 2.83e-10 * PARTICLE_RADIUS_FACTOR;

// According to: https://en.wikipedia.org/wiki/Classical_electron_radius
const ELECTRON_RENDER_RADIUS: f64 = 0.81794e-10 * PARTICLE_RADIUS_FACTOR;

#[derive(Debug, Copy, Clone)]
pub enum SubatomicParticleType {
    Proton = 1,
    Neutron = 2,
    Electron = 3,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ParticleProperty {
    Position,
    Velocity,
    Accelleration
}

#[derive(Copy, Clone)]
pub enum ParticleVector {
    X = 0,
    Y,
    Z
}

#[derive(Debug, Clone)]
pub struct Particle {
    pub pos: Vector3<f64>, // current Euclidean XYZ position [m]
    pub v: Vector3<f64>,   // current Euclidean XYZ accelleration [m/s]
    pub a: Vector3<f64>,   // current Euclidean XYZ accelleration [m/s^2]
    pub m: f64,            // particle mass [kg]
    pub r: f64,            // radius of the particle [m]
    pub e: f64,            // potential energy [kcal/mol]
    pub q: f64,            // electrical charge [C]
    pub particle_type: SubatomicParticleType,

    data_trace: HashMap<ParticleProperty, RefCell<Vec<Matrix3x1<f64>>>>
}

impl Particle {
    pub fn get_radius(particle_type: SubatomicParticleType) -> f64 {
        match particle_type {
            SubatomicParticleType::Proton | SubatomicParticleType::Neutron => PROTON_RENDER_RADIUS,
            SubatomicParticleType::Electron => ELECTRON_RENDER_RADIUS,
        }
    }

    pub fn get_color(particle_type: SubatomicParticleType) -> (f64, f64, f64) {
        match particle_type {
            SubatomicParticleType::Proton | SubatomicParticleType::Neutron => (1.0, 1.0, 1.0),
            SubatomicParticleType::Electron => (0.3, 0.3, 1.0),
        }
    }

    pub fn new(particle_type: SubatomicParticleType, mass: f64, charge: f64) -> Self {
        Self {
            particle_type: particle_type,
            v: Vector3::default(),
            a: Vector3::default(),
            m: mass,
            r: Particle::get_radius(particle_type),
            pos: Vector3::default(),
            e: 0.0,
            q: charge,
            data_trace: HashMap::new()
        }
        
    }
    pub fn create_electron(location: Vector3<f64>) -> Self {
        Self {
            particle_type: SubatomicParticleType::Electron,
            // v: Vector3::new(0.05e-15, 0.05e-15, 0.05e-15),
            v: Vector3::new(0.0, 0.0, 0.0),
            a: Vector3::new(0.0, 0.0, 0.0),
            // a: Vector3::new(1.0, 1.0, 1.0),
            m: 9.1093837015e-31,
            r: Particle::get_radius(SubatomicParticleType::Electron),
            pos: location,
            e: 0.0,
            q: -1.6e-19,
            data_trace: HashMap::new()
        }
    }

    pub fn create_proton(location: Vector3<f64>) -> Self {
        Self {
            particle_type: SubatomicParticleType::Proton,
            v: Vector3::default(),
            a: Vector3::default(),
            m: 1.67262192369e-27,
            r: Particle::get_radius(SubatomicParticleType::Proton),
            pos: location,
            e: 0.0,
            q: -1.6e-19,
            data_trace: HashMap::new()
        }
    }

    pub fn log_debug_data(&mut self) {

        let log_trace_fn = |s: &mut Particle, x: &ParticleProperty| {
            let v = s.get_property(x);
            match s.data_trace.get(x) {
                Some(trace) => { 
                    let mut t = trace.borrow_mut();
                    t.push(v);
                },
                None => { s.data_trace.insert(x.clone(), RefCell::new(vec![v])); }
            }
        };

        log_trace_fn(self, &ParticleProperty::Accelleration);
        log_trace_fn(self, &ParticleProperty::Position);
        log_trace_fn(self, &ParticleProperty::Velocity);
    }

    pub fn get_property(&self, x: &ParticleProperty) -> Matrix3x1<f64> {
        match x {
            ParticleProperty::Position => self.pos,
            ParticleProperty::Accelleration => self.a,
            ParticleProperty::Velocity => self.v
        }
    }

    pub fn get_log_size(&self) -> usize {
        self.data_trace.get(&ParticleProperty::Position).unwrap().borrow().len()
    }

    pub fn get_debug_data(&self, prop: &ParticleProperty, vec: ParticleVector) -> Vec<f64> {
        let res: Vec<f64> = self.data_trace.get(prop).unwrap().borrow().iter().map(|x| {
            x[vec as usize]
        }).collect();
        res
    }

    // pub fn setPosition()
}
