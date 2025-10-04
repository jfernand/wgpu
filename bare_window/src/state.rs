use crate::state::gpu::init_gpu;
use crate::primitives::triangle::VERTICES;
use std::sync::Arc;
use wgpu::Surface;
use wgpu::util::DeviceExt;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use winit::window::Window;
use crate::primitives::Primitive;
use crate::primitives::triangle::Triangle;

mod device;
mod gpu;
mod pipeline;
mod render_pass;

pub struct State<'a> {
    color: wgpu::Color,
    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    is_surface_configured: bool,
    num_vertices: usize,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    surface: Surface<'a>,
    vertex_buffer: wgpu::Buffer,
    pub window: Arc<Window>,
}

impl<'a> State<'a> {
    // We don't need this to be async right now,
    // but we will in the next tutorial
    pub async fn new(window: Arc<Window>) -> anyhow::Result<State<'a>> {
        let (surface, adapter, config) = init_gpu(window.clone())?;
        let (device, queue) = device::request_device(&adapter).await?;
        let tri = Triangle::new(VERTICES);
        let render_pipeline = pipeline::make_pipeline(&device, config.format);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(tri.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        Ok(Self {
            color: wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            },
            config,
            device,
            is_surface_configured: false,
            num_vertices: tri.len(),
            queue,
            render_pipeline,
            surface,
            vertex_buffer,
            window,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }
        let (output, encoder) = render_pass::render_pass(
            &self.surface,
            &self.device,
            &self.render_pipeline,
            self.vertex_buffer.slice(..),
            self.num_vertices as u32,
            self.color,
        )?;

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub(crate) fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if let (KeyCode::Escape, true) = (code, is_pressed) {
            event_loop.exit()
        }
    }

    pub fn handle_cursor(&mut self, _x: f64, _y: f64) {}
}
