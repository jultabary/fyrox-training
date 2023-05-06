use fyrox::core::algebra::{UnitQuaternion, Vector3};
use fyrox::core::log::Log;
use fyrox::core::pool::Handle;
use fyrox::event::Event;
use fyrox::scene::base::BaseBuilder;
use fyrox::scene::camera::CameraBuilder;
use fyrox::scene::collider::{ColliderBuilder, ColliderShape};
use fyrox::scene::node::Node;
use fyrox::scene::rigidbody::RigidBodyBuilder;
use fyrox::scene::Scene;
use fyrox::scene::transform::TransformBuilder;
use crate::jump::Jump;
use crate::keyboard::Keyboard;
use crate::mouse::Mouse;
use crate::vector::{Position, Velocity};


pub struct Player {
    camera: Handle<Node>,
    rigid_body: Handle<Node>,
    keyboard: Keyboard,
    mouse: Mouse,
    jump: Jump,
}

impl Player {
    pub fn new(scene: &mut Scene) -> Self {
        // Create rigid body with a camera, move it a bit up to "emulate" head.
        let camera = CameraBuilder::new(
            BaseBuilder::new().with_local_transform(
                TransformBuilder::new()
                    .with_local_position(Vector3::new(0.0, 2.0, 0.0))
                    .build(),
            ),
        ).build(&mut scene.graph);
        let current_position = Vector3::new(0.0, 1.0, -1.0);
        let rigid_body_handle = RigidBodyBuilder::new(
            BaseBuilder::new()
                .with_local_transform(
                    TransformBuilder::new()
                        // Offset player a bit.
                        .with_local_position(current_position)
                        .build(),
                )
                .with_children(&[camera,
                    // Add capsule collider for the rigid body.
                    ColliderBuilder::new(BaseBuilder::new())
                        .with_shape(ColliderShape::capsule_y(0.25, 0.2))
                        .build(&mut scene.graph),
                ]),
        )
            // We don't want the player to tilt.
            .with_locked_rotations(true)
            // We don't want the rigid body to sleep (be excluded from simulation)
            .with_can_sleep(false)
            .with_gravity_scale(3.0)
            .build(&mut scene.graph);
        Self {
            camera,
            rigid_body: rigid_body_handle,
            keyboard: Keyboard::new(false, false, false, false, false),
            mouse: Mouse::new(0.0, 0.0),
            jump: Jump::new(),
        }
    }

    pub fn update_player(&mut self, scene: &mut Scene) {
        self.move_position(scene);
        self.update_point_of_view(scene);
    }

    pub fn process_input_event(&mut self, event: &Event<()>) {
        self.keyboard.process_keyboard_event(event);
        self.mouse.process_mouse_event(event);
    }

    fn move_position(&mut self, scene: &mut Scene) {
        let body = scene.graph[self.rigid_body].as_rigid_body_mut();

        let mut velocity = Velocity::new(Vector3::new(0.0, body.lin_vel().y, 0.0));
        let current_position = Position::new(body.global_position());
        self.jump.update_jump(&velocity);
        let can_move = self.jump.can_move();

        if self.keyboard.move_forward && can_move {
            velocity.add_forward(body.look_vector());
        }
        if self.keyboard.move_backward && can_move {
            velocity.sub_backward(body.look_vector());
        }
        if self.keyboard.move_left && can_move {
            velocity.add_left(body.side_vector());
        }
        if self.keyboard.move_right && can_move {
            velocity.add_right(body.side_vector());
        }

        if self.keyboard.jump && self.jump.can_jump(&current_position, &velocity) {
            velocity.add_vertical_up(body.up_vector());
            self.jump.jump(&current_position, &velocity);
        }
        if self.jump.is_jump_initiated() {
            let is_jumping_velocity = self.jump.jumping_velocity();
            if is_jumping_velocity.is_ok() {
                let jumping_velocity = is_jumping_velocity.unwrap();
                velocity = Velocity::new(Vector3::new(jumping_velocity.x(), velocity.y(), jumping_velocity.z()));
            }
        }
        // Finally new linear velocity.
        body.set_lin_vel(velocity.accelerated_vector());
    }

    fn update_point_of_view(&mut self, scene: &mut Scene) {
        let vertical_point_of_view_value = self.mouse.vertical_axis.clamp(-90.0, 90.0);
        scene.graph[self.camera].local_transform_mut().set_rotation(
            UnitQuaternion::from_axis_angle(&Vector3::x_axis(), vertical_point_of_view_value.to_radians()),
        );
        let body = scene.graph[self.rigid_body].as_rigid_body_mut();
        body.local_transform_mut().set_rotation(
            UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.mouse.horizontal_axis.to_radians()));
    }
}