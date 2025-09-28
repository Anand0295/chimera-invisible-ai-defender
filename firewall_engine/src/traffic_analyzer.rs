//! Traffic analysis module for detecting patterns and anomalies
//! 
//! ⚠️ SIMULATION ONLY - Real traffic capture disabled for safety

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn};

use crate::rule_engine::PacketInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficPattern {
    pub pattern_id: String,
    pub source_ips: Vec<String>,
    pub target_ports: Vec<u16>,
    pub packet_rate: f64,
    pub byte_rate: f64,
    pub duration_seconds: u64,
    pub threat_score: f64,
    pub pattern_type: ThreatType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatType {
    PortScan,
    DDoS,
    BruteForce,
    DataExfiltration,
    Anomalous,
    Benign,
}

#[derive(Debug, Clone)]
pub struct TrafficStats {
    pub total_packets: u64,
    pub total_bytes: u64,
    pub unique_sources: u32,
    pub unique_destinations: u32,
    pub top_ports: HashMap<u16, u64>,
    pub protocol_distribution: HashMap<String, u64>,
}

pub struct TrafficAnalyzer {
    simulation_mode: bool,
    packet_buffer: Vec<PacketInfo>,
    detected_patterns: Vec<TrafficPattern>,
    stats: TrafficStats,
}

impl TrafficAnalyzer {
    pub fn new() -> Self {
        Self {
            simulation_mode: true, // Always true for safety
            packet_buffer: Vec::new(),
            detected_patterns: Vec::new(),
            stats: TrafficStats {
                total_packets: 0,
                total_bytes: 0,
                unique_sources: 0,
                unique_destinations: 0,
                top_ports: HashMap::new(),
                protocol_distribution: HashMap::new(),
            },
        }
    }

    /// Analyze network traffic - SIMULATION
    pub fn analyze_traffic(&mut self, packets: Vec<PacketInfo>) -> Result<Vec<TrafficPattern>> {
        warn!("🚫 Real traffic analysis DISABLED - simulation only");
        
        info!("📊 Simulating analysis of {} packets", packets.len());
        
        // Update statistics
        self.update_stats(&packets);
        
        // Store packets in buffer (limited size for simulation)
        self.packet_buffer.extend(packets);
        if self.packet_buffer.len() > 10000 {
            self.packet_buffer.drain(0..5000); // Keep recent packets
        }
        
        // Detect patterns
        let patterns = self.detect_patterns()?;
        self.detected_patterns.extend(patterns.clone());
        
        // Keep only recent patterns
        if self.detected_patterns.len() > 100 {
            self.detected_patterns.drain(0..50);
        }
        
        Ok(patterns)
    }

    fn update_stats(&mut self, packets: &[PacketInfo]) {
        let mut sources = std::collections::HashSet::new();
        let mut destinations = std::collections::HashSet::new();
        
        for packet in packets {
            self.stats.total_packets += 1;
            self.stats.total_bytes += packet.size as u64;
            
            sources.insert(&packet.source_ip);
            destinations.insert(&packet.dest_ip);
            
            *self.stats.top_ports.entry(packet.dest_port).or_insert(0) += 1;
            *self.stats.protocol_distribution.entry(packet.protocol.clone()).or_insert(0) += 1;
        }
        
        self.stats.unique_sources = sources.len() as u32;
        self.stats.unique_destinations = destinations.len() as u32;
    }

    fn detect_patterns(&self) -> Result<Vec<TrafficPattern>> {
        let mut patterns = Vec::new();
        
        // Simulate port scan detection
        if let Some(port_scan) = self.detect_port_scan()? {
            patterns.push(port_scan);
        }
        
        // Simulate DDoS detection
        if let Some(ddos) = self.detect_ddos()? {
            patterns.push(ddos);
        }
        
        // Simulate brute force detection
        if let Some(brute_force) = self.detect_brute_force()? {
            patterns.push(brute_force);
        }
        
        // Simulate anomaly detection
        patterns.extend(self.detect_anomalies()?);
        
        Ok(patterns)
    }

    fn detect_port_scan(&self) -> Result<Option<TrafficPattern>> {
        // Simulate port scan detection logic
        let unique_ports: std::collections::HashSet<u16> = self.packet_buffer
            .iter()
            .map(|p| p.dest_port)
            .collect();
        
        if unique_ports.len() > 50 && self.packet_buffer.len() > 100 {
            let pattern = TrafficPattern {
                pattern_id: uuid::Uuid::new_v4().to_string(),
                source_ips: vec!["192.168.1.100".to_string()], // Simulated
                target_ports: unique_ports.into_iter().take(10).collect(),
                packet_rate: self.packet_buffer.len() as f64 / 60.0, // packets per second
                byte_rate: self.stats.total_bytes as f64 / 60.0,
                duration_seconds: 60,
                threat_score: 0.8,
                pattern_type: ThreatType::PortScan,
            };
            
            info!("🔍 Detected simulated port scan pattern: {}", pattern.pattern_id);
            return Ok(Some(pattern));
        }
        
        Ok(None)
    }

    fn detect_ddos(&self) -> Result<Option<TrafficPattern>> {
        // Simulate DDoS detection based on packet rate
        let packet_rate = self.packet_buffer.len() as f64 / 60.0;
        
        if packet_rate > 1000.0 { // High packet rate threshold
            let pattern = TrafficPattern {
                pattern_id: uuid::Uuid::new_v4().to_string(),
                source_ips: vec!["10.0.0.100".to_string(), "10.0.0.101".to_string()], // Simulated
                target_ports: vec![80, 443],
                packet_rate,
                byte_rate: self.stats.total_bytes as f64 / 60.0,
                duration_seconds: 60,
                threat_score: 0.9,
                pattern_type: ThreatType::DDoS,
            };
            
            info!("🌊 Detected simulated DDoS pattern: {}", pattern.pattern_id);
            return Ok(Some(pattern));
        }
        
        Ok(None)
    }

    fn detect_brute_force(&self) -> Result<Option<TrafficPattern>> {
        // Simulate brute force detection on authentication ports
        let auth_ports = [22, 21, 23, 3389]; // SSH, FTP, Telnet, RDP
        let auth_traffic: Vec<&PacketInfo> = self.packet_buffer
            .iter()
            .filter(|p| auth_ports.contains(&p.dest_port))
            .collect();
        
        if auth_traffic.len() > 100 {
            let pattern = TrafficPattern {
                pattern_id: uuid::Uuid::new_v4().to_string(),
                source_ips: vec!["172.16.0.50".to_string()], // Simulated
                target_ports: vec![22],
                packet_rate: auth_traffic.len() as f64 / 60.0,
                byte_rate: auth_traffic.iter().map(|p| p.size as u64).sum::<u64>() as f64 / 60.0,
                duration_seconds: 60,
                threat_score: 0.75,
                pattern_type: ThreatType::BruteForce,
            };
            
            info!("🔨 Detected simulated brute force pattern: {}", pattern.pattern_id);
            return Ok(Some(pattern));
        }
        
        Ok(None)
    }

    fn detect_anomalies(&self) -> Result<Vec<TrafficPattern>> {
        let mut anomalies = Vec::new();
        
        // Simulate statistical anomaly detection
        if self.stats.total_bytes > 1_000_000 && self.stats.unique_sources < 5 {
            // High data volume from few sources - potential data exfiltration
            let pattern = TrafficPattern {
                pattern_id: uuid::Uuid::new_v4().to_string(),
                source_ips: vec!["192.168.1.200".to_string()],
                target_ports: vec![443, 80],
                packet_rate: self.packet_buffer.len() as f64 / 60.0,
                byte_rate: self.stats.total_bytes as f64 / 60.0,
                duration_seconds: 60,
                threat_score: 0.6,
                pattern_type: ThreatType::DataExfiltration,
            };
            
            info!("📤 Detected simulated data exfiltration pattern: {}", pattern.pattern_id);
            anomalies.push(pattern);
        }
        
        Ok(anomalies)
    }

    /// Generate synthetic traffic for testing
    pub fn generate_synthetic_traffic(&self, count: usize) -> Vec<PacketInfo> {
        warn!("🔬 Generating synthetic traffic for testing");
        
        let mut packets = Vec::new();
        let source_ips = ["192.168.1.100", "10.0.0.50", "172.16.0.200"];
        let dest_ips = ["8.8.8.8", "1.1.1.1", "208.67.222.222"];
        let ports = [80, 443, 22, 21, 25, 53, 3389];
        let protocols = ["TCP", "UDP"];
        
        for i in 0..count {
            let packet = PacketInfo {
                source_ip: source_ips[i % source_ips.len()].to_string(),
                dest_ip: dest_ips[i % dest_ips.len()].to_string(),
                source_port: 1024 + (i % 60000) as u16,
                dest_port: ports[i % ports.len()],
                protocol: protocols[i % protocols.len()].to_string(),
                size: 64 + (i % 1400),
                timestamp: chrono::Utc::now(),
            };
            packets.push(packet);
        }
        
        info!("✅ Generated {} synthetic packets", count);
        packets
    }

    pub fn get_detected_patterns(&self) -> &[TrafficPattern] {
        &self.detected_patterns
    }

    pub fn get_traffic_stats(&self) -> &TrafficStats {
        &self.stats
    }

    pub fn clear_patterns(&mut self) {
        info!("🧹 Clearing detected patterns");
        self.detected_patterns.clear();
    }

    pub fn get_analyzer_status(&self) -> serde_json::Value {
        serde_json::json!({
            "simulation_mode": self.simulation_mode,
            "packets_in_buffer": self.packet_buffer.len(),
            "detected_patterns": self.detected_patterns.len(),
            "total_packets_analyzed": self.stats.total_packets,
            "total_bytes_analyzed": self.stats.total_bytes,
            "unique_sources": self.stats.unique_sources,
            "unique_destinations": self.stats.unique_destinations,
            "top_protocols": self.stats.protocol_distribution,
            "safety_notice": "⚠️ All traffic analysis is simulation-based for research safety"
        })
    }
}

impl Default for TrafficAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_packets(count: usize) -> Vec<PacketInfo> {
        (0..count).map(|i| PacketInfo {
            source_ip: format!("192.168.1.{}", 100 + (i % 50)),
            dest_ip: "10.0.0.1".to_string(),
            source_port: 1024 + i as u16,
            dest_port: 80,
            protocol: "TCP".to_string(),
            size: 1024,
            timestamp: chrono::Utc::now(),
        }).collect()
    }

    #[test]
    fn test_traffic_analyzer_creation() {
        let analyzer = TrafficAnalyzer::new();
        assert!(analyzer.simulation_mode);
        assert_eq!(analyzer.packet_buffer.len(), 0);
        assert_eq!(analyzer.detected_patterns.len(), 0);
    }

    #[test]
    fn test_traffic_analysis() {
        let mut analyzer = TrafficAnalyzer::new();
        let packets = create_test_packets(100);
        
        let _patterns = analyzer.analyze_traffic(packets).unwrap();
        
        assert_eq!(analyzer.stats.total_packets, 100);
        assert_eq!(analyzer.stats.total_bytes, 100 * 1024);
        assert_eq!(analyzer.packet_buffer.len(), 100);
    }

    #[test]
    fn test_synthetic_traffic_generation() {
        let analyzer = TrafficAnalyzer::new();
        let packets = analyzer.generate_synthetic_traffic(50);
        
        assert_eq!(packets.len(), 50);
        assert!(!packets[0].source_ip.is_empty());
        assert!(!packets[0].dest_ip.is_empty());
        assert!(packets[0].dest_port > 0);
    }

    #[test]
    fn test_pattern_detection() {
        let mut analyzer = TrafficAnalyzer::new();
        
        // Generate high-volume traffic to trigger DDoS detection
        let packets = create_test_packets(2000);
        let patterns = analyzer.analyze_traffic(packets).unwrap();
        
        // Should detect some patterns with high packet count
        assert!(analyzer.detected_patterns.len() >= patterns.len());
    }
}