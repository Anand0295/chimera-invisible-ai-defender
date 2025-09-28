//! Integration tests for network forensics
//! 
//! ⚠️ These tests verify simulation behavior only - no real network operations

use anyhow::Result;
use network_forensics::{
    NetworkForensics, ForensicsConfig, NetworkEvent,
    packet_analyzer::PacketAnalyzer,
    traceback::NetworkTraceback,
    dns_resolver::DnsResolver,
};
use serial_test::serial;
use std::net::IpAddr;

#[tokio::test]
#[serial]
async fn test_network_forensics_lifecycle() -> Result<()> {
    let config = ForensicsConfig {
        simulation_mode: true,
        enable_packet_capture: false,
        capture_interface: "sim0".to_string(),
        max_packets: 1000,
        analysis_depth: 3,
    };

    let mut forensics = NetworkForensics::new(config)?;
    
    // Test startup
    forensics.start_capture().await?;
    
    let status = forensics.get_status();
    assert_eq!(status["simulation_mode"], true);
    assert_eq!(status["is_capturing"], true);
    
    // Test event addition
    let event = create_test_event();
    forensics.add_network_event(event.clone());
    
    assert_eq!(forensics.get_events().len(), 1);
    assert_eq!(forensics.get_events()[0].id, event.id);
    
    // Test IP filtering
    let ip_events = forensics.get_events_by_ip(event.source_ip);
    assert_eq!(ip_events.len(), 1);
    
    // Test shutdown
    forensics.stop_capture().await?;
    
    Ok(())
}

#[test]
fn test_packet_analyzer() -> Result<()> {
    let analyzer = PacketAnalyzer::new();
    
    // Test event generation
    let events = analyzer.generate_network_events(5);
    assert_eq!(events.len(), 5);
    
    // Test packet analysis
    let event = &events[0];
    let analysis = analyzer.analyze_packet(event)?;
    
    assert_eq!(analysis.event_id, event.id);
    assert_eq!(analysis.protocol_analysis.protocol, event.protocol);
    assert!(analysis.reputation_score >= 0.0 && analysis.reputation_score <= 1.0);
    
    // Test status
    let status = analyzer.get_analyzer_status();
    assert_eq!(status["simulation_mode"], true);
    
    Ok(())
}

#[tokio::test]
async fn test_network_traceback() -> Result<()> {
    let traceback = NetworkTraceback::new();
    
    // Test traceback
    let target_ip: IpAddr = "8.8.8.8".parse()?;
    let result = traceback.trace_route(target_ip).await?;
    
    assert_eq!(result.target_ip, target_ip);
    assert!(result.hop_count > 0);
    assert!(!result.route_hops.is_empty());
    assert!(result.confidence > 0.0);
    
    // Test reverse DNS
    let hostname = traceback.reverse_dns_lookup(target_ip).await?;
    assert!(hostname.is_some());
    
    // Test anonymization detection
    let normal_ip: IpAddr = "8.8.8.8".parse()?;
    let tor_ip: IpAddr = "185.1.2.3".parse()?;
    
    assert!(!traceback.detect_anonymization(normal_ip)?);
    assert!(traceback.detect_anonymization(tor_ip)?);
    
    // Test geolocation
    let location = traceback.geolocate_ip(target_ip).await?;
    assert!(location.is_some());
    
    // Test status
    let status = traceback.get_traceback_status();
    assert_eq!(status["simulation_mode"], true);
    
    Ok(())
}

