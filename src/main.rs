//TODO : change this file into lib.rs once developped
use crate::data::alias::JointValues;
use crate::data::kkconvention::JointGeometry;

pub mod data;
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
