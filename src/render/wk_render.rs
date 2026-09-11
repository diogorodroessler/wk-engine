use wgpu::{Adapter, Device, DisplayAndWindowHandle, Dx12BackendOptions, ExperimentalFeatures, Features, GlBackendOptions, Instance, Limits, MemoryHints, NoopBackendOptions, Queue, RequestAdapterError, RequestDeviceError, Surface, SurfaceTarget::{self, DisplayAndWindow}, Trace, WasmNotSend, wgt::DeviceDescriptor
};
use winit::window::{CursorIcon, WindowButtons};

use crate::render::wk_window::WkWindow;

#[derive(Debug)]
pub struct WkWgpuRendering
{

}
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

    fn wk_wgpu_configure_surface<'window>() -> Option<winit::window::Window>
    {
        let window = WkWindow::create_window_with_winit(
        ("general_win_name", "general_win_instance"), "Wk Engine",
        CursorIcon::Default, false, None,
        true, winit::window::WindowLevel::Normal, WindowButtons::all(),
        true)
        .unwrap()
        ;

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

    pub fn wk_wgpu_render()
    {
        let window = Self::wk_wgpu_configure_surface().unwrap();
        let current_surf_texture_view = &Self::wk_wgpu_surface(&window).unwrap().get_current_texture();
    }
}