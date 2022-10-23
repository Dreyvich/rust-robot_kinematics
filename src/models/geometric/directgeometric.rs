use crate::data::alias::Transform;
use crate::{JointGeometry, JointValues};

/// Compute the direct geometric model of a joint position based on asked geometry.
/// The result is the transformation matrix from frame R_0 to frame R_(NUMBER_OF_JOINTS)
pub fn compute_direct_geometric<const NUMBER_OF_JOINTS: usize>(
    joint_position: &JointValues<NUMBER_OF_JOINTS>,
    joint_geometry_array: &[JointGeometry; NUMBER_OF_JOINTS],
) -> Transform {
    joint_position
        .iter()
        .zip(joint_geometry_array.iter())
        .fold(Transform::identity(), |accumulator, joint_pair| {
            accumulator * joint_pair.1.compute_transform(*joint_pair.0)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let joint_values_zeros = JointValues::<3>::new(0., 0., 0.);
        let revolute_zeros = JointGeometry::Revolute {
            alpha: 0.,
            d: 0.,
            r: 0.,
        };
        let joint_geometry_array = [
            revolute_zeros.clone(),
            revolute_zeros.clone(),
            revolute_zeros,
        ];
        assert_eq!(
            compute_direct_geometric(&joint_values_zeros, &joint_geometry_array),
            Transform::identity()
        );
    }
    // TODO : test exhaustively
}
