//! Network Forensics Unit
//! 
//! ⚠️ EXPERIMENTAL USE ONLY - LAB ENVIRONMENT RESEARCH PROJECT ⚠️
//! 
//! This module simulates network forensics and packet analysis.
//! All real packet capture is DISABLED by default.

use anyhow::Result;
use serde::{Deserialize, Serialize};
// use std::collections::HashMap; // Unused
use std::net::IpAddr;
use tracing::{info, warn};

pub mod packet_analyzer;
pub mod traceback;
pub mod dns_resolver;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForensicsConfig {
    pub simulation_mode: bool,
    pub enable_packet_capture: bool,
    pub capture_interface: String,
    pub max_packets: usize,
    pub analysis_depth: u8,
}

impl Default for ForensicsConfig {
    fn default() -> Self {
        Self {
            simulation_mode: true, // Always default to simulation
            enable_packet_capture: false, // Disabled by default
            capture_interface: "sim0".to_string(),
            max_packets: 10000,
            analysis_depth: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEvent {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub source_ip: IpAddr,
    pub dest_ip: IpAddr,
    pub source_port: u16,
    pub dest_port: u16,
    pub protocol: String,
    pub packet_size: usize,
    pub flags: Vec<String>,
    pub payload_hash: Option<String>,
}

pub struct NetworkForensics {
    config: ForensicsConfig,
    captured_events: Vec<NetworkEvent>,
    is_capturing: bool,
}

impl NetworkForensics {
    pub fn new(config: ForensicsConfig) -> Result<Self> {
        // Force simulation mode for safety
        let mut safe_config = config;
        safe_config.simulation_mode = true;
        
        if safe_config.enable_packet_capture {
            warn!("⚠️ Real packet capture is disabled in this research build");
            safe_config.enable_packet_capture = false;
        }

        Ok(Self {
            config: safe_config,
            captured_events: Vec::new(),
            is_capturing: false,
        })
    }

    pub async fn start_capture(&mut self) -> Result<()> {
        info!("🔬 Starting network forensics (SIMULATION MODE)");
        
        if !self.config.simulation_mode {
            return Err(anyhow::anyhow!("Real packet capture is disabled for safety"));
        }

        self.is_capturing = true;
        
        // Simulate packet capture initialization
        self.simulate_capture_setup().await?;
        
        info!("✅ Network forensics simulation started successfully");
        Ok(())
    }

    async fn simulate_capture_setup(&self) -> Result<()> {
        warn!("🚫 Real packet capture DISABLED - simulation only");
        
        // In a real implementation, this would:
        // - Initialize libpcap or similar
        // - Set up network interface monitoring
        // - Configure packet filters
        // - Start capture thread
        
        info!("📝 Would capture on interface: {}", self.config.capture_interface);
        Ok(())
    }

    pub fn add_network_event(&mut self, event: NetworkEvent) {
        info!("📊 Recording network event: {}:{} -> {}:{}", 
              event.source_ip, event.source_port, 
              event.dest_ip, event.dest_port);
        
        self.captured_events.push(event);
        
        // Keep only recent events
        if self.captured_events.len() > self.config.max_packets {
            self.captured_events.drain(0..self.config.max_packets / 2);
        }
    }

    pub fn get_events(&self) -> &[NetworkEvent] {
        &self.captured_events
    }

    pub fn get_events_by_ip(&self, ip: IpAddr) -> Vec<&NetworkEvent> {
        self.captured_events.iter()
            .filter(|e| e.source_ip == ip || e.dest_ip == ip)
            .collect()
    }

    pub fn get_status(&self) -> serde_json::Value {
        serde_json::json!({
            "simulation_mode": self.config.simulation_mode,
            "is_capturing": self.is_capturing,
            "total_events": self.captured_events.len(),
            "capture_interface": self.config.capture_interface,
            "max_packets": self.config.max_packets,
            "safety_notice": "⚠️ All packet capture disabled for research safety"
        })
    }

    pub async fn stop_capture(&mut self) -> Result<()> {
        info!("🛑 Stopping network forensics simulation");
        self.is_capturing = false;
        info!("✅ Network forensics simulation stopped");
        Ok(())
    }
}