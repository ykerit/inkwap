mod application;
mod renderer;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    // !todo()
    // monitor::MonitorHandle,
    window::WindowBuilder,
};

// todo!()
// use application::AppCreator;
use tracing_subscriber;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// todo!()
// fn center_window_postion(monitor: Option<MonitorHandle>, settings: &mut settings::Settings) {
//     if let Some(monitor) = monitor {
//         let monitor_size = monitor.size();
//         let inner_size = settings
//             .initial_window_size
//             .unwrap_or(egui::Vec2 { x: 800.0, y: 600.0 });
//         if monitor_size.width > 0 && monitor_size.height > 0 {
//             let x = (monitor_size.width - inner_size.x as u32) / 2;
//             let y = (monitor_size.height - inner_size.y as u32) / 2;
//             settings.initial_window_postion = Some(egui::Pos2 {
//                 x: x as _,
//                 y: y as _,
//             });
//         }
//     }
// }

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run_app(app_name: &str) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            tracing_subscriber::fmt::init();
        }
    }
    let event_loop = EventLoop::new();
    // center_window_postion(event_loop.available_monitors().next(), &mut settings);
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(450, 400));
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("main")?;
                let canvs = web_sys::Element::from(window.canvas());
                dst.append_child(&canvs).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.")
    }
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            *control_flow = ControlFlow::Wait;
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
