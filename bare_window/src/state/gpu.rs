use std::sync::Arc;
use anyhow::Error;
use wgpu::{Adapter, Surface, TextureFormat};
use wgpu::wgt::SurfaceConfiguration;
use winit::window::Window;

pub(crate) fn init_gpu<'a>(window: Arc<Window>) -> Result<(Surface<'a>, Adapter, SurfaceConfiguration<Vec<TextureFormat>>), Error> {
let size = window.clone().inner_size();
let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
    #[cfg(not(target_arch = "wasm32"))]
    backends: wgpu::Backends::PRIMARY,
    #[cfg(target_arch = "wasm32")]
    backends: wgpu::Backends::GL,
    ..Default::default()
});
dbg!(&instance);
let surface = instance.create_surface(window.clone())?;

dbg!(&surface);

let adapter = pollster::block_on(instance
    .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }))?;
dbg!(&adapter);
let surface_caps = surface.get_capabilities(&adapter);
// Shader code in this tutorial assumes an sRGB surface texture. Using a different
// one will result in all the colors coming out darker. If you want to support non
// sRGB surfaces, you'll need to account for that when drawing to the frame.
dbg!(&surface_caps);
let surface_format = surface_caps
    .formats
    .iter()
    .find(|f| f.is_srgb())
    .copied()
    .unwrap_or(surface_caps.formats[0]);
let config = wgpu::SurfaceConfiguration {
    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    format: surface_format,
    width: size.width,
    height: size.height,
    present_mode: surface_caps.present_modes[0],
    alpha_mode: surface_caps.alpha_modes[0],
    view_formats: vec![],
    desired_maximum_frame_latency: 2,
};
Ok((surface, adapter, config))
}
