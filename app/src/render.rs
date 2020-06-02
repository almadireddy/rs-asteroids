use asteroids::geometry::Point;
use asteroids::{Asteroid, Blast, Particle, Player};
use std::f64::consts::PI;

use wasm_bindgen::prelude::wasm_bindgen;

#[repr(C)]
#[derive(std::fmt::Debug)]
pub struct Path {
    offset: usize,
    length: usize,
}

#[repr(u8)]
#[derive(std::fmt::Debug)]
pub enum PathEnd {
    Open = 0,
    Closed = 1,
}

#[wasm_bindgen]
#[derive(std::fmt::Debug)]
pub struct PathList {
    paths: Vec<Path>,
    alphas: Vec<f64>,
    ends: Vec<PathEnd>,
    points: Vec<Point>,
}

impl PathList {
    pub fn new() -> Self {
        PathList {
            paths: Vec::new(),
            points: Vec::new(),
            alphas: Vec::new(),
            ends: Vec::new(),
        }
    }

    pub fn push(&mut self, points: &mut Vec<Point>, alpha: f64, end: PathEnd) -> &mut Self {
        if points.is_empty() {
            return self;
        }
        self.paths.push(Path {
            offset: self.points.len(),
            length: points.len(),
        });
        self.alphas.push(alpha);
        self.ends.push(end);
        self.points.append(points);
        self
    }
}

#[wasm_bindgen]
impl PathList {
    pub fn length(&self) -> usize {
        self.paths.len()
    }

    pub fn paths(&self) -> *const Path {
        self.paths.as_ptr()
    }

    pub fn alphas(&self) -> *const f64 {
        self.alphas.as_ptr()
    }

    pub fn ends(&self) -> *const PathEnd {
        self.ends.as_ptr()
    }

    pub fn points_length(&self) -> usize {
        self.points.len()
    }

    pub fn points(&self) -> *const Point {
        self.points.as_ptr()
    }
}

//

pub fn player(player: &Player, list: &mut PathList) {
    list.push(&mut player.hull(), 0.9, PathEnd::Closed);
    list.push(&mut player.interior(), 0.7, PathEnd::Open);
    for (alpha, mut path) in player.exhaust() {
        list.push(&mut path, alpha, PathEnd::Open);
    }
    if let Some(mut shield) = player.shield() {
        list.push(&mut shield, 0.7, PathEnd::Closed);
    }
}

pub fn asteroids(asteroids: &[Asteroid], list: &mut PathList) {
    for asteroid in asteroids.iter() {
        list.push(&mut asteroid.to_path(), 0.5, PathEnd::Closed);
    }
}

pub fn blasts(blasts: &[Blast], list: &mut PathList) {
    for blast in blasts.iter() {
        let (a, b) = blast.endpoints();
        list.push(&mut vec![a, b], 1.0, PathEnd::Open);
    }
}

pub fn particles(particles: &[Particle], list: &mut PathList) {
    for particle in particles.iter() {
        let (a, b) = particle.endpoints();
        let alpha = 0.5 + (0.5 - (particle.rotation() / PI).rem_euclid(1.0)).abs();
        list.push(&mut vec![a, b], alpha, PathEnd::Open);
    }
}

pub fn polylines(polylines: &[Vec<Point>], alpha: f64, list: &mut PathList) {
    for polyline in polylines {
        list.push(&mut polyline.clone(), alpha, PathEnd::Open);
    }
}
