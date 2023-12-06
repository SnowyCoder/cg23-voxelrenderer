use std::{borrow::Cow, mem};

use cgmath::{Vector3, Point3, EuclideanSpace};
use wgpu::{Device, Queue, ShaderModule, TextureFormat, PipelineLayout, RenderPipeline, Instance, Adapter, util::{DeviceExt, BufferInitDescriptor}, BufferUsages};
use winit::{event_loop::EventLoopWindowTarget, dpi::PhysicalSize};

use crate::{parser::{Model, self, Scene}, camera::{CameraUniform, Camera, CameraController}, model::{ModelVertex, InstanceData}, texture::Texture};

pub const CUBE_MODEL_PLY: &'static [u8] = include_bytes!("../models/pcube.ply");

fn load_cube() -> Model {
    let mut reader = CUBE_MODEL_PLY;
    parser::parse_model(&mut reader)
}

pub struct RenderState {
    _shader: ShaderModule,
    pub target_format: TextureFormat,
    _pipeline_layout: PipelineLayout,
    pub queue: Queue,
    pub render_pipeline: RenderPipeline,
    pub depth_texture: Texture,

    // model
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub model: Model,
    // instances
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,

    // camera
    pub camera_uniform: CameraUniform,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,

    pub device: Device,
}

pub struct SurfaceState {
    pub window: winit::window::Window,
    pub surface: wgpu::Surface,
}

pub struct WorldState {
    pub camera: Camera,
    pub camera_controller: CameraController,
    pub scene: Option<Scene>,
}

pub struct App {
    pub instance: Instance,
    pub adapter: Option<Adapter>,
    pub surface_state: Option<SurfaceState>,
    pub render_state: Option<RenderState>,
    pub world_state: WorldState,
}

impl App {
    pub fn new(instance: Instance) -> Self {
        Self {
            instance,
            adapter: None,
            surface_state: None,
            render_state: None,
            world_state: WorldState {
                camera: Camera::new(1.0),
                camera_controller: CameraController::new(0.2),
                scene: None,
            }
        }
    }
}

impl App {
    fn create_surface<T>(&mut self, event_loop: &EventLoopWindowTarget<T>) {
        let window = winit::window::Window::new(event_loop).unwrap();
        log::info!("WGPU: creating surface for native window");

        // # Panics
        // Currently create_surface is documented to only possibly fail with with WebGL2
        let surface = unsafe {
            self.instance
                .create_surface(&window)
                .expect("Failed to create surface")
        };
        self.surface_state = Some(SurfaceState { window, surface });
    }

