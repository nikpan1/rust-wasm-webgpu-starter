use wgpu::util::DeviceExt;
use web_sys::HtmlCanvasElement;

use crate::geometry::{Vertex, INDICES, VERTICES};
use crate::math;

// ─────────────────────────── State ───────────────────────────────────────────

pub struct State {
    surface:            wgpu::Surface<'static>,
    device:             wgpu::Device,
    queue:              wgpu::Queue,
    config:             wgpu::SurfaceConfiguration,
    render_pipeline:    wgpu::RenderPipeline,
    vertex_buffer:      wgpu::Buffer,
    index_buffer:       wgpu::Buffer,
    num_indices:        u32,
    uniform_buffer:     wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    depth_view:         wgpu::TextureView,
    pub rotation_angle: f32,
    pub rotation_speed: f32,
}

impl State {
    pub async fn new(canvas: HtmlCanvasElement) -> Result<Self, String> {
        let width  = canvas.width();
        let height = canvas.height();

        // ── Instance ────────────────────────────────────────────────────
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends:                 wgpu::Backends::BROWSER_WEBGPU,
            flags:                    wgpu::InstanceFlags::default(),
            memory_budget_thresholds: Default::default(),
            backend_options:          Default::default(),
            display:                  None,
        });

        // ── Surface from canvas ─────────────────────────────────────────
        let surface = instance
            .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
            .map_err(|e| format!("create_surface: {e}"))?;

        // ── Adapter ──────────────────────────────────────────────────────
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference:       wgpu::PowerPreference::HighPerformance,
                compatible_surface:     Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(|e| format!("request_adapter: {e}"))?;

        // ── Device + Queue ───────────────────────────────────────────────
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .map_err(|e| format!("request_device: {e}"))?;

        // ── Surface configuration ────────────────────────────────────────
        let surface_caps   = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats.iter().copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage:                         wgpu::TextureUsages::RENDER_ATTACHMENT,
            format:                        surface_format,
            width,
            height,
            present_mode:                  wgpu::PresentMode::Fifo,
            alpha_mode:                    surface_caps.alpha_modes[0],
            view_formats:                  vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // ── Shader ───────────────────────────────────────────────────────
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label:  Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // ── Uniform buffer (MVP matrix = 64 bytes) ───────────────────────
        let initial_mvp = math::compute_mvp(0.0, width as f32 / height as f32);
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label:    Some("uniform_buffer"),
            contents: bytemuck::cast_slice(&initial_mvp),
            usage:    wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // ── Bind group layout ────────────────────────────────────────────
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label:   Some("bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding:    0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty:                 wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size:   None,
                },
                count: None,
            }],
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:   Some("uniform_bg"),
            layout:  &bgl,
            entries: &[wgpu::BindGroupEntry {
                binding:  0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // ── Pipeline layout ───────────────────────────────────────────────
        // wgpu 29: bind_group_layouts = &[Option<&BGL>], push_constant_ranges gone → immediate_size
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:              Some("pipeline_layout"),
            bind_group_layouts: &[Some(&bgl)],
            immediate_size:     0,
        });

        // ── Render pipeline ───────────────────────────────────────────────
        // wgpu 29: depth_write_enabled / depth_compare are Option<_>
        //          multiview renamed to multiview_mask: Option<NonZero<u32>>
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label:  Some("render_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module:              &shader,
                entry_point:         Some("vs_main"),
                buffers:             &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module:              &shader,
                entry_point:         Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format:     config.format,
                    blend:      Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology:           wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face:         wgpu::FrontFace::Ccw,
                cull_mode:          Some(wgpu::Face::Back),
                polygon_mode:       wgpu::PolygonMode::Fill,
                unclipped_depth:    false,
                conservative:       false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format:              wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare:       Some(wgpu::CompareFunction::Less),
                stencil:             wgpu::StencilState::default(),
                bias:                wgpu::DepthBiasState::default(),
            }),
            multisample:    wgpu::MultisampleState::default(),
            multiview_mask: None,   // Option<NonZero<u32>> — None = no multiview
            cache:          None,
        });

        // ── Geometry buffers ─────────────────────────────────────────────
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label:    Some("vertex_buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage:    wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label:    Some("index_buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage:    wgpu::BufferUsages::INDEX,
        });

        // ── Depth texture ─────────────────────────────────────────────────
        let depth_view = Self::make_depth_view(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices: INDICES.len() as u32,
            uniform_buffer,
            uniform_bind_group,
            depth_view,
            rotation_angle: 0.0,
            rotation_speed: 0.005,
        })
    }

    // ── Helpers ──────────────────────────────────────────────────────────────

    fn make_depth_view(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::TextureView {
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label:           Some("depth_texture"),
            size: wgpu::Extent3d { width: config.width, height: config.height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count:    1,
            dimension:       wgpu::TextureDimension::D2,
            format:          wgpu::TextureFormat::Depth32Float,
            usage:           wgpu::TextureUsages::RENDER_ATTACHMENT
                           | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats:    &[],
        });
        tex.create_view(&wgpu::TextureViewDescriptor::default())
    }

    // ── Per-frame logic ───────────────────────────────────────────────────────

    /// Advance rotation and upload new MVP.
    pub fn tick(&mut self) {
        self.rotation_angle += self.rotation_speed;
        let aspect = self.config.width as f32 / self.config.height as f32;
        let mvp = math::compute_mvp(self.rotation_angle, aspect);
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&mvp));
    }

    /// Encode + submit one render pass.
    ///
    /// wgpu 29: get_current_texture() → CurrentSurfaceTexture (enum)
    ///          Success/Suboptimal contain SurfaceTexture with .texture and .present()
    ///          RenderPassColorAttachment has new `depth_slice: None` field
    ///          RenderPassDescriptor has `multiview_mask: Option<NonZero<u32>>`
    pub fn render_frame(&mut self) {
        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) => t,
            wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            // Skip this frame if surface isn't ready
            _ => return,
        };

        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: Some("render_encoder") },
        );

        {
            let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view:           &view,
                    resolve_target: None,
                    depth_slice:    None,   // wgpu 29: required new field
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05, g: 0.05, b: 0.12, a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load:  wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes:    None,
                occlusion_query_set: None,
                multiview_mask:      None, // Option<NonZero<u32>>
            });

            rp.set_pipeline(&self.render_pipeline);
            rp.set_bind_group(0, &self.uniform_bind_group, &[]);
            rp.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            rp.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            rp.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }
}
