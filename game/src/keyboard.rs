use fyrox::event::{ElementState, Event, VirtualKeyCode, WindowEvent};

#[derive(Default, Debug)]
pub struct Keyboard {
    pub move_forward: bool,
    pub move_backward: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub jump: bool,
}

impl Keyboard {
    pub fn new(move_forward: bool, move_backward: bool, move_left: bool, move_right: bool, jump: bool) -> Self {
        Self { move_forward, move_backward, move_left, move_right, jump }
    }

    pub fn process_keyboard_event(&mut self, event: &Event<()>) {
        match event {
            Event::WindowEvent {
                event, ..
            } => {
                if let WindowEvent::KeyboardInput { input, .. } = event {
                    if let Some(key_code) = input.virtual_keycode {
                        match key_code {
                            VirtualKeyCode::Z => {
                                self.move_forward = input.state == ElementState::Pressed;
                            }
                            VirtualKeyCode::S => {
                                self.move_backward = input.state == ElementState::Pressed;
                            }
                            VirtualKeyCode::Q => {
                                self.move_left = input.state == ElementState::Pressed;
                            }
                            VirtualKeyCode::D => {
                                self.move_right = input.state == ElementState::Pressed;
                            }
                            VirtualKeyCode::Space => {
                                self.jump = input.state == ElementState::Pressed;
                            }
                            _ => (),
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

