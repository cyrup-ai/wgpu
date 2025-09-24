#[cfg(webgpu)]
pub mod webgpu;
#[cfg(webgpu)]
pub(crate) use webgpu::{ContextWebGpu, get_browser_gpu_property};

#[cfg(wgpu_core)]
pub mod wgpu_core;

#[cfg(wgpu_core)]
pub(crate) use wgpu_core::ContextWgpuCore;

#[cfg(custom)]
pub mod custom;
