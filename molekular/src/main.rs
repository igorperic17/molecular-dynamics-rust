mod atom;
mod particle;
mod simulation;

extern crate kiss3d;

use kiss3d::light::Light;
use kiss3d::nalgebra::{Matrix, Point2, Point3, Translation3, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use simulation as sim;

pub const SIMULATION_SCALE: f32 = 1e10;

struct AppState {
    nodes: Vec<SceneNode>,
    simulation: simulation::Simulation,
}

impl AppState {
    pub fn new(window: &mut kiss3d::window::Window, simulation: simulation::Simulation) -> Self {
        // init all paticle nodes in the scene
        let mut nodes: Vec<SceneNode> = Vec::<SceneNode>::new();
        for particle in &simulation.particles {
            let mut particle_node = window.add_sphere(
                particle::Particle::get_radius(particle.particle_type) * SIMULATION_SCALE,
            );
            let (r, g, b) = particle::Particle::get_color(particle.particle_type);
            particle_node.set_color(r, g, b);
            particle_node.append_translation(&Translation3::new(
                particle.pos.x * SIMULATION_SCALE,
                particle.pos.y * SIMULATION_SCALE,
                particle.pos.z * SIMULATION_SCALE,
            ));
            nodes.push(particle_node);
        }

        Self {
            nodes: nodes,
            simulation: simulation,
        }
    }
}

impl State for AppState {
    fn step(&mut self, _: &mut Window) {
        self.simulation.step();
        for (i, particle) in self.simulation.particles.iter().enumerate() {
            self.nodes[i].set_local_translation(Translation3::new(
                particle.pos.x * SIMULATION_SCALE,
                particle.pos.y * SIMULATION_SCALE,
                particle.pos.z * SIMULATION_SCALE,
            ));
        }
    }
}

fn main() {
    let steps = -1;
    // let steps = 1000;

    // create a simulation with deltaT = 1fs and temperature = 500 K
    let mut simulation = simulation::Simulation::new(1e-30, 500.0);

    let hydrogen = atom::Atom::create_hidrogen(Vector3::new(0.0, 0.0, 1e-10));
    simulation.add_atom(&hydrogen);

    let window: &mut Window = &mut Window::new("Hydrogen atom simulation");
    window.set_light(Light::StickToCamera);
    let state = &mut AppState::new(window, simulation);

    let mut i = 0;
    while window.render() {
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

        // window.draw_planar_line(
        //     &Point2::new(-100.0, -200.0),
        //     &Point2::new(100.0, -200.0),
        //     &Point3::new(1.0, 1.0, 1.0),
        // );

        i += 1;
    }
}
