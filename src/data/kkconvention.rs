use super::alias::Transform;
use nalgebra::Vector3;

/// Describe the geometry of a joint, based on Khalil Kleinfinger-convention.
/// Between two joints j-1 and j, the transformation matrix from frame R_j-1 to frame R_j
/// is expressed as a function of four geometrics parameters : α, d, Θ and r.
/// Θ and r can be variable depending of the type of joint
#[derive(Debug, Copy, Clone)]
pub enum JointGeometry {
    /// Joint with distance r variable
    Prismatic {
        /// Angle between z_j-1 and z_j about x_j-1
        alpha: f64,
        /// Distance between z_j-1 and z_j along x_j-1
        d: f64,
        /// Angle between x_j-1 and x_j about z_j
        theta: f64,
    },
    /// Joint with angle Θ variable
    Revolute {
        /// Angle between z_j-1 and z_j about x_-1
        alpha: f64,
        /// Distance between z_j-1 and z_j along x_j-1
        d: f64,
        /// Distance between x_j-1 and x_j along z_j
        r: f64,
    },
}

impl JointGeometry {
    /// Compute the transformation matrix between previous joint and asked joint
    /// based on the associated joint geometry.
    pub fn compute_transform(&self, joint_value: f64) -> Transform {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_transform_zeroes() {
        let prismatic_zeros = JointGeometry::Prismatic {
            alpha: 0.,
            d: 0.,
            theta: 0.,
        };
        let revolute_zeros = JointGeometry::Revolute {
            alpha: 0.,
            d: 0.,
            r: 0.,
        };
        let transform_identity = Transform::identity();
        assert_eq!(prismatic_zeros.compute_transform(0.), transform_identity);
        assert_eq!(revolute_zeros.compute_transform(0.), transform_identity);
    }
    // TODO : test prismatic and revolute properly
}
