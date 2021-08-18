use futures::future;
use std::time::Duration;
use tokio::time::sleep;
use wgpu::{Adapter, BackendBit, DeviceDescriptor, Instance, Label};
use winit::{event_loop::EventLoop, window::Window};
async fn handle_adapter(adaptor: Adapter) {
    let mut event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    let descriptor = DeviceDescriptor {
        label: Some("foo"),
        features: adaptor.features(),
        limits: adaptor.limits(),
    };
    println!("{:?}", adaptor);
    println!("{:?}", adaptor.get_info());
    println!("{:?}", adaptor.limits());
    println!("{:#x?}\n", adaptor.features());
    let (device, queue) = adaptor
        .request_device(&descriptor, None)
        .await
        .expect("failed to build");
    println!("{:?}", device);
    println!("{:?}", queue);
    sleep(Duration::from_secs(10)).await;
}
#[tokio::main]
async fn main() {
    let instance = Instance::new(BackendBit::PRIMARY);
    future::join_all(
        instance
            .enumerate_adapters(BackendBit::PRIMARY)
            .map(|a| handle_adapter(a)),
    )
    .await;
}
