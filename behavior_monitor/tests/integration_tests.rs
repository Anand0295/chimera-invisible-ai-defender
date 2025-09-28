//! Integration tests for behavior monitor
//! 
//! ⚠️ These tests verify simulation behavior only - no real system monitoring

use anyhow::Result;
use behavior_monitor::{
    BehaviorMonitor, MonitorConfig, BehaviorEvent, EventType,
    file_monitor::FileMonitor,
    process_monitor::ProcessMonitor,
    anomaly_detector::AnomalyDetector,
};
use serial_test::serial;
use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::TempDir;

#[tokio::test]
#[serial]
async fn test_behavior_monitor_lifecycle() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config = MonitorConfig {
        simulation_mode: true,
        enable_file_monitoring: false,
        enable_process_monitoring: false,
        watch_paths: vec![temp_dir.path().to_path_buf()],
        anomaly_threshold: 0.8,
    };

    let mut monitor = BehaviorMonitor::new(config)?;
    
    // Test startup
    monitor.start().await?;
    
    let status = monitor.get_status();
    assert_eq!(status["simulation_mode"], true);
    
    // Test event addition
    let event = create_test_event();
    monitor.add_event(event.clone());
    
    assert_eq!(monitor.get_events().len(), 1);
    assert_eq!(monitor.get_events()[0].id, event.id);
    
    // Test shutdown
    monitor.stop().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_file_monitor_simulation() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let mut monitor = FileMonitor::new(vec![temp_dir.path().to_path_buf()]);
    
    // Test monitoring startup
    monitor.start_monitoring().await?;
    
    // Should have some simulated integrity records
    assert!(monitor.get_integrity_records().len() > 0);
    
    // Test file hash calculation
    let hash = monitor.calculate_file_hash(&PathBuf::from("/tmp/test"))?;
    assert!(!hash.is_empty());
    
    // Test event generation
    let events = monitor.generate_file_events(5);
    assert_eq!(events.len(), 5);
    assert!(events.iter().any(|e| matches!(e.event_type, EventType::FileCreated)));
    
    // Test status
    let status = monitor.get_monitor_status();
    assert_eq!(status["simulation_mode"], true);
    
    Ok(())
}

#[tokio::test]
async fn test_process_monitor_simulation() -> Result<()> {
    let mut monitor = ProcessMonitor::new();
    
    // Test monitoring startup
    monitor.start_monitoring().await?;
    
    // Should have some simulated processes
    assert!(monitor.get_tracked_processes().len() > 0);
    
    // Test process enumeration
    let processes = monitor.get_running_processes()?;
    assert!(!processes.is_empty());
    
    // Test suspicious process detection
    let first_process = &processes[0];
    let _is_suspicious = monitor.is_suspicious_process(first_process);
    
    // Test event generation
    let events = monitor.generate_process_events(3);
    assert_eq!(events.len(), 3);
    assert!(events.iter().any(|e| matches!(e.event_type, EventType::ProcessStarted)));
    
    // Test process termination (simulation)
    monitor.terminate_process(1234)?;
    
    // Test status
    let status = monitor.get_monitor_status();
    assert_eq!(status["simulation_mode"], true);
    
    Ok(())
}

#[test]
fn test_anomaly_detector() -> Result<()> {
    let mut detector = AnomalyDetector::new(0.8);
    
    // Test model initialization
    detector.initialize_model()?;
    
    // Test feature extraction
    let event = create_test_event();
    let features = detector.extract_features(&event);
    assert!(features.contains_key("risk_score"));
    assert!(features.contains_key("event_type_risk"));
    
    // Test anomaly detection
    let score = detector.detect_anomaly(&event)?;
    assert_eq!(score.event_id, event.id);
    assert!(score.score >= 0.0 && score.score <= 1.0);
    
    // Test batch processing
    let events = vec![
        create_test_event(),
        create_high_risk_event(),
        create_test_event(),
    ];
    
    let results = detector.process_batch(&events)?;
    assert_eq!(results.len(), 3);
    
    // Test model update (simulation)
    let feedback = vec![
        ("event1".to_string(), true),
        ("event2".to_string(), false),
    ];
    detector.update_model(&feedback)?;
    
    // Test stats
    let stats = detector.get_stats();
    assert_eq!(stats.samples_processed, 4); // 1 + 3 from batch
    
    // Test status
    let status = detector.get_detector_status();
    assert_eq!(status["simulation_mode"], true);
    
    Ok(())
}

