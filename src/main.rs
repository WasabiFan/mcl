mod resample;
mod geometry;
mod stats;
mod localization;

use geometry::Point;
use localization::{
    LocalizationParticleFilter, 
    Map,
    OdometryMeasurement, 
    LaserScanMeasurement, 
    Pose
};

use std::error::Error;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use plotters::drawing::bitmap_pixel::BGRXPixel;
use plotters::element::*;
use plotters::prelude::*;

// map in meters
const MAP_WIDTH: f64 = 10.;
const MAP_HEIGHT: f64 = 10.;

const VISUALIZER_WIDTH: usize = 600;
const VISUALIZER_HEIGHT: usize = 600;

const LASERSCAN_STDDEV: f64 = 0.2;

fn map_point_to_window(point: Point) -> (i32, i32) {
    ((point.x / MAP_WIDTH * VISUALIZER_WIDTH as f64) as i32, (point.y / MAP_HEIGHT * VISUALIZER_HEIGHT as f64) as i32)
}

struct VisualizerApp {
    map: Map,
    true_pose: Pose,
    mcl: LocalizationParticleFilter,
}

impl VisualizerApp {
    pub fn new() -> VisualizerApp {
        let map = Map { width: MAP_WIDTH, height: MAP_HEIGHT };
        VisualizerApp {
            map,
            true_pose: Pose { location: Point { x: MAP_WIDTH / 2., y: MAP_HEIGHT / 2.} },
            mcl: LocalizationParticleFilter::new(500, &map)
        }
    }

    pub fn main(&mut self) -> Result<(), Box<dyn Error>> {
        let mut buf = vec![0u8; VISUALIZER_WIDTH * VISUALIZER_HEIGHT * 4];
        let mut window = Window::new(
            "Monte Carlo Localization Demo",
            VISUALIZER_WIDTH,
            VISUALIZER_HEIGHT,
            WindowOptions::default(),
        )?;

        while window.is_open() && !window.is_key_down(Key::Escape) {
            if let Some(keys) = window.get_keys_pressed(KeyRepeat::No) {
                for key in keys {
                    match key {
                        Key::Left  => self.apply_velocity_step(-0.1, 0.),
                        Key::Right => self.apply_velocity_step(0.1, 0.),
                        Key::Up    => self.apply_velocity_step(0., -0.1),
                        Key::Down  => self.apply_velocity_step(0., 0.1),
                        _ => {
                            continue;
                        }
                    }
                }
            }

            let root =
            BitMapBackend::<BGRXPixel>::with_buffer_and_format(&mut buf[..], (VISUALIZER_WIDTH as u32, VISUALIZER_HEIGHT as u32))?
                .into_drawing_area();
            root.fill(&BLACK)?;

            for particle in &self.mcl.particles {
                let (x, y) = map_point_to_window(particle.location);
                root.draw(&Circle::new((x, y), 1, ShapeStyle::from(&RED).filled()))?;
            }

            if let Some(pose_estimate) = self.mcl.get_pose_estimate() {
                root.draw(&Circle::new(
                    map_point_to_window(pose_estimate.location), 
                    4, 
                    ShapeStyle::from(&YELLOW).filled())
                )?;
            }

            root.draw(&Circle::new(
                map_point_to_window(self.true_pose.location), 
                    4, 
                    ShapeStyle::from(&GREEN).filled()
                ))?;

            drop(root);
            window.update_with_buffer(unsafe { std::mem::transmute(&buf[..]) }, VISUALIZER_WIDTH, VISUALIZER_HEIGHT)?;
        }

        Ok(())
    }

    fn apply_velocity_step(&mut self, vx: f64, vy: f64) {
        self.mcl.predict(OdometryMeasurement { vx, vy, sigma: 0.05 });
        self.true_pose.location += (vx, vy);
        self.mcl.update(&LaserScanMeasurement::simulated_measure_from(&self.true_pose, &self.map, LASERSCAN_STDDEV));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = VisualizerApp::new();
    app.main()
}