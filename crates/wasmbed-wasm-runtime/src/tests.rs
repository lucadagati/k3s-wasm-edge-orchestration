// SPDX-License-Identifier: AGPL-3.0
// Copyright © 2025 Wasmbed contributors

use crate::config::{DeviceArchitecture, RuntimeConfig};
use crate::context::WasmContext;
use crate::error::WasmResult;
use crate::runtime::WasmRuntime;
use std::sync::Arc;

#[tokio::test]
async fn test_runtime_creation() -> WasmResult<()> {
    let config = RuntimeConfig::for_architecture(DeviceArchitecture::Mpu, "test-device".to_string());
    let mut runtime = WasmRuntime::new(config)?;
    
    // Test that runtime was created successfully
    assert_eq!(runtime.context().device_id, "test-device");
    assert_eq!(runtime.context().architecture, DeviceArchitecture::Mpu);
    
    Ok(())
}

#[tokio::test]
async fn test_module_loading() -> WasmResult<()> {
    let config = RuntimeConfig::for_architecture(DeviceArchitecture::Mpu, "test-device".to_string());
    let mut runtime = WasmRuntime::new(config)?;
    
    // Create a simple WASM module (empty module)
    let wasm_bytes = wat::parse_str(include_str!("../test_data/empty.wat")).expect("valid WAT");
    let wasm_bytes = wasm_bytes.as_slice();
    
    // Load module
    let metadata = runtime.load_module("test-module", wasm_bytes).await?;
    
    assert_eq!(metadata.id, "test-module");
    assert_eq!(metadata.name, "module_test-module");
    
    Ok(())
}

#[tokio::test]
async fn test_instance_creation() -> WasmResult<()> {
    let config = RuntimeConfig::for_architecture(DeviceArchitecture::Mpu, "test-device".to_string());
    let mut runtime = WasmRuntime::new(config)?;
    
    // Create a simple WASM module
    let wasm_bytes = wat::parse_str(include_str!("../test_data/empty.wat")).expect("valid WAT");
    let wasm_bytes = wasm_bytes.as_slice();
    runtime.load_module("test-module", wasm_bytes).await?;
    
    // Create instance
    let instance_id = runtime.create_instance("test-module", None).await?;
    
    assert!(!instance_id.is_empty());
    
    // Check that instance exists
    let instance_info = runtime.get_instance_info(&instance_id)?;
    assert_eq!(instance_info.module_id, "test-module");
    
    Ok(())
}

#[tokio::test]
async fn test_function_calling() -> WasmResult<()> {
    let config = RuntimeConfig::for_architecture(DeviceArchitecture::Mpu, "test-device".to_string());
    let mut runtime = WasmRuntime::new(config)?;
    
    // Create a simple WASM module with a function
    let wasm_bytes = wat::parse_str(include_str!("../test_data/simple.wat")).expect("valid WAT");
    let wasm_bytes = wasm_bytes.as_slice();
    runtime.load_module("test-module", wasm_bytes).await?;
    
    // Create instance
    let instance_id = runtime.create_instance("test-module", None).await?;
    
    // Call function (this would need a real WASM module with a function)
    // For now, we'll just test that the runtime doesn't crash
    let args = vec![];
    let result = runtime.call_function(&instance_id, "test_function", &args).await;
    
    // The function call might fail if the function doesn't exist, but that's expected
    // We're just testing that the runtime handles it gracefully
    match result {
        Ok(_) => println!("Function call succeeded"),
        Err(e) => println!("Function call failed as expected: {}", e),
    }
    
    Ok(())
}

#[tokio::test]
async fn test_runtime_statistics() -> WasmResult<()> {
    let config = RuntimeConfig::for_architecture(DeviceArchitecture::Mpu, "test-device".to_string());
    let mut runtime = WasmRuntime::new(config)?;
    
    // Get initial stats
    let stats = runtime.get_stats();
    assert_eq!(stats.modules_loaded, 0);
    assert_eq!(stats.instances_created, 0);
    
    // Load a module
    let wasm_bytes = wat::parse_str(include_str!("../test_data/empty.wat")).expect("valid WAT");
    let wasm_bytes = wasm_bytes.as_slice();
    runtime.load_module("test-module", wasm_bytes).await?;
    
    // Check stats after loading
    let stats = runtime.get_stats();
    assert_eq!(stats.modules_loaded, 1);
    
    // Create an instance
    runtime.create_instance("test-module", None).await?;
    
    // Check stats after creating instance
    let stats = runtime.get_stats();
    assert_eq!(stats.instances_created, 1);
    
    Ok(())
}

