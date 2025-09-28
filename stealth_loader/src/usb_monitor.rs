//! USB monitoring simulation module
//! 
//! ⚠️ USB MONITORING IS DISABLED FOR SAFETY
//! This module only provides simulation for research purposes

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbDevice {
    pub vendor_id: u16,
    pub product_id: u16,
    pub serial_number: Option<String>,
    pub device_path: String,
    pub device_name: String,
}

#[derive(Debug, Clone)]
pub enum UsbEvent {
    DeviceInserted(UsbDevice),
    DeviceRemoved(String), // device_path
}

pub struct UsbMonitor {
    simulation_mode: bool,
    event_sender: Option<mpsc::UnboundedSender<UsbEvent>>,
    trigger_devices: HashMap<(u16, u16), String>, // (vendor_id, product_id) -> trigger_name
}

impl UsbMonitor {
    pub fn new() -> Self {
        Self {
            simulation_mode: true, // Always true for safety
            event_sender: None,
            trigger_devices: HashMap::new(),
        }
    }

    /// Add a USB device that should trigger the stealth loader - DISABLED
    pub fn add_trigger_device(&mut self, vendor_id: u16, product_id: u16, trigger_name: String) {
        warn!("🚫 USB triggers DISABLED - simulation only");
        info!("📝 Would monitor USB device: {:04x}:{:04x} ({})", vendor_id, product_id, trigger_name);
        self.trigger_devices.insert((vendor_id, product_id), trigger_name);
    }

    /// Start monitoring USB events - DISABLED
    pub async fn start_monitoring(&mut self) -> Result<mpsc::UnboundedReceiver<UsbEvent>> {
        warn!("🚫 USB monitoring DISABLED - simulation only");
        
        let (sender, receiver) = mpsc::unbounded_channel();
        self.event_sender = Some(sender);

        // Simulate some USB events for testing
        if self.simulation_mode {
            self.simulate_usb_events().await?;
        }

        Ok(receiver)
    }

    /// Simulate USB events for testing
    async fn simulate_usb_events(&self) -> Result<()> {
        if let Some(sender) = &self.event_sender {
            info!("🔬 Simulating USB events for testing");

            // Simulate a USB device insertion
            let simulated_device = UsbDevice {
                vendor_id: 0x1234,
                product_id: 0x5678,
                serial_number: Some("SIM123456".to_string()),
                device_path: "/dev/sim_usb0".to_string(),
                device_name: "Simulated USB Device".to_string(),
            };

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            let _ = sender.send(UsbEvent::DeviceInserted(simulated_device.clone()));

            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            let _ = sender.send(UsbEvent::DeviceRemoved(simulated_device.device_path));
        }

        Ok(())
    }

    /// Check if a USB device should trigger the loader - DISABLED
    pub fn should_trigger(&self, device: &UsbDevice) -> Option<&String> {
        if !self.simulation_mode {
            return None; // Never trigger in real mode
        }

        let key = (device.vendor_id, device.product_id);
        let trigger_name = self.trigger_devices.get(&key);
        
        if trigger_name.is_some() {
            warn!("🚫 USB trigger detected but DISABLED for safety: {:?}", trigger_name);
        }
        
        None // Always return None to prevent triggering
    }

    /// Stop monitoring
    pub fn stop_monitoring(&mut self) {
        info!("🛑 Stopping USB monitoring simulation");
        self.event_sender = None;
    }

    /// Get monitoring status
    pub fn get_status(&self) -> serde_json::Value {
        serde_json::json!({
            "simulation_mode": self.simulation_mode,
            "monitoring_active": self.event_sender.is_some(),
            "trigger_devices_count": self.trigger_devices.len(),
            "trigger_devices": self.trigger_devices.iter().map(|((vid, pid), name)| {
                serde_json::json!({
                    "vendor_id": format!("{:04x}", vid),
                    "product_id": format!("{:04x}", pid),
                    "trigger_name": name
                })
            }).collect::<Vec<_>>(),
            "safety_notice": "⚠️ USB monitoring and triggers disabled for research safety"
        })
    }
}

impl Default for UsbMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// USB device database for common devices (for simulation)
pub mod device_db {
    use super::UsbDevice;
    use std::collections::HashMap;

    pub fn get_common_devices() -> HashMap<(u16, u16), &'static str> {
        let mut devices = HashMap::new();
        
        // Common USB vendors (for simulation purposes)
        devices.insert((0x0781, 0x5567), "SanDisk Cruzer Blade");
        devices.insert((0x058f, 0x6387), "Generic USB Flash Drive");
        devices.insert((0x090c, 0x1000), "Samsung Flash Drive");
        devices.insert((0x0951, 0x1666), "Kingston DataTraveler");
        
        devices
    }

    pub fn create_simulated_device(vendor_id: u16, product_id: u16) -> UsbDevice {
        let device_name = get_common_devices()
            .get(&(vendor_id, product_id))
            .unwrap_or(&"Unknown USB Device")
            .to_string();

        UsbDevice {
            vendor_id,
            product_id,
            serial_number: Some(format!("SIM{:08x}", rand::random::<u32>())),
            device_path: format!("/dev/sim_usb_{:04x}_{:04x}", vendor_id, product_id),
            device_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_usb_monitor_creation() {
        let monitor = UsbMonitor::new();
        assert!(monitor.simulation_mode);
        assert!(monitor.event_sender.is_none());
    }

    #[tokio::test]
    async fn test_trigger_device_management() {
        let mut monitor = UsbMonitor::new();
        monitor.add_trigger_device(0x1234, 0x5678, "Test Trigger".to_string());
        
        let device = device_db::create_simulated_device(0x1234, 0x5678);
        
        // Should not trigger due to safety measures
        assert!(monitor.should_trigger(&device).is_none());
    }

    #[tokio::test]
    async fn test_monitoring_simulation() {
        let mut monitor = UsbMonitor::new();
        let mut receiver = monitor.start_monitoring().await.unwrap();
        
        // Should receive simulated events
        let event = receiver.recv().await;
        assert!(event.is_some());
        
        match event.unwrap() {
            UsbEvent::DeviceInserted(device) => {
                assert_eq!(device.vendor_id, 0x1234);
                assert_eq!(device.product_id, 0x5678);
            }
            _ => panic!("Expected DeviceInserted event"),
        }
    }
}