    async fn init_render_state(adapter: &Adapter, target_format: TextureFormat, window_size: PhysicalSize<u32>) -> RenderState {
        log::info!("Initializing render state");

        log::info!("WGPU: requesting device");
        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        log::info!("WGPU: loading shader");
        // Load the shaders from disk
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        // Camera
        let camera_uniform = CameraUniform::new();
        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Buffer"),
            size: mem::size_of::<CameraUniform>() as _,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let depth_texture =
            Texture::create_depth_texture(&device, (window_size.width, window_size.height), "depth_texture");

        log::info!("WGPU: creating pipeline layout");
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        log::info!("WGPU: creating render pipeline");
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[ModelVertex::desc(), InstanceData::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(target_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let model = load_cube();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(model.vertices.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(model.indices.as_slice()),
            usage: wgpu::BufferUsages::INDEX,
        });
        // Create a placeholder buffer
        // When we load a real scene we'll fill this.
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: 0,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        RenderState {
            device,
            queue,
            _shader: shader,
            target_format,
            _pipeline_layout: pipeline_layout,
            render_pipeline,
            depth_texture,

            camera_uniform,
            camera_buffer,
            camera_bind_group,

            model,
            vertex_buffer,
            index_buffer,
            instance_buffer,
            instance_count: 0,
        }
    }

    // We want to defer the initialization of our render state until
    // we have a surface so we can take its format into account.
    //
    // After we've initialized our render state once though we
    // expect all future surfaces will have the same format and we
    // so this stat will remain valid.
    async fn ensure_render_state_for_surface(&mut self) {
        if let Some(surface_state) = &self.surface_state {
            if self.adapter.is_none() {
                log::info!("WGPU: requesting a suitable adapter (compatible with our surface)");
                let adapter = self
                    .instance
                    .request_adapter(&wgpu::RequestAdapterOptions {
                        power_preference: wgpu::PowerPreference::default(),
                        force_fallback_adapter: false,
                        // Request an adapter which can render to our surface
                        compatible_surface: Some(&surface_state.surface),
                    })
                    .await
                    .expect("Failed to find an appropriate adapter");

                log::info!("WGPU Adapter features: {:?}", adapter.features());

                self.adapter = Some(adapter);
            }
            let adapter = self.adapter.as_ref().unwrap();

            if self.render_state.is_none() {
                log::info!("WGPU: finding supported swapchain format");
                let surface_caps = surface_state.surface.get_capabilities(adapter);
                let swapchain_format = surface_caps.formats[0];
                let window_size = surface_state.window.inner_size();
                let rs = Self::init_render_state(adapter, swapchain_format, window_size).await;
                self.render_state = Some(rs);
            }

            self.load_scene();
        }
    }

    pub fn configure_surface_swapchain(&mut self) {
        if let (Some(render_state), Some(surface_state)) = (&mut self.render_state, &self.surface_state)
        {
            let swapchain_format = render_state.target_format;
            let size = surface_state.window.inner_size();

            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: swapchain_format,
                width: size.width,
                height: size.height,
                //present_mode: wgpu::PresentMode::Mailbox,
                present_mode: wgpu::PresentMode::AutoVsync,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: vec![swapchain_format],
            };

            log::info!("WGPU: Configuring surface swapchain: format = {swapchain_format:?}, size = {size:?}");
            surface_state
                .surface
                .configure(&render_state.device, &config);


            let depth_texture =
                Texture::create_depth_texture(&render_state.device, (size.width, size.height), "depth_texture");
            render_state.depth_texture = depth_texture;

            self.world_state.camera.update_aspect_ratio(size.width as f32, size.height as f32);
        }
    }

    pub fn queue_redraw(&self) {
        if let Some(surface_state) = &self.surface_state {
            log::trace!("Making Redraw Request");
            surface_state.window.request_redraw();
        }
    }

    pub fn resume<T>(&mut self, event_loop: &EventLoopWindowTarget<T>) {
        log::info!("Resumed, creating render state...");
        self.create_surface(event_loop);
        pollster::block_on(self.ensure_render_state_for_surface());
        self.configure_surface_swapchain();
        self.queue_redraw();
    }

    pub fn load_scene(&mut self) {
        let (scene, rs) = match (self.world_state.scene.as_ref(), self.render_state.as_mut()) {
            (Some(x), Some(y)) => (x, y),
            _ => return,
        };

        let center = scene.grid_size.map(|x| x as f32 / 2.0);
        let real_dims = scene.voxels.iter().fold(Vector3::<u32>::new(0, 0, 0), |a, x| {
            Vector3::new(a.x.max(x.pos.x), a.y.max(x.pos.y), a.z.max(x.pos.z))
        }) + Vector3::new(1, 1, 1);

        log::error!("Center: {center:?}");
        log::error!("Dims: {:?} vs {:?}", real_dims, scene.grid_size);


        let instances: Vec<InstanceData> = scene.voxels.iter().map(|x| InstanceData {
            pos: [x.pos.x as f32, x.pos.y as f32, x.pos.z as f32 ],
            color: scene.colors[x.color as usize].as_instance_data(),
        }).collect();
        let instance_buffer = rs.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Indices Bufer"),
            contents: bytemuck::cast_slice(instances.as_slice()),
            usage: BufferUsages::VERTEX,
        });

        rs.instance_buffer = instance_buffer;
        rs.instance_count = instances.len() as _;
        log::warn!("Loaded scene!!: {}", instances.len());

        let camera = &mut self.world_state.camera;
        camera.target = Point3::from_vec(center);
        camera.eye = Point3::from_vec(center * 3.0);
    }
}
