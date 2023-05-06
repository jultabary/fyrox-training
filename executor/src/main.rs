//! Executor with your game connected to it as a plugin.
use std::sync::Arc;
use std::time::Instant;
use fyrox::asset::manager::ResourceManager;
use fyrox::dpi::{LogicalPosition, LogicalSize, Position, Size};
use fyrox::engine::{Engine, EngineInitParams, GraphicsContextParams, SerializationContext};
use fyrox::event::{Event, VirtualKeyCode, WindowEvent};
use fyrox::event_loop::{ControlFlow, EventLoop};
use fyrox::window::{WindowAttributes};
use getting_stared::Game;


// Our game logic will be updated at 60 Hz rate.
const TIMESTEP: f32 = 1.0 / 60.0;


fn main() {
    let event_loop = EventLoop::new();
    let graphics_context_params = GraphicsContextParams {
        window_attributes: WindowAttributes {
            title: "3D Shooter Tutorial".to_string(),
            resizable: true,
            visible: true,
            position: Some(Position::Logical(LogicalPosition::new(0.0, 0.0))),
            inner_size: Some(Size::Logical(LogicalSize::new(800.0, 600.0))),
            ..Default::default()
        },
        vsync: true,
    };
    let serialization_context = Arc::new(SerializationContext::new());
    let mut engine = Engine::new(EngineInitParams {
        graphics_context_params,
        resource_manager: ResourceManager::new(),
        serialization_context,
    }).unwrap();
    let mut engine_initialized = false;
    let mut game = fyrox::core::futures::executor::block_on(Game::new(&mut engine));
    let mut previous = Instant::now();
    let mut lag = 0.0;
    event_loop.run(move |event, window_target, control_flow| {
        game.player.process_input_event(&event);
        match event {
            Event::MainEventsCleared => {
                let elapsed = previous.elapsed();
                previous = Instant::now();
                lag += elapsed.as_secs_f32();
                while lag >= TIMESTEP {
                    lag -= TIMESTEP;

                    // Run our game's logic.
                    game.update(&mut engine);

                    // Update engine each frame.
                    engine.update(TIMESTEP, control_flow, &mut lag, Default::default());
                }
                if engine_initialized == false {
                    let error = engine.initialize_graphics_context(window_target);
                    if error.is_ok() {
                        engine_initialized = true;
                    }
                }
                if engine_initialized == true {
                    let graphic_context = engine.graphics_context.as_initialized_ref();
                    graphic_context.window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                // Render at max speed - it is not tied to the game code.
                engine.render().unwrap();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    // Exit game by hitting Escape.
                    if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                        *control_flow = ControlFlow::Exit
                    }
                }
                WindowEvent::Resized(size) => {
                    engine.set_frame_size(size.into()).unwrap();
                }
                _ => (),
            }
            _ => *control_flow = ControlFlow::Poll,
        }
    });
}