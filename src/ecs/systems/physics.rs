use specs::prelude::*;
use specs::{System, Write, ReaderId, ReadStorage, Storage, BitSet};
use specs::shrev::EventIterator;
use crate::ecs::resources::*;
use crate::ecs::components::*;
use specs::prelude::ComponentEvent;
use nphysics3d::object::RigidBodyDesc;
use nalgebra::{Isometry3, Isometry, Translation3, UnitQuaternion, Point3};
use crate::utils::ToVec3;
use nalgebra_glm::{Vec3, vec3};

fn iterate_component_events(events: EventIterator<ComponentEvent>) -> (BitSet, BitSet, BitSet) {
    let mut inserted = BitSet::new();
    let mut modified = BitSet::new();
    let mut removed = BitSet::new();

    for event in events {
        match event {
            ComponentEvent::Inserted(id) => inserted.add(*id),
            ComponentEvent::Modified(id) => modified.add(*id),
            ComponentEvent::Removed(id) => removed.add(*id),
        };
    }
    (inserted, modified, removed)
}

pub struct PhysicsStepperSystem;

impl<'a> System<'a> for PhysicsStepperSystem {
    type SystemData = (Write<'a, PhysicsWorld>);

    fn run(&mut self, mut pw: Self::SystemData) {
        pw.world.step();
    }
}

pub struct SyncBodiesToPhysicsSystem {
    pub transforms_reader_id: ReaderId<ComponentEvent>,
    pub rigidbodies_reader_id: ReaderId<ComponentEvent>,
}

impl<'a> System<'a> for SyncBodiesToPhysicsSystem {
    type SystemData = (ReadStorage<'a, Transform>,
                       ReadStorage<'a, Rigidbody>,
                       Write<'a, PhysicsWorld>);

    fn run(&mut self, (transforms, rigidbodies, mut physics): Self::SystemData) {
        let (inserted_rb, modified_rb, removed_rb) =
            iterate_component_events(rigidbodies.channel().read(&mut self.rigidbodies_reader_id));

        let (_, modified_tr, _) =
            iterate_component_events(transforms.channel().read(&mut self.transforms_reader_id));


        // Inserted rigidbodies
        for (transform, rigidbody, id) in (&transforms, &rigidbodies, &inserted_rb).join() {
            let transform = transform as &Transform;
            let rigidbody = rigidbody as &Rigidbody;

            // remove already existing bodies for this inserted component;
            // this technically should never happen but we need to keep the list of body
            // handles clean
            physics.body_handles.remove(&id);

            let body = RigidBodyDesc::new()
                .translation(transform.position)
                .rotation(transform.rotation)
                .mass(rigidbody.mass)
                .user_data(id)
                .local_center_of_mass(Point3::new(0.0, 60.0 * 0.05, 0.0))
                .build(&mut physics.world);

            body.set_linear_velocity((10.0, 30.0, 0.0).to_vec3());
            body.set_angular_velocity(vec3(1.0, 2.0, 3.0));

            let handle = body.handle();

            physics.body_handles.insert(id, handle);
        }

        // Modified rigidbodies
        for (rigidbody, id) in (&rigidbodies, &modified_rb).join() {
            let rigidbody = rigidbody as &Rigidbody;

            let handle = physics.body_handles.get(&id).unwrap().clone();
            if let Some(body) = physics.world.rigid_body_mut(handle) {
                body.set_mass(rigidbody.mass);
            }
        }

        // Modified transforms
        for (transform, id) in (&transforms, &modified_tr).join() {
            let transform = transform as &Transform;

            if let Some(handle) = physics.body_handles.get(&id).cloned() {
                if let Some(body) = physics.world.rigid_body_mut(handle) {
                    body.set_position(Isometry::from_parts(
                        transform.position.into(),
                        UnitQuaternion::from_euler_angles(
                            transform.rotation.x,
                            transform.rotation.y,
                            transform.rotation.z,
                        ),
                    ));
                }
            }
        }

        // Removed rigidbodies
        for (rigidbody, id) in (&rigidbodies, &removed_rb).join() {
            let rigidbody = rigidbody as &Rigidbody;

            if let Some(handle) = physics.body_handles.remove(&id) {
                physics.world.remove_bodies(&[handle]);
                println!("Removed rigid body from world with id: {}", id);
            }
        }
    }
}

pub struct SyncBodiesFromPhysicsSystem;

impl<'a> System<'a> for SyncBodiesFromPhysicsSystem {
    type SystemData = (WriteStorage<'a, Transform>,
                       ReadStorage<'a, Rigidbody>,
                       Read<'a, PhysicsWorld>,
                       Entities<'a>);

    fn run(&mut self, (mut transforms, rigidbodies, physics, entities): Self::SystemData) {
        for (transform, rigidbody, e) in (&mut transforms, &rigidbodies, &entities).join() {
            let transform = transform as &mut Transform;
            let rigidbody = rigidbody as &Rigidbody;

            if let Some(handle) = physics.body_handles.get(&e.id()).cloned() {
                if let Some(rigidbody) = physics.world.rigid_body(handle) {
                    let iso = rigidbody.position();
                    transform.position = iso.translation.vector;
                    let a = iso.rotation as UnitQuaternion<f32>;
                    transform.rotation = a.euler_angles().to_vec3();
                }
            }
        }
    }
}