//! Integration tests for firewall engine
//! 
//! ⚠️ These tests verify simulation behavior only - no real firewall operations

use anyhow::Result;
use firewall_engine::{
    FirewallConfig, FirewallEngine, FirewallRule, RuleAction, RuleSource,
    ai_interface::{AIInterface, TrafficFeatures},
    rule_engine::{RuleEngine, PacketInfo},
    traffic_analyzer::{TrafficAnalyzer, ThreatType},
    grpc_service::{GrpcService, RuleOperation, RuleUpdateRequest},
};
use serial_test::serial;
use std::path::PathBuf;
use tempfile::TempDir;
use tokio_test;

#[tokio::test]
#[serial]
async fn test_firewall_engine_lifecycle() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config = FirewallConfig {
        simulation_mode: true,
        enable_ai_rules: false,
        python_service_path: temp_dir.path().to_path_buf(),
        grpc_port: 50052,
        max_rules: 100,
        learning_rate: 0.01,
    };

    let mut engine = FirewallEngine::new(config)?;
    
    // Test startup
    engine.start().await?;
    
    let status = engine.get_status();
    assert_eq!(status["simulation_mode"], true);
    
    // Test rule addition
    let rule = create_test_rule();
    engine.add_rule(rule.clone())?;
    
    assert_eq!(engine.get_rules().len(), 1);
    assert!(engine.get_rules().contains_key(&rule.id));
    
    // Test rule removal
    engine.remove_rule(&rule.id)?;
    assert_eq!(engine.get_rules().len(), 0);
    
    // Test shutdown
    engine.shutdown().await?;
    
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_ai_interface_simulation() -> Result<()> {
    let mut ai = AIInterface::new()?;
    
    // Test Python service initialization (should be disabled)
    ai.initialize_python_service("test/path")?;
    
    // Test feature extraction
    let traffic_data = vec![0u8; 1000];
    let features = ai.extract_features(&traffic_data)?;
    
    assert_eq!(features.byte_count, 1000);
    assert!(features.packet_count > 0);
    
    // Test AI recommendations
    let recommendations = ai.get_ai_recommendations(&features)?;
    
    // Should generate some recommendations for testing
    if !recommendations.is_empty() {
        assert!(recommendations[0].confidence > 0.0);
        assert!(!recommendations[0].reasoning.is_empty());
    }
    
    // Test model stats
    let stats = ai.get_model_stats();
    assert_eq!(stats["simulation_mode"], true);
    
    Ok(())
}

#[test]
fn test_rule_engine_operations() -> Result<()> {
    let mut engine = RuleEngine::new();
    
    // Test rule application
    let rule = create_test_rule();
    engine.apply_rule(rule.clone())?;
    
    assert_eq!(engine.get_active_rules().len(), 1);
    assert!(engine.get_rule_stats().contains_key(&rule.id));
    
    // Test traffic processing
    let packet = create_test_packet();
    let action = engine.process_traffic(&packet)?;
    
    // Should match the rule and return Block action
    assert!(matches!(action, RuleAction::Block));
    
    // Check stats were updated
    let stats = engine.get_rule_stats().get(&rule.id).unwrap();
    assert_eq!(stats.matches, 1);
    assert_eq!(stats.bytes_processed, 1024);
    
    // Test rule removal
    engine.remove_rule(&rule.id)?;
    assert_eq!(engine.get_active_rules().len(), 0);
    
    // Test clear all rules
    engine.apply_rule(create_test_rule())?;
    engine.clear_all_rules()?;
    assert_eq!(engine.get_active_rules().len(), 0);
    
    Ok(())
}

#[tokio::test]
async fn test_traffic_analyzer() -> Result<()> {
    let mut analyzer = TrafficAnalyzer::new();
    
    // Test synthetic traffic generation
    let packets = analyzer.generate_synthetic_traffic(100);
    assert_eq!(packets.len(), 100);
    
    // Test traffic analysis
    let patterns = analyzer.analyze_traffic(packets).await?;
    
    // Should have updated statistics
    let stats = analyzer.get_traffic_stats();
    assert_eq!(stats.total_packets, 100);
    assert!(stats.total_bytes > 0);
    
    // Test pattern detection with high-volume traffic
    let high_volume_packets = analyzer.generate_synthetic_traffic(2000);
    let patterns = analyzer.analyze_traffic(high_volume_packets).await?;
    
    // Should detect some patterns with high packet count
    assert!(analyzer.get_detected_patterns().len() >= patterns.len());
    
    // Test status
    let status = analyzer.get_analyzer_status();
    assert_eq!(status["simulation_mode"], true);
    
    Ok(())
}

