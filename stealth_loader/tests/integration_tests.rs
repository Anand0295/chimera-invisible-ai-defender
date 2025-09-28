//! Integration tests for stealth loader
//! 
//! ⚠️ These tests verify simulation behavior only - no real stealth operations

use anyhow::Result;
use serial_test::serial;
use stealth_loader::{StealthConfig, StealthLoader};
use std::path::PathBuf;
use tempfile::TempDir;

#[tokio::test]
#[serial]
async fn test_stealth_loader_installation() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config = StealthConfig {
        simulation_mode: true,
        enable_persistence: false,
        enable_usb_trigger: false,
        encryption_key: None,
        install_path: temp_dir.path().to_path_buf(),
    };

    let mut loader = StealthLoader::new(config)?;
    assert!(!loader.is_installed());

    // Test installation
    loader.install().await?;
    assert!(loader.is_installed());

    // Verify simulation files were created
    let payload_path = temp_dir.path().join("payload.enc");
    assert!(payload_path.exists());

    // Test status
    let status = loader.get_status();
    assert_eq!(status["installed"], true);
    assert_eq!(status["simulation_mode"], true);

    // Test uninstallation
    loader.uninstall().await?;
    assert!(!loader.is_installed());
    assert!(!payload_path.exists());

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_stealth_loader_safety_enforcement() -> Result<()> {
    // Test that dangerous configurations are automatically disabled
    let config = StealthConfig {
        simulation_mode: false, // Try to disable simulation
        enable_persistence: true, // Try to enable persistence
        enable_usb_trigger: true, // Try to enable USB triggers
        encryption_key: Some("test_key".to_string()),
        install_path: PathBuf::from("/tmp/test_stealth"),
    };

    let loader = StealthLoader::new(config)?;
    let status = loader.get_status();

    // Verify safety measures are enforced
    assert_eq!(status["simulation_mode"], true);
    assert!(status["safety_notice"].as_str().unwrap().contains("disabled"));

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_stealth_loader_invisibility() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config = StealthConfig {
        simulation_mode: true,
        enable_persistence: false,
        enable_usb_trigger: false,
        encryption_key: None,
        install_path: temp_dir.path().to_path_buf(),
    };

    let mut loader = StealthLoader::new(config)?;
    
    // Capture logs to verify no console output in stealth mode
    let _guard = tracing_subscriber::fmt()
        .with_test_writer()
        .with_max_level(tracing::Level::WARN)
        .try_init();

    loader.install().await?;

    // In a real stealth loader, we would verify:
    // - No console output
    // - No log files created
    // - Process not visible in task manager
    // - Files hidden from directory listings
    
    // For simulation, we just verify the operation completed
    assert!(loader.is_installed());

    loader.uninstall().await?;
    Ok(())
}

#[test]
fn test_crypto_manager() -> Result<()> {
    use stealth_loader::crypto::CryptoManager;

    let key = CryptoManager::generate_key();
    let crypto = CryptoManager::new(&key);

    let test_data = b"Sensitive payload data for testing";
    let encrypted = crypto.encrypt(test_data)?;
    let decrypted = crypto.decrypt(&encrypted)?;

    assert_eq!(test_data, decrypted.as_slice());
    assert_ne!(test_data.to_vec(), encrypted);

    Ok(())
}

#[test]
fn test_obfuscation() {
    use stealth_loader::crypto::obfuscation;

    let data = b"Test data for obfuscation";
    let key = obfuscation::generate_obfuscation_key(16);
    
    let obfuscated = obfuscation::simple_xor_obfuscate(data, &key);
    let deobfuscated = obfuscation::simple_xor_obfuscate(&obfuscated, &key);
    
    assert_eq!(data, deobfuscated.as_slice());
    assert_ne!(data.to_vec(), obfuscated);

    // Test padding
    let padded = obfuscation::add_padding(data, 100);
    assert_eq!(padded.len(), 100);
    assert_eq!(&padded[..data.len()], data);
}

#[test]
fn test_persistence_manager() -> Result<()> {
    use stealth_loader::persistence::PersistenceManager;

    let pm = PersistenceManager::new();
    let dummy_path = PathBuf::from("/tmp/test_executable");

    // All operations should succeed but do nothing in simulation
    pm.install_registry_persistence(&dummy_path)?;
    pm.install_service_persistence("test_service", &dummy_path)?;
    pm.install_autostart_persistence(&dummy_path)?;
    pm.install_cron_persistence(&dummy_path)?;
    pm.install_launch_agent_persistence(&dummy_path)?;

    // Persistence should never be reported as installed
    assert!(!pm.is_persistence_installed());

    pm.remove_all_persistence()?;

    let status = pm.get_persistence_status();
    assert_eq!(status["simulation_mode"], true);
    assert!(status["safety_notice"].as_str().unwrap().contains("disabled"));

    Ok(())
}

#[test]
fn test_platform_manager() -> Result<()> {
    use stealth_loader::platform::PlatformManager;

    let pm = PlatformManager::new();
    
    // Test safe operations
    let user = pm.get_current_user()?;
    assert!(!user.is_empty());

    let _is_elevated = pm.is_elevated();
    // Should work regardless of actual privilege level

    // Test dangerous operations (should succeed but do nothing)
    pm.elevate_privileges()?;
    pm.hide_process(1234)?;
    pm.hide_file(&PathBuf::from("/tmp/test"))?;

    let info = pm.get_platform_info();
    assert_eq!(info["simulation_mode"], true);
    assert!(info["safety_notice"].as_str().unwrap().contains("disabled"));

    Ok(())
}

#[tokio::test]
async fn test_usb_monitor() -> Result<()> {
    use stealth_loader::usb_monitor::{UsbMonitor, device_db};

    let mut monitor = UsbMonitor::new();
    
    // Add trigger device
    monitor.add_trigger_device(0x1234, 0x5678, "Test Trigger".to_string());
    
    // Start monitoring (simulation)
    let mut receiver = monitor.start_monitoring().await?;
    
    // Should receive simulated events
    let event = tokio::time::timeout(
        tokio::time::Duration::from_secs(2),
        receiver.recv()
    ).await.map_err(|_| anyhow::anyhow!("Timeout waiting for USB event"))?
    .ok_or_else(|| anyhow::anyhow!("No USB event received"))?;

    // Verify event received
    assert!(matches!(event, stealth_loader::usb_monitor::UsbEvent::DeviceInserted(_)));

    // Test trigger detection (should be disabled)
    let device = device_db::create_simulated_device(0x1234, 0x5678);
    assert!(monitor.should_trigger(&device).is_none());

    let status = monitor.get_status();
    assert_eq!(status["simulation_mode"], true);
    assert!(status["safety_notice"].as_str().unwrap().contains("disabled"));

    monitor.stop_monitoring();
    Ok(())
}