#[tokio::test]
async fn test_end_to_end_monitoring() -> Result<()> {
    // Test complete monitoring workflow
    let temp_dir = TempDir::new()?;
    let config = MonitorConfig {
        simulation_mode: true,
        enable_file_monitoring: false,
        enable_process_monitoring: false,
        watch_paths: vec![temp_dir.path().to_path_buf()],
        anomaly_threshold: 0.7,
    };

    let mut behavior_monitor = BehaviorMonitor::new(config)?;
    behavior_monitor.start().await?;
    
    // Set up component monitors
    let mut file_monitor = FileMonitor::new(vec![temp_dir.path().to_path_buf()]);
    let mut process_monitor = ProcessMonitor::new();
    let mut anomaly_detector = AnomalyDetector::new(0.7);
    
    // Initialize all monitors
    file_monitor.start_monitoring().await?;
    process_monitor.start_monitoring().await?;
    anomaly_detector.initialize_model()?;
    
    // Generate events from different sources
    let file_events = file_monitor.generate_file_events(3);
    let process_events = process_monitor.generate_process_events(3);
    
    // Combine all events
    let mut all_events = Vec::new();
    all_events.extend(file_events);
    all_events.extend(process_events);
    
    // Add events to behavior monitor
    for event in &all_events {
        behavior_monitor.add_event(event.clone());
    }
    
    // Run anomaly detection on all events
    let anomaly_scores = anomaly_detector.process_batch(&all_events)?;
    
    // Verify results
    assert_eq!(behavior_monitor.get_events().len(), 6);
    assert_eq!(anomaly_scores.len(), 6);
    
    let high_risk_events = behavior_monitor.get_high_risk_events();
    assert!(high_risk_events.len() <= 6);
    
    // Test status reporting
    let status = behavior_monitor.get_status();
    assert_eq!(status["simulation_mode"], true);
    assert_eq!(status["total_events"], 6);
    
    // Cleanup
    behavior_monitor.stop().await?;
    
    Ok(())
}

#[test]
fn test_safety_enforcement() -> Result<()> {
    // Test that dangerous configurations are automatically disabled
    let config = MonitorConfig {
        simulation_mode: false, // Try to disable simulation
        enable_file_monitoring: true, // Try to enable real monitoring
        enable_process_monitoring: true, // Try to enable real monitoring
        watch_paths: vec![PathBuf::from("/")], // Dangerous path
        anomaly_threshold: 0.0, // Dangerous threshold
    };

    let monitor = BehaviorMonitor::new(config)?;
    let status = monitor.get_status();

    // Verify safety measures are enforced
    assert_eq!(status["simulation_mode"], true);
    assert!(status["safety_notice"].as_str().unwrap().contains("disabled"));

    Ok(())
}

// Helper functions
fn create_test_event() -> BehaviorEvent {
    let mut details = HashMap::new();
    details.insert("path".to_string(), "/tmp/test.txt".to_string());
    
    BehaviorEvent {
        id: uuid::Uuid::new_v4().to_string(),
        event_type: EventType::FileModified,
        timestamp: chrono::Utc::now(),
        source: "test".to_string(),
        details,
        risk_score: 0.3,
    }
}

fn create_high_risk_event() -> BehaviorEvent {
    let mut details = HashMap::new();
    details.insert("path".to_string(), "/etc/passwd".to_string());
    
    BehaviorEvent {
        id: uuid::Uuid::new_v4().to_string(),
        event_type: EventType::FileDeleted,
        timestamp: chrono::Utc::now(),
        source: "test".to_string(),
        details,
        risk_score: 0.9,
    }
}