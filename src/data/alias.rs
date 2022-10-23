extern crate nalgebra as na;
use na::{IsometryMatrix3, SVector};

/// Contains values for multiple joints
pub type JointValues<const D: usize> = SVector<f64, D>;
/// Transformation matrix between two frames
pub type Transform = IsometryMatrix3<f64>;
