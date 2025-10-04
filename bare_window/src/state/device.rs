use anyhow::Error;

pub(crate) async fn request_device(adapter: &wgpu::Adapter) -> Result<(wgpu::Device, wgpu::Queue), Error> {
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            // WebGL doesn't support all the features of wgpu, so if
            // we're building for the web, we'll have to disable some.
            required_limits: if cfg!(target_arch = "wasm32") {
                wgpu::Limits::downlevel_webgl2_defaults()
            } else {
                wgpu::Limits::default()
            },
            memory_hints: Default::default(),
            trace: wgpu::Trace::Off,
        })
        .await?;
    dbg!(&device);
    Ok((device, queue))
}
