use log::Level;
use wgpu::Color;
use winit::{dpi::*, event::*, event_loop::*, window::*};

mod renderer;
use renderer::*;

fn main() {
    // initialize logger
    simple_logger::init_with_level(Level::Warn).unwrap();

    // create event loop for windows
    let event_loop = EventLoop::new();

    // create window
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(320, 240))
        .build(&event_loop)
        .unwrap();

    // create renderer
    let mut renderer = Renderer::new(&window, None);

    // create sprite pipeline?
    let sprite_shader = renderer.load_shader_from_memory(include_str!("sprite.wgsl"));
    let sprite_pipeline_layout = renderer.create_pipeline_layout(&[]);
    let sprite_render_pipeline =
        renderer.create_render_pipeline(&sprite_pipeline_layout, &sprite_shader);

    // run event loop
    event_loop.run(move |event, _, control_flow| match event {
        // process window events for current window
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        Event::RedrawRequested(_) => {
            const CORNFLOWER_BLUE: Color = Color {
                r: 100.0 / 255.0,
                g: 149.0 / 255.0,
                b: 237.0 / 255.0,
                a: 1.0,
            };
            renderer.render_pass(CORNFLOWER_BLUE, |render_pass| {
                //render_pass.set_pipeline(&sprite_render_pipeline);
            });
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => (),
    });
}
