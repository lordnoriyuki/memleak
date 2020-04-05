use winit;

fn main() {
	let winit_event_loop = winit::event_loop::EventLoop::new();
	let logical_size = winit::dpi::LogicalSize::new(400, 400);
	let window = winit::window::WindowBuilder::new()
			.with_inner_size(logical_size)
			.build(&winit_event_loop).unwrap();

    let surface = wgpu::Surface::create(&window);

    let (adapter, device, queue) = futures::executor::block_on(create_device());
    
    let mut swap_chain = device.create_swap_chain(
        &surface,
        &wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8Unorm,
            width: 400,
            height: 400,
            present_mode: wgpu::PresentMode::Immediate,
        },
    );

    let mut count = 0;

    winit_event_loop.run(move |event, _, control_flow| {
		*control_flow = winit::event_loop::ControlFlow::Poll;
		match event {
			winit::event::Event::MainEventsCleared => {
				*control_flow = winit::event_loop::ControlFlow::Poll;
                window.request_redraw();
			},
			winit::event::Event::RedrawRequested(_) => {
                let frame = swap_chain.get_next_texture().unwrap();
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
          
                let vertices = vec![0u8; 2_000_000 + (count % 500) * 1000];
                device.create_buffer_with_data(&vertices, wgpu::BufferUsage::VERTEX);
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                      attachment: &frame.view,
                      resolve_target: None,
                      load_op: wgpu::LoadOp::Clear,
                      store_op: wgpu::StoreOp::Store,
                      clear_color: wgpu::Color { r: 0.0, g: 0.0, b: 0.5, a: 1.0 },
                    }],
                    depth_stencil_attachment: None,
                });
        
                queue.submit(&[encoder.finish()]);
        
                count += 1;
            },
			winit::event::Event::WindowEvent { event, .. } => match event {
				winit::event::WindowEvent::CloseRequested => {
					*control_flow = winit::event_loop::ControlFlow::Exit
				}
				_ => {}
			},
			_ => {}
        };
    })
}

async fn create_device() -> (wgpu::Adapter, wgpu::Device, wgpu::Queue) {
    let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
      power_preference: wgpu::PowerPreference::HighPerformance,
      compatible_surface: None,
    }, wgpu::BackendBit::PRIMARY).await.unwrap();

    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
      extensions: wgpu::Extensions {
        anisotropic_filtering: false,
      },
      limits: wgpu::Limits::default(),
    }).await;

    (adapter, device, queue)
  }
