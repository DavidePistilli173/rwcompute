//! Graphics compute context.

use rwlog::sender::Logger;

use crate::error::ContextCreationError;

/// Graphics compute context.
pub struct Context {
    /// Physical device used for computing.
    adapter: wgpu::Adapter,
    /// Logical device used for computing.
    device: wgpu::Device,
    /// Compute command queue.
    queue: wgpu::Queue,
}

impl Context {
    /// Get the active physical compute device.
    pub fn adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    /// Get the active compute device.
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    /// Create a new graphics compute context.
    /// Optionally pass a previously created WGPU instance.
    /// Optionally pass a compatible surface for obtaining a device capable of both computing and rendering workloads.
    pub fn new(
        logger: Logger,
        instance: Option<wgpu::Instance>,
        surface: Option<&wgpu::Surface>,
        features: wgpu::Features,
    ) -> Result<Self, ContextCreationError> {
        pollster::block_on(Context::new_internal(logger, instance, surface, features))
    }

    /// Create a new graphics compute context.
    /// Optionally pass a compatible surface for obtaining a device capable of both computing and rendering workloads.
    async fn new_internal(
        logger: Logger,
        instance: Option<wgpu::Instance>,
        surface: Option<&wgpu::Surface>,
        features: wgpu::Features,
    ) -> Result<Self, ContextCreationError> {
        // Retrieve or create the WGPU instance
        let instance = instance.unwrap_or_else(|| {
            wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends: wgpu::Backends::all(),
                dx12_shader_compiler: Default::default(),
            })
        });

        // Get the physical compute device.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: surface,
            })
            .await
            .ok_or_else(|| {
                rwlog::rel_err!(&logger, "Failed to get a compute device.");
                ContextCreationError::NoPhysicalComputeDevice
            })?;

        // Get logical device and command queue from the graphics adapter.
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: features,
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None,
            )
            .await
            .map_err(|err| {
                rwlog::rel_err!(
                    &logger,
                    "Failed to create logical compute device and queue: {err}."
                );
                ContextCreationError::DeviceOrQueueCreation
            })?;

        Ok(Self {
            adapter,
            device,
            queue,
        })
    }

    /// Get the command queue for the active device.
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
}
