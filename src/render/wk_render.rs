use wgpu::{
    Adapter, Color, CommandEncoder, Device, Dx12BackendOptions,
    ExperimentalFeatures, Features, GlBackendOptions, Instance,
    Limits, LoadOp, MemoryHints, NoopBackendOptions, Operations,
    Queue, RenderPassColorAttachment, RenderPassDescriptor,
    RequestAdapterError, RequestDeviceError, Surface, SurfaceConfiguration,
    SurfaceTarget::{self}, TextureFormat, TextureUsages, TextureViewDescriptor,
    Trace, WasmNotSend,
    wgc::{
        resource::{
            CreateTextureError::CreateTextureView, CreateTextureViewError
        }
    },
    wgt::{CommandEncoderDescriptor, DeviceDescriptor},
};
use winit::window::{CursorIcon, WindowButtons};

use crate::render::wk_window::WkWindow;

#[derive(Debug)]
pub struct WkWgpuRendering;
impl WkWgpuRendering
{
    fn wk_wgpu_force_vulkan_instance() -> Instance
    {
        let inst = wgpu::Instance::new(
            wgpu::InstanceDescriptor {
                backends: wgpu::Backends::VULKAN,
                flags: wgpu::InstanceFlags::GPU_BASED_VALIDATION,
                memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
                backend_options: wgpu::BackendOptions {
                    gl: GlBackendOptions::from_env_or_default(),
                    dx12: Dx12BackendOptions::from_env_or_default(),
                    noop: NoopBackendOptions::from_env_or_default()
                    },
                    display: None
                }
            );
        inst
    }

