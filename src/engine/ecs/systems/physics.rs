use specs::prelude::*;
use specs::{System, Write, ReaderId, ReadStorage, BitSet};
use specs::shrev::EventIterator;
use crate::ecs::resources::*;
use crate::ecs::components::*;
use specs::prelude::ComponentEvent;
use nphysics3d::object::{RigidBodyDesc, ColliderDesc, BodyPartHandle};
use nalgebra::{Isometry, UnitQuaternion};
use crate::utils::ToVec3;
use nphysics3d::material::MaterialHandle;
use glfw::ffi::glfwGetTime;

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

pub struct PhysicsStepperSystem {
    tickrate: u32,
    last_time: f64,
}

impl PhysicsStepperSystem {
    pub fn new(physics: &mut nphysics3d::world::World<f32>, tickrate: u32) -> Self {
        let mut stepper = PhysicsStepperSystem {
            tickrate,
            last_time: 0.0
        };

        stepper.set_tickrate(physics, tickrate);
        stepper
    }

    fn set_tickrate(&mut self, physics: &mut nphysics3d::world::World<f32>, tickrate: u32) {
        physics.set_timestep(1.0f32 / tickrate as f32);
    }
}

impl<'a> System<'a> for PhysicsStepperSystem {
    type SystemData = (Write<'a, PhysicsWorld>);

    fn run(&mut self, mut pw: Self::SystemData) {
        let now = unsafe { glfwGetTime() };
        if now - self.last_time >= pw.world.timestep().into() {
            pw.world.step();
            self.last_time = now;
        }
    }
}

pub struct SyncBodiesToPhysicsSystem {
    pub transforms_reader_id: ReaderId<ComponentEvent>,
    pub rigidbodies_reader_id: ReaderId<ComponentEvent>,
}

impl<'a> System<'a> for SyncBodiesToPhysicsSystem {
    type SystemData = (ReadStorage<'a, Transform>,
                       ReadStorage<'a, RigidBody>,
                       Write<'a, PhysicsWorld>);

    fn run(&mut self, (transforms, rigidbodies, mut physics): Self::SystemData) {
        let (inserted_rb, modified_rb, removed_rb) =
            iterate_component_events(rigidbodies.channel().read(&mut self.rigidbodies_reader_id));

        let (_, modified_tr, _) =
            iterate_component_events(transforms.channel().read(&mut self.transforms_reader_id));


        // Inserted rigidbodies
        for (transform, rigid_body, id) in (&transforms, &rigidbodies, &inserted_rb).join() {
            let transform = transform as &Transform;
            let rigid_body = rigid_body as &RigidBody;

            // remove already existing bodies for this inserted component;
            // this technically should never happen but we need to keep the list of body
            // handles clean
            physics.body_handles.remove(&id);

            let body = RigidBodyDesc::new()
                .translation(transform.position)
                .rotation(transform.rotation)
                .name(rigid_body.name.clone())
                .gravity_enabled(rigid_body.gravity_enabled)
                .status(rigid_body.status)
                .velocity(rigid_body.velocity)
                .angular_inertia(rigid_body.angular_inertia)
                .mass(rigid_body.mass)
                .local_center_of_mass(rigid_body.local_center_of_mass)
                .sleep_threshold(rigid_body.sleep_threshold)
                .kinematic_translations(rigid_body.kinematic_translations)
                .kinematic_rotations(rigid_body.kinematic_rotations)
                .user_data(id)
                .build(&mut physics.world);

            let handle = body.handle();
            physics.body_handles.insert(id, handle);
        }

        // Modified rigidbodies
        for (rigid_body, id) in (&rigidbodies, &modified_rb).join() {
            let rigid_body = rigid_body as &RigidBody;

            // TODO Handle all the changes
            let handle = physics.body_handles.get(&id).unwrap().clone();
            if let Some(body) = physics.world.rigid_body_mut(handle) {
                body.set_mass(rigid_body.mass);
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
        for (_rigid_body, id) in (&rigidbodies, &removed_rb).join() {
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
                       ReadStorage<'a, RigidBody>,
                       Read<'a, PhysicsWorld>,
                       Entities<'a>);

    fn run(&mut self, (mut transforms, rigidbodies, physics, entities): Self::SystemData) {
        for (transform, _rigid_body, e) in (&mut transforms, &rigidbodies, &entities).join() {
            let transform = transform as &mut Transform;

            if let Some(handle) = physics.body_handles.get(&e.id()).cloned() {
                if let Some(rigid_body) = physics.world.rigid_body(handle) {
                    let iso = rigid_body.position();
                    transform.position = iso.translation.vector;
                    let a = iso.rotation as UnitQuaternion<f32>;
                    transform.rotation = a.euler_angles().to_vec3();
                }
            }
        }
    }
}

pub struct SyncCollidersToPhysicsSystem {
    pub colliders_reader_id: ReaderId<ComponentEvent>,
}

impl<'a> System<'a> for SyncCollidersToPhysicsSystem {
    type SystemData = (ReadStorage<'a, Transform>,
                       ReadStorage<'a, Collider>,
                       Write<'a, PhysicsWorld>);

    fn run(&mut self, (transforms, colliders, mut physics): Self::SystemData) {
        let (inserted_col, _modified_col, removed_col) =
            iterate_component_events(colliders.channel().read(&mut self.colliders_reader_id));

        // Inserted colliders
        for (_transform, collider, id) in (&transforms, &colliders, &inserted_col).join() {
            let collider = collider as &Collider;

            let body_part_handle = match physics.body_handles.get(&id) {
                Some(handle) => physics
                    .world
                    .rigid_body(*handle)
                    .map_or(BodyPartHandle::ground(), |rb| rb.part_handle()),
                None => BodyPartHandle::ground()
            };

            let collider_handle = ColliderDesc::new(collider.shape.clone())
                .material(MaterialHandle::new(collider.material))
                .build_with_parent(body_part_handle, &mut physics.world)
                .unwrap()
                .handle();

            physics.collider_handles.insert(id, collider_handle);
        }

        // TODO Modified colliders

        // Removed colliders
        for (_, id) in (&colliders, &removed_col).join() {
            if let Some(handle) = physics.collider_handles.remove(&id) {
                if physics.world.collider(handle).is_some() {
                    physics.world.remove_colliders(&[handle]);
                }
                println!("Removed collider from world with id: {}", id);
            }
        }
    }
}