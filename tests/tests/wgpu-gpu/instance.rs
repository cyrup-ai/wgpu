use wgpu_test::{GpuTestConfiguration, gpu_test};

#[gpu_test]
static INITIALIZE: GpuTestConfiguration = GpuTestConfiguration::new().run_sync(|_ctx| {});
