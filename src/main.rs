mod hmap;
mod graphics;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

fn main() {
	let event_loop = EventLoop::new();
	let window = WindowBuilder::new()
		.with_title("tetra")
		.with_inner_size(winit::dpi::LogicalSize::new(512.0, 512.0))
		.build(&event_loop)
		.unwrap();

	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Wait;

		match event {
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				window_id,
			} if window_id == window.id() => *control_flow = ControlFlow::Exit,
			_ => (),
		}
	});
}
