use wgpu::{RenderPassDescriptor, RenderPassDepthStencilAttachment};

use crate::app::App;



pub fn render(app: &mut App) {
    //log::info!("Handling Redraw Request");

    let (surface_state, rs) = match (&app.surface_state, &app.render_state) {
        (Some(x), Some(y)) => (x, y),
        _ => return,
    };
    //log::info!("1");

    // Update camera uniforms
    rs.queue.write_buffer(
        &rs.camera_buffer,
        0,
        bytemuck::cast_slice(&[rs.camera_uniform]),
    );

    //log::info!("2");

    let frame = surface_state
        .surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");

    //log::info!("3");

    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder =
        rs.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: None,
            });
    {
        let mut rpass =
            encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &rs.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        rpass.set_pipeline(&rs.render_pipeline);
        rpass.set_bind_group(0, &rs.camera_bind_group, &[]);
        rpass.set_vertex_buffer(0, rs.vertex_buffer.slice(..));
        rpass.set_vertex_buffer(1, rs.instance_buffer.slice(..));
        rpass.set_index_buffer(rs.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        
        let index_count = rs.model.indices.len() as _;
        let instance_count = rs.instance_count;
        rpass.draw_indexed(0..index_count, 0, 0..instance_count);
    }
    rs.queue.submit(Some(encoder.finish()));
    frame.present();
    surface_state.window.request_redraw();
    //log::info!("-");
}