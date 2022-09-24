mod settings;
use wgpu::{Device, DeviceDescriptor, Features, Instance, Limits, Queue, RequestAdapterOptions};

use settings::WgpuSettings;

pub async fn initialize_renderer(
    instance: &Instance,
    options: WgpuSettings,
    request_adapter_options: &RequestAdapterOptions<'_>,
) -> (Device, Queue) {
    let adapter = instance
        .request_adapter(request_adapter_options)
        .await
        .expect("Unable to find GPU, Please check GPU devices drivers!");
    let adapter_info = adapter.get_info();
    tracing::info!("{:?}", adapter_info);
    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: options.device_lable.as_ref().map(|a| a.as_ref()),
                features: Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    Limits::downlevel_webgl2_defaults()
                } else {
                    Limits::default()
                },
            },
            None,
        )
        .await
        .unwrap();

    return (device, queue);
}
