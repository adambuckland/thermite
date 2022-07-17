use std::error;
use std::fmt::{Debug, Display, Formatter, Write};
use ash::{LoadingError, vk};
use crate::core::error::Error::{RendererError, VulkanError};

#[derive(Debug)]
pub enum Error {
    RendererError(LoadingError),
    VulkanError(vk::Result)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RendererError(str) => {
                f.write_fmt(format_args!("Renderer Error: {}", str))
            }
            Error::VulkanError(result) => {
                f.write_fmt(format_args!("Vulkan Error: {}", result))
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::RendererError(e) => Some(e),
            Error::VulkanError(e) => {
                match e {
                    &vk::Result::SUCCESS => None,
                    _ => Some(e),
                }
            }
        }
    }
}

impl From<ash::vk::Result> for Error {
    fn from(result: ash::vk::Result) -> Self {
        VulkanError(result)
    }
}

impl From<ash::LoadingError> for Error {
    fn from(le: LoadingError) -> Self {
        RendererError(le)
    }
}