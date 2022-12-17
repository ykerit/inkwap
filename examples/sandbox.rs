use inkwap::canvas::view_style::ViewStyle;
use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(960, 720))
        .build(&event_loop)
        .unwrap();
    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::PhysicalSize;
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                match doc.get_element_by_id("wasm-example") {
                    Some(dst) => {
                        window.set_inner_size(PhysicalSize::new(450, 400));
                        let _ = dst.append_child(&web_sys::Element::from(window.canvas()));
                    }
                    None => {
                        window.set_inner_size(PhysicalSize::new(800, 800));
                        let canvas = window.canvas();
                        canvas.style().set_css_text(
                            "background-color: black; display: block; margin: 20px auto;",
                        );
                        doc.body().and_then(|body| {
                            Some(body.append_child(&web_sys::Element::from(canvas)))
                        });
                    }
                };
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }
    let mut canvas = inkwap::Canvas::new(&window).await.with_style(ViewStyle {
        background_color: wgpu::Color {
            r: 0f64,
            g: 162f64,
            b: 232f64,
            a: 1f64,
        },
        ..Default::default()
    });
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
        Event::RedrawRequested(window_id) if window_id == window.id() => match canvas.render() {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost) => {}
            Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            Err(e) => eprintln!("{:?}", e),
        },
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}

fn main() {
    pollster::block_on(run());
}