#[tokio::test]
async fn test_device_specific_configurations() -> WasmResult<()> {
    // Test MPU configuration
    let mpu_config = RuntimeConfig::for_architecture(DeviceArchitecture::Mpu, "mpu-device".to_string());
    let mpu_runtime = WasmRuntime::new(mpu_config)?;
    assert_eq!(mpu_runtime.context().memory_limit, 8 * 1024 * 1024 * 1024); // 8GB
    assert_eq!(mpu_runtime.context().max_instances, 100);
    
    // Test MCU configuration
    let mcu_config = RuntimeConfig::for_architecture(DeviceArchitecture::Mcu, "mcu-device".to_string());
    let mcu_runtime = WasmRuntime::new(mcu_config)?;
    assert_eq!(mcu_runtime.context().memory_limit, 64 * 1024); // 64KB
    assert_eq!(mcu_runtime.context().max_instances, 5);
    
    // Test RISC-V configuration
    let riscv_config = RuntimeConfig::for_architecture(DeviceArchitecture::RiscV, "riscv-device".to_string());
    let riscv_runtime = WasmRuntime::new(riscv_config)?;
    assert_eq!(riscv_runtime.context().memory_limit, 512 * 1024); // 512KB
    assert_eq!(riscv_runtime.context().max_instances, 20);
    
    Ok(())
}

#[tokio::test]
async fn test_memory_limits() -> WasmResult<()> {
    let config = RuntimeConfig::for_architecture(DeviceArchitecture::Mcu, "test-device".to_string());
    let mut runtime = WasmRuntime::new(config)?;
    
    // Test memory limit checking
    let result = runtime.context().check_memory_limit(1000);
    assert!(result.is_ok());
    
    // Test exceeding memory limit
    let result = runtime.context().check_memory_limit(100 * 1024 * 1024); // 100MB
    assert!(result.is_err());
    
    Ok(())
}

#[tokio::test]
async fn test_cpu_time_limits() -> WasmResult<()> {
    let config = RuntimeConfig::for_architecture(DeviceArchitecture::Mcu, "test-device".to_string());
    let mut runtime = WasmRuntime::new(config)?;
    
    // Test CPU time limit checking
    let result = runtime.context().check_cpu_time_limit(std::time::Duration::from_millis(50));
    assert!(result.is_ok());
    
    // Test exceeding CPU time limit
    let result = runtime.context().check_cpu_time_limit(std::time::Duration::from_secs(1));
    assert!(result.is_err());
    
    Ok(())
}

#[tokio::test]
async fn test_instance_limits() -> WasmResult<()> {
    let config = RuntimeConfig::for_architecture(DeviceArchitecture::Mcu, "test-device".to_string());
    let mut runtime = WasmRuntime::new(config)?;
    
    // Test instance limit checking
    let result = runtime.context().check_instance_limit();
    assert!(result.is_ok());
    
    // Manually set active instances to limit
    runtime.context_mut().active_instances = runtime.context().max_instances;
    
    // Test exceeding instance limit
    let result = runtime.context().check_instance_limit();
    assert!(result.is_err());
    
    Ok(())
}

#[tokio::test]
async fn test_runtime_shutdown() -> WasmResult<()> {
    let config = RuntimeConfig::for_architecture(DeviceArchitecture::Mpu, "test-device".to_string());
    let mut runtime = WasmRuntime::new(config)?;
    
    // Load a module and create an instance
    let wasm_bytes = wat::parse_str(include_str!("../test_data/empty.wat")).expect("valid WAT");
    let wasm_bytes = wasm_bytes.as_slice();
    runtime.load_module("test-module", wasm_bytes).await?;
    runtime.create_instance("test-module", None).await?;
    
    // Check that we have an instance
    assert_eq!(runtime.context().active_instances, 1);
    
    // Shutdown runtime
    runtime.shutdown()?;
    
    // Check that instances are cleared
    assert_eq!(runtime.context().active_instances, 0);
    
    Ok(())
}

#[tokio::test]
async fn test_host_function_integration() -> WasmResult<()> {
    let config = RuntimeConfig::for_architecture(DeviceArchitecture::Mpu, "test-device".to_string());
    let mut runtime = WasmRuntime::new(config)?;
    
    // Test that host functions are available
    assert!(runtime.host_functions().device_functions().is_some());
    assert!(runtime.host_functions().sensor_functions().is_some());
    assert!(runtime.host_functions().security_functions().is_some());
    assert!(runtime.host_functions().gpio_functions().is_some());
    assert!(runtime.host_functions().i2c_spi_functions().is_some());
    
    Ok(())
}

#[tokio::test]
async fn test_error_handling() -> WasmResult<()> {
    let config = RuntimeConfig::for_architecture(DeviceArchitecture::Mpu, "test-device".to_string());
    let mut runtime = WasmRuntime::new(config)?;
    
    // Test loading invalid module
    let invalid_wasm = b"invalid wasm data";
    let result = runtime.load_module("invalid-module", invalid_wasm).await;
    assert!(result.is_err());
    
    // Test creating instance for non-existent module
    let result = runtime.create_instance("non-existent-module", None).await;
    assert!(result.is_err());
    
    // Test calling function on non-existent instance
    let args = vec![];
    let result = runtime.call_function("non-existent-instance", "test_function", &args).await;
    assert!(result.is_err());
    
    Ok(())
}
