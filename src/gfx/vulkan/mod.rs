mod utils;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use ash::{Entry, Instance, LoadingError, vk};
use ash::extensions::khr::GetPhysicalDeviceProperties2;
use ash::vk::{ApplicationInfo, InstanceCreateInfo, MAX_PHYSICAL_DEVICE_NAME_SIZE};
use crate::core::error::Error;

pub struct VulkanBackend {
    instance: ash::Instance,
}

pub fn create_instance() -> Result<VulkanBackend, Error> {
    let app_info = ApplicationInfo {
        api_version: vk::API_VERSION_1_0,
        ..Default::default()
    };

    let layer_names: Vec<std::ffi::CString> =
        vec![std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
    let layer_name_ptrs: Vec<*const i8> = layer_names
        .iter()
        .map(|layer_name| layer_name.as_ptr())
        .collect();
    let extension_name_ptrs: Vec<*const i8> =
        vec![ash::extensions::ext::DebugUtils::name().as_ptr()];

    let create_info = InstanceCreateInfo {
        p_application_info: &app_info,
        pp_enabled_layer_names: layer_name_ptrs.as_ptr(),
        enabled_layer_count: layer_name_ptrs.len() as u32,
        pp_enabled_extension_names: extension_name_ptrs.as_ptr(),
        enabled_extension_count: extension_name_ptrs.len() as u32,
        ..Default::default()
    };

    let entry = utils::create_entry();
    let instance = unsafe {entry.create_instance(&create_info, None)? };

    Ok(VulkanBackend{
        instance,
    })
}

impl VulkanBackend {
    pub fn print_devices(&self) {
        let devices = unsafe {self.instance.enumerate_physical_devices().unwrap()};
        let names: Vec<String> = devices.iter().map(|device| unsafe {
                let props = self.instance.get_physical_device_properties(*device);
                let raw = unsafe { CStr::from_ptr(props.device_name.as_ptr())};
                raw.to_str().unwrap().to_string()
            })
            .collect();
        println!("{:#?}", names);
    }
}

impl Drop for VulkanBackend {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}