#[tokio::test]
async fn test_dns_resolver() -> Result<()> {
    let mut resolver = DnsResolver::new();
    
    // Test domain resolution
    let records = resolver.resolve_domain("google.com").await?;
    assert!(!records.is_empty());
    assert!(records.iter().any(|r| r.record_type == "A"));
    
    // Test domain analysis
    let analysis = resolver.analyze_domain("google.com").await?;
    assert_eq!(analysis.domain, "google.com");
    assert!(analysis.reputation_score > 0.5);
    assert!(!analysis.is_dga);
    
    // Test malicious domain analysis
    let malicious_analysis = resolver.analyze_domain("malware.example.com").await?;
    assert!(malicious_analysis.reputation_score < 0.5);
    assert!(!malicious_analysis.suspicious_indicators.is_empty());
    
    // Test reverse lookup
    let ip: IpAddr = "8.8.8.8".parse()?;
    let hostname = resolver.reverse_lookup(ip).await?;
    assert!(hostname.is_some());
    
    // Test reputation check
    assert!(resolver.check_reputation("google.com") > 0.9);
    assert!(resolver.check_reputation("malware.example.com") < 0.2);
    
    // Test status
    let status = resolver.get_resolver_status();
    assert_eq!(status["simulation_mode"], true);
    
    Ok(())
}

#[tokio::test]
async fn test_end_to_end_forensics() -> Result<()> {
    // Test complete forensics workflow
    let config = ForensicsConfig {
        simulation_mode: true,
        enable_packet_capture: false,
        capture_interface: "sim0".to_string(),
        max_packets: 1000,
        analysis_depth: 5,
    };

    let mut forensics = NetworkForensics::new(config)?;
    forensics.start_capture().await?;
    
    // Set up analysis components
    let analyzer = PacketAnalyzer::new();
    let traceback = NetworkTraceback::new();
    let mut resolver = DnsResolver::new();
    
    // Generate network events
    let events = analyzer.generate_network_events(10);
    
    // Add events to forensics system
    for event in &events {
        forensics.add_network_event(event.clone());
    }
    
    // Analyze each event
    for event in &events {
        // Packet analysis
        let packet_analysis = analyzer.analyze_packet(event)?;
        assert_eq!(packet_analysis.event_id, event.id);
        
        // Network traceback
        let traceback_result = traceback.trace_route(event.source_ip).await?;
        assert_eq!(traceback_result.target_ip, event.source_ip);
        
        // DNS analysis (if applicable)
        if let Some(hostname) = traceback.reverse_dns_lookup(event.source_ip).await? {
            let dns_analysis = resolver.analyze_domain(&hostname).await?;
            assert_eq!(dns_analysis.domain, hostname);
        }
    }
    
    // Verify all events were captured
    assert_eq!(forensics.get_events().len(), 10);
    
    // Test filtering by IP
    let first_event = &events[0];
    let ip_events = forensics.get_events_by_ip(first_event.source_ip);
    assert!(!ip_events.is_empty());
    
    // Test status reporting
    let status = forensics.get_status();
    assert_eq!(status["simulation_mode"], true);
    assert_eq!(status["total_events"], 10);
    
    // Cleanup
    forensics.stop_capture().await?;
    
    Ok(())
}

#[test]
fn test_safety_enforcement() -> Result<()> {
    // Test that dangerous configurations are automatically disabled
    let config = ForensicsConfig {
        simulation_mode: false, // Try to disable simulation
        enable_packet_capture: true, // Try to enable real capture
        capture_interface: "eth0".to_string(), // Real interface
        max_packets: 1000000, // Large number
        analysis_depth: 10,
    };

    let forensics = NetworkForensics::new(config)?;
    let status = forensics.get_status();

    // Verify safety measures are enforced
    assert_eq!(status["simulation_mode"], true);
    assert!(status["safety_notice"].as_str().unwrap().contains("disabled"));

    Ok(())
}

// Helper functions
fn create_test_event() -> NetworkEvent {
    NetworkEvent {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: chrono::Utc::now(),
        source_ip: "192.168.1.100".parse().unwrap(),
        dest_ip: "8.8.8.8".parse().unwrap(),
        source_port: 12345,
        dest_port: 80,
        protocol: "TCP".to_string(),
        packet_size: 1024,
        flags: vec!["SYN".to_string()],
        payload_hash: Some("abc123def456".to_string()),
    }
}