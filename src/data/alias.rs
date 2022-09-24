extern crate nalgebra as na;
use na::{IsometryMatrix3, SVector};

pub type JointValues<const D: usize> = SVector<f64, D>;
pub type Transform = IsometryMatrix3<f64>;
