use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    CreateSurface(#[from] wgpu::CreateSurfaceError),
    #[error(transparent)]
    RequestAdapter(#[from] wgpu::RequestAdapterError),
    #[error(transparent)]
    RequestDevice(#[from] wgpu::RequestDeviceError),
    #[error("surface is not supported by wgpu adapter")]
    SurfaceNotSupportedByAdapter,
    #[error("surface capabilities contain no texture format")]
    SurfaceCapabilitiesNoFormats,
}

pub struct Setup {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    pub gpu: shame_wgpu::Gpu,
}

impl Setup {
    pub async fn new(window: &Arc<winit::window::Window>) -> Result<Setup, Error> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(Arc::clone(window))?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty() | wgpu::Features::IMMEDIATES,
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                required_limits: wgpu::Limits {
                    max_immediate_size: 4,
                    ..wgpu::Limits::default().using_resolution(adapter.limits())
                },
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
                experimental_features: Default::default(),
            })
            .await?;

        let mut config = surface
            .get_default_config(&adapter, 1, 1)
            .ok_or(Error::SurfaceNotSupportedByAdapter)?;

        config.present_mode = wgpu::PresentMode::AutoVsync;

        let mut setup = Setup {
            instance,
            adapter,
            surface,
            gpu: shame_wgpu::Gpu::new(device, queue, Some(config.format)),
            surface_config: config,
        };

        setup.resize(window.inner_size().into());
        Ok(setup)
    }

    pub fn resize(&mut self, size: [u32; 2]) {
        let cfg = &mut self.surface_config;

        // swapchain sizes of 0 can cause crashes
        [cfg.width, cfg.height] = size.map(|d| d.max(1));

        self.surface.configure(&self.gpu, cfg);
    }

    fn try_acquire_surface_texture(&self) -> wgpu::SurfaceTexture {
        use wgpu::CurrentSurfaceTexture as C;
        loop {
            match self.surface.get_current_texture() {
                C::Success(texture) | C::Suboptimal(texture) => return texture,
                C::Timeout => continue,
                C::Outdated | C::Lost | C::Occluded => {
                    self.surface.configure(&self.gpu, &self.surface_config)
                }
                C::Validation => {
                    self.surface.configure(&self.gpu, &self.surface_config)
                }
            }
        }
    }

    pub fn try_acquire_surface(&self) -> (wgpu::SurfaceTexture, wgpu::TextureView) {
        let surface_texture = self.try_acquire_surface_texture();

        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor {
            format: Some(self.surface_config.format),
            ..wgpu::TextureViewDescriptor::default()
        });
        (surface_texture, view)
    }
}
