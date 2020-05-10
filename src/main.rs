extern crate nalgebra as na;

use na::{Vector2, Point2, Isometry2};
use std::f64::consts;
use nphysics2d::object::{BodyStatus, RigidBodyDesc};
use nphysics2d::math::{Velocity, Inertia};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use ncollide2d::shape::{ShapeHandle, Ball};
use ncollide2d::world;
use nphysics2d::object::ColliderDesc;
use nphysics2d::material::{MaterialHandle, BasicMaterial};



fn main() {
    let mut mechanical_world = DefaultMechanicalWorld::new(Vector2::new(0.0, -9.81));
    let mut geometrical_world = DefaultGeometricalWorld::new();
    let mut bodies = DefaultBodySet::new();
    let mut colliders = DefaultColliderSet::new();
    let mut joint_constraints = DefaultJointConstraintSet::new();
    let mut force_generators = DefaultForceGeneratorSet::new();

    let rigid_body = RigidBodyDesc::new()
        // The rigid body translation.
        // Default: zero vector.
        .translation(Vector2::y() * 5.0)
        // The rigid body rotation.
        // Default: no rotation.
        .rotation(5.0)
        // The rigid body position. Will override `.translation(...)` and `.rotation(...)`.
        // Default: the identity isometry.
        .position(Isometry2::new(Vector2::new(1.0, 2.0), consts::PI))
        // Whether or not this rigid body is affected by gravity.
        // Default: true
        .gravity_enabled(false)
        // The status of this rigid body.
        // Default: BodyStatus::Dynamic
        .status(BodyStatus::Kinematic)
        // The velocity of this body.
        // Default: zero velocity.
        .velocity(Velocity::linear(1.0, 2.0))
        // The linear damping applied to this rigid body velocity to slow it down automatically.
        // Default: zero (no damping at all).
        .linear_damping(10.0)
        // The angular damping applied to this rigid body velocity to slow down its rotation automatically.
        // Default: zero (no damping at all).
        .angular_damping(5.0)
        // The maximum linear velocity this rigid body can reach.
        // Default: f32::max_value() or f64::max_value() (no limit).
        .max_linear_velocity(10.0)
        // The maximum angular velocity this rigid body can reach.
        // Default: f32::max_value() or f64::max_value() (no limit).
        .max_angular_velocity(1.7)
        // The angular inertia tensor of this rigid body, expressed on its local-space.
        // Default: the zero matrix.
        //.angular_inertia(3.0)
        // The rigid body mass.
        // Default: 0.0
        .mass(1.2)
        // The mass and angular inertia of this rigid body expressed in
        // its local-space. Default: zero.
        // Will override previous calls to `.mass(...)` and `.angular_inertia(...)`.
        .local_inertia(Inertia::new(1.0, 3.0))
        // The center of mass of this rigid body expressed in its local-space.
        // Default: the origin.
        .local_center_of_mass(Point2::new(1.0, 2.))
        // The threshold for putting this rigid body to sleep.
        // Default: Some(ActivationStatus::default_threshold())
        .sleep_threshold(None)
        // The translations that will be locked for this rigid body.
        // Default: nothing is locked (false everywhere).
        .kinematic_translations(Vector2::new(true, false))
        // The translations that will be locked for this rigid body.
        // Default: nothing is locked (false everywhere).
        .kinematic_rotations(true)
        // Whether this rigid body motion should be interpolated linearly during CCD resolution.
        // Default: false (which implies non-linear interpolation)
        //.enable_linear_motion_interpolation(true)
        // Arbitrary user-defined data associated to the rigid body to be built.
        // Default: no associated data
        .user_data(10)
        // All done, actually build the rigid-body.
        .build();    let shape = ShapeHandle::new(Ball::new(1.5));

    let mut rb_desc = RigidBodyDesc::new()
        .mass(1.2);

// Build a first rigid body.
    let rigid_body1 = rb_desc.build();

// Change the rigid body translation and velocity before building
// another one. It will still have the same mass and rotation as
// initialized above.
    let mut rigid_body2 = rb_desc.set_translation(Vector2::new(2.0, 1.0))
        .set_velocity(Velocity::linear(1.0, 3.0))
        .build();
    rigid_body2.set_mass(10.0);
    let mut body_set = DefaultBodySet::new();
    let handle = body_set.insert(rigid_body);
    println!("built");
    loop {
        // Run the simulation.
        println!("running");
        mechanical_world.step(
            &mut geometrical_world,
            &mut bodies,
            &mut colliders,
            &mut joint_constraints,
            &mut force_generators
        )
    }}