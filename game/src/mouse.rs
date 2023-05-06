use fyrox::event::{DeviceEvent, Event};

#[derive(Default, Debug)]
pub struct Mouse {
    pub vertical_axis: f32,
    pub horizontal_axis: f32,

}

impl Mouse {
    pub fn new(pitch: f32, yaw: f32) -> Self {
        Self { vertical_axis: pitch, horizontal_axis: yaw }
    }

    pub fn process_mouse_event(&mut self, event: &Event<()>) {
        match event {
            Event::DeviceEvent { event, .. } => {
                if let DeviceEvent::MouseMotion { delta } = event {
                    self.horizontal_axis -= delta.0 as f32;
                    self.vertical_axis += delta.1 as f32;
                }
            }
            _ => {}
        }
    }
}