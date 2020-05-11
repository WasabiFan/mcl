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

const VISUALIZER_WIDTH: usize = 200;
const VISUALIZER_HEIGHT: usize = 200;

const LASERSCAN_STDDEV: f64 = 10.;

fn map_point_to_window(point: Point) -> (i32, i32) {
    ((point.x / MAP_WIDTH * VISUALIZER_WIDTH as f64) as i32, (point.y / MAP_HEIGHT * VISUALIZER_HEIGHT as f64) as i32)
}

enum MCLPhase {
    Predict, Update
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = vec![0u8; VISUALIZER_WIDTH * VISUALIZER_HEIGHT * 4];
    let mut window = Window::new(
        "Monte Carlo Localization Demo",
        VISUALIZER_WIDTH,
        VISUALIZER_HEIGHT,
        WindowOptions::default(),
    )?;

    let map = Map { width: MAP_WIDTH, height: MAP_HEIGHT };
    let mut true_pose = Pose { location: Point { x: MAP_WIDTH / 2., y: MAP_HEIGHT / 2.} };
    let mut mcl = LocalizationParticleFilter::new(500, &map);
    let mut next_phase: MCLPhase = MCLPhase::Predict;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        if let Some(keys) = window.get_keys_pressed(KeyRepeat::No) {
            for key in keys {
                match key {
                    Key::Space => {
                        next_phase = match next_phase {
                            MCLPhase::Predict => {
                                mcl.predict(OdometryMeasurement { vx: 0.1, vy: 0.0, sigma: 0.05 });
                                true_pose.location += (0.1, 0.);
                                MCLPhase::Update
                            }
                            MCLPhase::Update => {
                                mcl.update(&LaserScanMeasurement::simulated_measure_from(&true_pose, &map, LASERSCAN_STDDEV));
                                MCLPhase::Predict
                            }
                        }
                    }
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

        for particle in &mcl.particles {
            let (x, y) = map_point_to_window(particle.location);
            root.draw(&Circle::new((x, y), 1, ShapeStyle::from(&RED).filled()))?;
        }
        root.draw(&Circle::new(map_point_to_window(true_pose.location), 4, ShapeStyle::from(&GREEN).filled()))?;
        drop(root);
        window.update_with_buffer(unsafe { std::mem::transmute(&buf[..]) }, VISUALIZER_WIDTH, VISUALIZER_HEIGHT)?;
    }

    Ok(())
}