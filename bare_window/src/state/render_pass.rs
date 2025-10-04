use wgpu::{Color, CommandEncoder, Device, RenderPipeline, Surface, SurfaceError, SurfaceTexture};

pub(crate) fn render_pass(
    surface: &Surface,
    device: &Device,
    pipeline: &RenderPipeline,
    color: Color,
) -> Result<(SurfaceTexture, CommandEncoder), SurfaceError> {
    let output = surface.get_current_texture()?;
    let view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        // The ops field takes a wgpu::Operations object. This tells wgpu what to do with
        // the colors on the screen (specified by view). The load field tells wgpu how to
        // handle colors stored from the previous frame. Currently, we are clearing the
        // screen with a bluish color. The store field tells wgpu whether we want to store
        // the rendered results to the Texture behind our TextureView (in this case, it's
        // the SurfaceTexture). We use StoreOp::Store as we do want to store our render results.
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(color),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
    });
    render_pass.set_pipeline(&pipeline);
    render_pass.draw(0..3, 0..1);
    drop(render_pass);
    Ok((output, encoder))
}
