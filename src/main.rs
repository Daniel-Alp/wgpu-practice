use glfw::{fail_on_errors, Action, Key, Window};

struct State<'a> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: (i32, i32),
    window: &'a mut Window
}

impl<'a> State<'a> {
    
    async fn new(window: &'a mut Window) -> Self
    {
        let size = window.get_framebuffer_size();
        let instance_desc = wgpu::InstanceDescriptor{
            backends: wgpu::Backends::all(), ..Default::default()
        };
        let instance = wgpu::Instance::new(&instance_desc);

        let surface = instance.create_surface(window.render_context()).unwrap();

        let adapter_desc = wgpu::RequestAdapterOptionsBase{
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false
        };
        let adapter = instance.request_adapter(&adapter_desc).await.unwrap();

        let device_desc = wgpu::DeviceDescriptor{
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("Device"),
            memory_hints: wgpu::MemoryHints::default(),
            trace: wgpu::Trace::Off
        };
        let (device, queue) = adapter.request_device(&device_desc).await.unwrap();

        let surface_cap = surface.get_capabilities(&adapter);
        let surface_fmt = surface_cap.formats.iter()
            .find(|fmt| fmt.is_srgb())
            .copied()
            .unwrap_or(surface_cap.formats[0]);
        let config = wgpu::SurfaceConfiguration{
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_fmt,
            width: size.0 as u32,
            height: size.1 as u32,
            present_mode: surface_cap.present_modes[0],
            alpha_mode: surface_cap.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2
        };
        surface.configure(&device, &config);

        Self {
            instance,
            window,
            surface,
            device,
            queue,
            config,
            size,

        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError>
    {
        let drawable = self.surface.get_current_texture()?;
        let img_view_desc = wgpu::TextureViewDescriptor::default();
        let img_view = drawable.texture.create_view(&img_view_desc);
         
        let cmd_encoder_desc = wgpu::CommandEncoderDescriptor{
            label: Some("Render Encoder")
        };
        let mut cmd_encoder = self.device.create_command_encoder(&cmd_encoder_desc);

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &img_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        };
        let render_pass_desc = wgpu::RenderPassDescriptor{
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None
        };

        cmd_encoder.begin_render_pass(&render_pass_desc);
        self.queue.submit(std::iter::once(cmd_encoder.finish()));

        drawable.present();

        Ok(())
    }

    fn resize(&mut self, new_size: (i32, i32))
    {
        if new_size.0 > 0 && new_size.1 > 0 {
            self.size = new_size;
            self.config.width = new_size.0 as u32;
            self.config.height = new_size.1 as u32;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn update_surface(&mut self)
    {
        self.surface = self.instance.create_surface(self.window.render_context()).unwrap();
    }
}

async fn run() 
{
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    let (mut window, receiver) = glfw.create_window(
        1280, 720, "WGPU practice", glfw::WindowMode::Windowed).unwrap();
    
    let mut state = State::new(&mut window).await;

    state.window.set_framebuffer_size_polling(true);
    state.window.set_pos_polling(true);
    state.window.set_key_polling(true);
    state.window.set_mouse_button_polling(true);

    while !state.window.should_close() {
        glfw.wait_events();
        for (_, event) in glfw::flush_messages(&receiver) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    state.window.set_should_close(true);
                }
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    state.update_surface();
                    state.resize((width, height));
                }
                glfw::WindowEvent::Pos(..) => {
                    state.update_surface();
                    state.resize(state.size);
                }
                _ => {}
            }
        }
        match state.render() {
            Ok(_) => {},
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                state.update_surface();
                state.resize(state.size);
            }
            Err(e) => eprintln!("{:?}", e)
        };
    }
}

fn main() 
{
    pollster::block_on(run());
}