    fn wk_wgpu_surface<'window>(
        window_target: impl Into<SurfaceTarget<'window>> + winit::raw_window_handle::HasWindowHandle + std::marker::Send
    ) -> Result<Surface<'window>, wgpu::CreateSurfaceError>
    {
        let surf = Self::wk_wgpu_force_vulkan_instance().create_surface(window_target)?;

        Ok(surf)
    }

    fn wk_wgpu_surface_pushback<'window>() -> wgpu::Surface<'window>
    {
        let window_target = Self::wk_wgpu_winit_create_window_fallback();
        let surf = Self::wk_wgpu_surface(window_target).unwrap();
        
        surf
    }

    fn wk_wgpu_surface_configure(height: u32, width: u32) -> SurfaceConfiguration
    {
        let config = SurfaceConfiguration {
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            color_space: wgpu::SurfaceColorSpace::Srgb,
            // Check for mutiples monitors :PP
            desired_maximum_frame_latency: 1,
            format: TextureFormat::Rgba32Float,
            height: height,
            width: width,
            present_mode: wgpu::PresentMode::Immediate,
            usage: TextureUsages::all(),
            view_formats: vec![]
        };

        config
    }

    fn wk_wgpu_texture_createview() -> Option<wgpu::TextureView>
    {
        let surface = Self::wk_wgpu_surface_pushback();
        let device = Self::wk_wgpu_device_pushback();
        let config = Self::wk_wgpu_surface_configure(1920, 1080);

        let current_texture = Self::wk_wgpu_surface_pushback().get_current_texture();

        let surf_texture = match current_texture {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,

            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                // opcional: reconfigurar a surface
                texture
            }

            wgpu::CurrentSurfaceTexture::Timeout => {
                return None;
            }

            wgpu::CurrentSurfaceTexture::Occluded => {
                return None;
            }

            wgpu::CurrentSurfaceTexture::Outdated => {
                // reconfigure surface
                let _conf = surface.configure(&device, &config);
                return None;
            }

            wgpu::CurrentSurfaceTexture::Lost => {
                // recriar/reconfigurar surface
                return None;
            }

            wgpu::CurrentSurfaceTexture::Validation => {
                return None;
            }
        };

        // Possible an implementation of the "TextureViewDescriptor" now is a default
        let texture_view = surf_texture.texture.create_view(&TextureViewDescriptor::default());

        Some(texture_view)
    }

    fn wk_wgpu_adapter<'window>(
        window_target: impl Into<SurfaceTarget<'window>> + winit::raw_window_handle::HasWindowHandle + std::marker::Send
    ) -> impl Future<Output = Result<Adapter, RequestAdapterError>> + WasmNotSend
    {
        let surf = Self::wk_wgpu_surface(window_target).unwrap();
        let adap = Self::wk_wgpu_force_vulkan_instance()
            .request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    apply_limit_buckets: false,
                    compatible_surface: Some(&surf),
                    force_fallback_adapter: false
                }
            );

        async move { adap.await.map(|adapter| adapter) }
    }

    fn wk_wgpu_device<'window>(
        window_target: impl Into<SurfaceTarget<'window>> + winit::raw_window_handle::HasWindowHandle + std::marker::Send
    ) -> impl Future<Output = Result<(Device, Queue), RequestDeviceError>> + WasmNotSend
    {
        let lambda = async move {
            let adap = Self::wk_wgpu_adapter(window_target).await.unwrap()
            .request_device(
                &DeviceDescriptor {
                    label: Some("Main Device 1"),
                    experimental_features: ExperimentalFeatures::disabled(),
                    memory_hints: MemoryHints::Performance,
                    required_features: Features::all(),
                    required_limits: Limits::defaults(),
                    trace: Trace::Off
                }
            );
            return adap;
        };

        async move { lambda.await.await.map(|(dev, queue)| {
            (dev, queue)
        }) }
    }

    fn wk_wgpu_winit_create_window_fallback() -> winit::window::Window
    {
        let window = WkWindow::create_window_with_winit(
        ("general_win_name", "general_win_instance"), "Wk Engine",
        CursorIcon::Default, false, None,
        true, winit::window::WindowLevel::Normal, WindowButtons::all(),
        true)
        .unwrap()
        ;

        window
    }

    fn wk_wgpu_device_pushback() -> wgpu::Device
    {
        let window_target = Self::wk_wgpu_winit_create_window_fallback();
        let mut dev: Option<wgpu::Device> = None;

        async
        {
            let (device, _queue) = Self::wk_wgpu_device(&window_target).await.unwrap();
            dev = Some(device);
        };

        dev.unwrap()
    }

    fn wk_wgpu_configure_surface<'window>() -> Option<winit::window::Window>
    {
        let window = Self::wk_wgpu_winit_create_window_fallback();

        async 
        {
            let device = Self::wk_wgpu_device(&window).await.unwrap();
            let size = window.inner_size();
            let adapter = Self::wk_wgpu_adapter(&window).await.map(|adap| { adap }).unwrap();
            let surf = Self::wk_wgpu_surface(&window).unwrap();
            let surf_caps = surf.get_capabilities(&adapter);
                
            let format = surf_caps
            .formats
            .iter()
            .copied()
            .find(|pf| { pf.is_srgb() })
            .unwrap_or(surf_caps.formats[0])
            ;
        
            let config = wgpu::SurfaceConfiguration {
                alpha_mode: surf_caps.alpha_modes[0],
                color_space: wgpu::SurfaceColorSpace::Srgb,
                desired_maximum_frame_latency: 2_u32,
                format: format,
                width: size.width,
                height: size.height,
                present_mode: surf_caps.present_modes[0],
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: vec![]
            };
        
            surf.configure(&device.0, &config);
        };

        None
    }

    fn wk_wgpu_create_texture_view()
    {
        let tex_view_err = CreateTextureViewError::TextureViewFormatNotRenderable(TextureFormat::Rgba32Float);
        let _create_texture_view_err = CreateTextureView(tex_view_err);
        let texture_view_desc = TextureViewDescriptor::default();
    }

    async fn wk_wgpu_create_encoder() -> CommandEncoder
    {
        let window = Self::wk_wgpu_configure_surface().unwrap();
        let (device, _queue) = Self::wk_wgpu_device(&window).await.unwrap();

        let desc = CommandEncoderDescriptor {
            label: Some("Main Encoder 1")
        };
        let encoder = device.create_command_encoder( &desc );

        encoder
    }

    fn wk_wgpu_render_pass()
    {
        let texture_view = Self::wk_wgpu_texture_createview().unwrap();
        {
            async
            {
                let mut render_pass_self = Self::wk_wgpu_create_encoder().await;
                let _binding = render_pass_self
                .begin_render_pass(&RenderPassDescriptor {
                    label: Some("Main Render Pass 1"),
                    
                    color_attachments: &[Some(
                        RenderPassColorAttachment {
                        depth_slice: None,
                        ops: Operations::<Color> {
                            load: LoadOp::Clear(Color { r: 0.05, g: 0.05, b: 0.08, a: 1.0 }),
                            store: wgpu::StoreOp::Store
                        },
                        resolve_target: None,
                        view: &texture_view
                    })],
                    // TODO: Fix all none
                    depth_stencil_attachment: None,
                    multiview_mask: None,
                    occlusion_query_set: None,
                    timestamp_writes: None
                });
            };
        }
    }

    pub fn wk_wgpu_render()
    {
        
    }
}