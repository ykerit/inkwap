use std::mem::size_of;

pub struct Vertex<T: Copy> {
    buffer: wgpu::Buffer,
    capacity: usize,
    data: Vec<T>,
    lable: String,
}

impl<T: Copy> Vertex<T> {
    pub fn new(device: &wgpu::Device, capacity: usize, lable: &str) -> Vertex<T> {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(lable),
            size: (size_of::<T>() * capacity) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Self {
            buffer,
            capacity,
            data: vec![],
            lable: lable.into(),
        }
    }
}