#[tokio::test]
async fn test_grpc_service_simulation() -> Result<()> {
    let mut service = GrpcService::new();
    
    // Test service startup
    let _rx = service.start(50053).await?;
    
    // Test rule update handling
    let request = service.create_test_request(RuleOperation::Add);
    let response = service.handle_rule_update(request).await?;
    
    assert!(response.success);
    assert!(response.rule_id.is_some());
    
    // Test status handling
    let status_request = firewall_engine::grpc_service::StatusRequest {};
    let status_response = service.handle_status_request(status_request).await?;
    
    assert!(status_response.simulation_mode);
    
    // Test client simulation
    let responses = service.simulate_client_requests(5).await?;
    assert_eq!(responses.len(), 5);
    assert!(responses.iter().all(|r| r.success));
    
    // Test service stats
    let stats = service.get_service_stats();
    assert_eq!(stats["simulation_mode"], true);
    assert!(stats["requests_processed"].as_u64().unwrap() > 0);
    
    // Test shutdown
    service.shutdown().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_end_to_end_simulation() -> Result<()> {
    // Test complete firewall engine workflow
    let temp_dir = TempDir::new()?;
    let config = FirewallConfig {
        simulation_mode: true,
        enable_ai_rules: false,
        python_service_path: temp_dir.path().to_path_buf(),
        grpc_port: 50054,
        max_rules: 1000,
        learning_rate: 0.01,
    };

    let mut engine = FirewallEngine::new(config)?;
    engine.start().await?;
    
    // Generate synthetic traffic
    let traffic_data = vec![0u8; 5000];
    
    // Analyze traffic and generate rules
    let ai_rules = engine.analyze_traffic(&traffic_data)?;
    
    // Add generated rules
    for rule in ai_rules {
        engine.add_rule(rule)?;
    }
    
    // Verify rules were added
    assert!(engine.get_rules().len() > 0);
    
    // Test status reporting
    let status = engine.get_status();
    assert_eq!(status["simulation_mode"], true);
    assert!(status["total_rules"].as_u64().unwrap() > 0);
    
    // Cleanup
    engine.shutdown().await?;
    
    Ok(())
}

#[test]
fn test_safety_enforcement() -> Result<()> {
    // Test that dangerous configurations are automatically disabled
    let config = FirewallConfig {
        simulation_mode: false, // Try to disable simulation
        enable_ai_rules: true,  // Try to enable AI rules
        python_service_path: PathBuf::from("/dangerous/path"),
        grpc_port: 80, // Privileged port
        max_rules: 10000,
        learning_rate: 1.0, // Dangerous learning rate
    };

    let engine = FirewallEngine::new(config)?;
    let status = engine.get_status();

    // Verify safety measures are enforced
    assert_eq!(status["simulation_mode"], true);
    assert!(status["safety_notice"].as_str().unwrap().contains("disabled"));

    Ok(())
}

// Helper functions
fn create_test_rule() -> FirewallRule {
    FirewallRule {
        id: uuid::Uuid::new_v4().to_string(),
        source_ip: Some("192.168.1.100".to_string()),
        dest_ip: None,
        source_port: None,
        dest_port: Some(80),
        protocol: "TCP".to_string(),
        action: RuleAction::Block,
        confidence: 0.9,
        created_by: RuleSource::AI,
        timestamp: chrono::Utc::now(),
    }
}

fn create_test_packet() -> PacketInfo {
    PacketInfo {
        source_ip: "192.168.1.100".to_string(),
        dest_ip: "10.0.0.1".to_string(),
        source_port: 12345,
        dest_port: 80,
        protocol: "TCP".to_string(),
        size: 1024,
        timestamp: chrono::Utc::now(),
    }
}