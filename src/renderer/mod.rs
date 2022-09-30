mod mesh;
mod settings;

use settings::WgpuSettings;

pub async fn initialize_renderer(
    instance: &wgpu::Instance,
    options: WgpuSettings,
    request_adapter_options: &wgpu::RequestAdapterOptions<'_>,
) -> (wgpu::Device, wgpu::Queue) {
    let adapter = instance
        .request_adapter(request_adapter_options)
        .await
        .expect("Unable to find GPU, Please check GPU devices drivers!");
    let adapter_info = adapter.get_info();
    tracing::info!("{:?}", adapter_info);
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: options.device_lable.as_ref().map(|a| a.as_ref()),
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
            },
            None,
        )
        .await
        .unwrap();

    return (device, queue);
}

pub fn create_render_pipeline(device: &wgpu::Device) {}
