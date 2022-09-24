use std::borrow::Cow;

use wgpu::{Backends, PowerPreference};

pub struct WgpuSettings {
    pub device_lable: Option<Cow<'static, str>>,
    pub backends: Option<Backends>,
    pub power_preference: PowerPreference,
    pub size: (u32, u32),
}
