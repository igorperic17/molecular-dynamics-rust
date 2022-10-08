mod atom;
mod particle;
mod simulation;
mod plot;

extern crate kiss3d;

use std::cell::RefCell;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Point3, Translation3, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};

use plotters::prelude::*;

pub const SIMULATION_SCALE: f64 = 1e10;

struct AppState {
    nodes: Vec<SceneNode>,
    simulation: RefCell<simulation::Simulation>,
}

impl AppState {
    pub fn new(window: &mut kiss3d::window::Window, simulation: simulation::Simulation) -> Self {
        // init all paticle nodes in the scene
        let mut nodes: Vec<SceneNode> = Vec::<SceneNode>::new();
        for particle in &simulation.particles {
            let mut particle_node = window.add_sphere(
                (particle::Particle::get_radius(particle.particle_type) * SIMULATION_SCALE) as f32,
            );
            let (r, g, b) = particle::Particle::get_color(particle.particle_type);
            particle_node.set_color(r as f32, g as f32, b as f32);
            particle_node.append_translation(&Translation3::new(
                (particle.pos.x * SIMULATION_SCALE) as f32,
                (particle.pos.y * SIMULATION_SCALE) as f32,
                (particle.pos.z * SIMULATION_SCALE) as f32,
            ));
            nodes.push(particle_node);
        }

        Self {
            nodes: nodes,
            simulation: RefCell::new(simulation),
        }
    }
}

impl State for AppState {
    fn step(&mut self, _: &mut Window) {
        self.simulation.borrow_mut().step();
        for (i, particle) in self.simulation.borrow().particles.iter().enumerate() {
            self.nodes[i].set_local_translation(Translation3::new(
                (particle.pos.x * SIMULATION_SCALE) as f32,
                (particle.pos.y * SIMULATION_SCALE) as f32,
                (particle.pos.z * SIMULATION_SCALE) as f32,
            ));
            // let current_scale = self.nodes[i].data().local_scale();
            // self.nodes[i].set_local_scale(1.0, 1.0, 1.0);
            println!("---> NODE POS: {:?}", particle.pos.x);
        }
    }
}

fn main() {
    // let steps = -1;
    let steps = 50;

    // create a simulation with deltaT = 1fs and temperature = 500 K
    let mut simulation = simulation::Simulation::new(1e-9, 500.0);

    let hydrogen =
        atom::Atom::create_hidrogen(Vector3::new(0.0, 0.0, 0.05 * (1.0 / SIMULATION_SCALE)));
    simulation.add_atom(&hydrogen);

    let window: &mut Window = &mut Window::new("Hydrogen atom simulation");
    window.set_light(Light::StickToCamera);
    let state = &mut AppState::new(window, simulation); 

    let mut i = 0;
    loop {
        if steps == -1 || i < steps {
            state.step(window);
        }

        let axis_len = 0.1;
        let zero = Point3::new(0.0, 0.0, 0.0);
        let a = Point3::new(axis_len, 0.0, 0.0);
        let b = Point3::new(0.0, axis_len, 0.0);
        let c = Point3::new(0.0, 0.0, axis_len);

        // window.set_line_width(2.0);
        window.draw_line(&zero, &a, &Point3::new(1.0, 0.0, 0.0));
        window.draw_line(&zero, &b, &Point3::new(0.0, 1.0, 0.0));
        window.draw_line(&zero, &c, &Point3::new(0.0, 0.0, 1.0));

        i += 1;

        if i % 1 == 0 {
            if !window.render() {
                break;
            }
        };
    }

    let plot_data = state.simulation.borrow().particles[0].get_debug_data();
    plot::plot(&plot_data, "/Users/igor/code/rust/molecular-dynamics-rust/molekular/chart.png");
}
