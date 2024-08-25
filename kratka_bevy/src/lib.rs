use std::ops::{Add, Sub};

use bevy::app::App;
use bevy::math::Vec2;
use bevy::prelude::*;

pub mod grid_layout;

#[derive(Component)]
pub struct GridSprite;

#[derive(Component, Reflect, Default, Debug, Copy, Clone, PartialEq)]
#[reflect(Component)]
pub struct GridPosition {
    pub coordinates: Vec2, // the full-square coordinates on the whole grid
    pub offset: Vec2,      // the offset within a single grid cell
}

impl GridPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            coordinates: Vec2::new(x, y),
            offset: Vec2::ZERO,
        }
    }

    pub fn from_ivec(ivec: &IVec2) -> Self {
        Self {
            coordinates: Vec2::new(ivec.x as f32, ivec.y as f32),
            offset: Vec2::ZERO,
        }
    }

    pub fn direction_to(&self, other: &Self) -> Vec2 {
        if other == self {
            return Vec2::ZERO;
        }
        (other.coordinates + other.offset) - (self.coordinates + self.offset)
    }

    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }

    /// If the offset is more than a whole cell, then update the coordinates (and bring the offset back within 0..1)
    pub fn fix_offset_overflow(&mut self) {
        if self.offset.x >= 0.5 {
            self.coordinates.x += 1.;
            self.offset.x -= 1.;
        }
        if self.offset.y >= 0.5 {
            self.coordinates.y += 1.;
            self.offset.y -= 1.;
        }
        if self.offset.x < -0.5 {
            self.coordinates.x -= 1.;
            self.offset.x += 1.;
        }
        if self.offset.y < -0.5 {
            self.coordinates.y -= 1.;
            self.offset.y += 1.;
        }
    }

    /// retrieves current coordinate and offset
    pub fn get_values(&self) -> (Vec2, Vec2) {
        (self.coordinates, self.offset)
    }

    /// sets the position to a fixed coordinate and offset
    pub fn set(&mut self, coordinates: Vec2, offset: Vec2) {
        self.coordinates = coordinates;
        self.offset = offset;
    }
}

impl Sub for GridPosition {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Self {
            coordinates: self.coordinates.sub(rhs.coordinates),
            offset: self.offset.sub(rhs.offset),
        };
        res.fix_offset_overflow();
        res
    }
}

impl Add for GridPosition {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Self {
            coordinates: self.coordinates.add(rhs.coordinates),
            offset: self.offset.add(rhs.offset),
        };
        res.fix_offset_overflow();
        res
    }
}


