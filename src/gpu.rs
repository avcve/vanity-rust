use cust::prelude::*;
use std::error::Error;

pub fn run_gpu(prefix: &str, suffix: &str, batch_size: u32) -> Result<(), Box<dyn Error>> {
    // Initialize CUDA
    let _ctx = cust::quick_init()?;

    let ptx = r#"
    extern "C" __global__ void vanity_kernel(char* results, int batch_size) {
        int idx = blockIdx.x * blockDim.x + threadIdx.x;
        if(idx >= batch_size) return;

        // TODO: implement wallet generation & pattern matching
        results[idx] = 0;
    }
    "#;

    let module = Module::from_ptx(ptx, &[])?;
    let func = module.get_function("vanity_kernel")?;

    let mut results = vec![0u8; batch_size as usize];
    let results_buf = DeviceBuffer::from_slice(&results)?;

    unsafe {
        launch!(func<<<batch_size/256, 256, 0, Stream::null()>>>(
            results_buf.as_device_ptr(),
            batch_size
        ))?;
    }

    results_buf.copy_to(&mut results)?;

    println!("Results: {:?}", &results[..]);
    Ok(())
}
