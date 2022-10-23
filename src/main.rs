//!
//! # rust-robot_kinematics
//!
//! rust-robot_kinematics is a library written in rust for high performance (hopefully) robotic
//! algorithms.
//!
//! # Features
//! The library proposes data structures and functions associated with robotic models such as :
//! * Robot geometric description using the Khalil Kleinfinger-convention ['JointGeometry'](JointGeometry)
//! * Direct geometric model computation

//TODO : change this file into lib.rs once developed
use crate::data::alias::JointValues;
use crate::data::kkconvention::JointGeometry;

/// Data structure used in the project
pub mod data;
/// Models for joint space to cartesian space computation and vice versa
pub mod models;

fn main() {
    let jvs = JointValues::<5>::new(1.2, 2.4, 3.6, 4.7, 5.8);
    println!("JVs={:?}", jvs);
    let joint = JointGeometry::Prismatic {
        alpha: 1.,
        d: 2.,
        theta: 3.,
    };
    println!("JGs={:?}", joint);
    let base_t_1 = joint.compute_transform(0.5);
    println!("Transform={:?}", base_t_1);
}
