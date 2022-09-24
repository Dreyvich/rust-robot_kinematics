use super::alias::Transform;
use nalgebra::Vector3;

/// Describe the geometry of a joint, based on Khalil Kleinfinger-convention.
///
/// Geometrics values for a joint j are :
/// - alpha_j : angle between z_j-i and z_j about x_j-1
/// - d_j : distance between z_j-i and z_j along x_j-1
/// - theta_j : angle between x_j-i and x_j about z_j
/// - r_j : distance between x_j-i and x_j along z_j
#[derive(Debug)]
pub enum JointGeometry {
    /// Joint with distance r variable
    Prismatic { alpha: f64, d: f64, theta: f64 },
    /// Joint with angle theta variable
    Revolute { alpha: f64, d: f64, r: f64 },
}

impl JointGeometry {
    pub fn compute_transfo(&self, joint_value: f64) -> Transform {
        let (alphaj, dj, thetaj, rj): (f64, f64, f64, f64);
        match self {
            JointGeometry::Prismatic { alpha, d, theta } => {
                alphaj = *alpha;
                dj = *d;
                thetaj = *theta;
                rj = joint_value;
            }
            JointGeometry::Revolute { alpha, d, r } => {
                alphaj = *alpha;
                dj = *d;
                thetaj = joint_value;
                rj = *r;
            }
        };

        Transform::rotation(Vector3::x() * alphaj)
            * Transform::translation(dj, 0., 0.)
            * Transform::rotation(Vector3::z() * thetaj)
            * Transform::translation(0., 0., rj)
    }
}
