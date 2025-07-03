fn create_render_pipeline(&self) -> RenderPipeline {
    let device = self.device.as_ref().unwrap();
    let surface = self.surface.as_ref().unwrap();
    // surface.getc

    let shader_module = device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl("".into()),
    });

    let vertex_state = VertexState {
        module: &shader_module,
        entry_point: None,
        compilation_options: PipelineCompilationOptions::default(),
        buffers: &[],
    };

    let color_target_state = ColorTargetState {
        format: todo!(),
        blend: todo!(),
        write_mask: todo!(),
    };

    let fragment_state = FragmentState {
        module: &shader_module,
        entry_point: None,
        compilation_options: PipelineCompilationOptions::default(),
        targets: &[],
    };

    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: None,
        vertex: vertex_state,
        primitive: PrimitiveState::default(),
        depth_stencil: None,
        multisample: MultisampleState::default(),
        fragment: Some(fragment_state),
        multiview: None,
        cache: None,
    })
}

// render_pass.set_pipeline(&self.create_render_pipeline());
// render_pass.draw(0..0, 0..0);
