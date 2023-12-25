use nvml_wrapper::enum_wrappers::device::{Clock, TemperatureSensor};
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::Nvml;

pub struct NvGpuStat {
    name: String,
    temperature: u32,
    mem_free: u64,
    mem_total: u64,
    mem_used: u64,
    graphics_clock: u32,
    mem_clock: u32,
    link_gen: u32,
    link_width: u32,
    max_link_gen: u32,
    max_link_width: u32,
    cuda_cores: u32,
    architecture: String,
}

pub fn cuda_version() -> Result<i32, NvmlError> {
    let nvml = Nvml::init()?;
    let cuda_version = nvml.sys_cuda_driver_version()?;
    return Ok(cuda_version);
}

// Function: list
//
// Description:
// The `list` function enumerates and gathers detailed information about each NVIDIA GPU in the system.
// It utilizes the `nvml_wrapper` crate to access NVIDIA Management Library (NVML) functionalities.
// This function initializes NVML, counts the number of GPUs, and then iterates over each GPU to collect
// various details, encapsulating them in a `Vec<NvGPU>`.
//
// Return Type:
// - Result<Vec<NvGPU>, NvmlError>: On success, returns a vector of `NvGPU` structures, each representing a GPU.
//   On failure, returns an `NvmlError`.
//
// Usage Example:
// ```
// fn main() {
//     match list() {
//         Ok(gpus) => {
//             for gpu in gpus {
//                 println!("GPU Name: {}", gpu.name);
//                 // ... other details
//             }
//         }
//         Err(e) => println!("Error occurred: {}", e),
//     }
// }
// `
pub fn list() -> Result<Vec<NvGpuStat>, NvmlError> {
    let nvml = Nvml::init()?;
    let num_gpus = nvml.device_count()?;
    let mut gpus = Vec::new();
    for i in 0..num_gpus {
        let device = nvml.device_by_index(i)?;
        gpus.push(NvGpuStat {
            // Now we can do whatever we want, like getting some data...
            name: device.name()?,
            temperature: device.temperature(TemperatureSensor::Gpu)?,
            mem_free: device.memory_info()?.free,
            mem_total: device.memory_info()?.total,
            mem_used: device.memory_info()?.used,
            graphics_clock: device.clock_info(Clock::Graphics)?,
            mem_clock: device.clock_info(Clock::Memory)?,
            link_gen: device.current_pcie_link_gen()?,
            link_width: device.current_pcie_link_width()?,
            max_link_gen: device.max_pcie_link_gen()?,
            max_link_width: device.max_pcie_link_width()?,
            cuda_cores: device.num_cores()?,
            architecture: device.architecture()?.to_string(),
        });
    }

    Ok(gpus)
}
