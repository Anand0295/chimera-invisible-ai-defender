//! Firewall rule engine for managing and applying rules
//! 
//! ⚠️ SIMULATION ONLY - Real firewall rule application disabled for safety

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn};

use crate::{FirewallRule, RuleAction};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleStats {
    pub rule_id: String,
    pub matches: u64,
    pub bytes_processed: u64,
    pub last_match: Option<chrono::DateTime<chrono::Utc>>,
    pub effectiveness_score: f64,
}

pub struct RuleEngine {
    simulation_mode: bool,
    active_rules: HashMap<String, FirewallRule>,
    rule_stats: HashMap<String, RuleStats>,
}

impl RuleEngine {
    pub fn new() -> Self {
        Self {
            simulation_mode: true, // Always true for safety
            active_rules: HashMap::new(),
            rule_stats: HashMap::new(),
        }
    }

    /// Apply a firewall rule - DISABLED
    pub fn apply_rule(&mut self, rule: FirewallRule) -> Result<()> {
        warn!("🚫 Firewall rule application DISABLED - simulation only");
        
        info!("📝 Would apply firewall rule: {} ({:?})", rule.id, rule.action);
        
        // Simulate rule application based on platform
        self.simulate_rule_application(&rule)?;
        
        // Store rule and initialize stats
        self.rule_stats.insert(rule.id.clone(), RuleStats {
            rule_id: rule.id.clone(),
            matches: 0,
            bytes_processed: 0,
            last_match: None,
            effectiveness_score: 0.0,
        });
        
        self.active_rules.insert(rule.id.clone(), rule);
        Ok(())
    }

    fn simulate_rule_application(&self, rule: &FirewallRule) -> Result<()> {
        match rule.action {
            RuleAction::Allow => {
                info!("🟢 Simulating ALLOW rule for traffic matching: {}", self.format_rule_criteria(rule));
            }
            RuleAction::Block => {
                info!("🔴 Simulating BLOCK rule for traffic matching: {}", self.format_rule_criteria(rule));
                // In real implementation: iptables -A INPUT -s source_ip -j DROP
            }
            RuleAction::Log => {
                info!("📋 Simulating LOG rule for traffic matching: {}", self.format_rule_criteria(rule));
                // In real implementation: iptables -A INPUT -s source_ip -j LOG
            }
            RuleAction::RateLimit(limit) => {
                info!("⏱️ Simulating RATE LIMIT ({} pps) for: {}", limit, self.format_rule_criteria(rule));
                // In real implementation: iptables -A INPUT -s source_ip -m limit --limit {}/sec -j ACCEPT
            }
        }
        Ok(())
    }

    fn format_rule_criteria(&self, rule: &FirewallRule) -> String {
        let mut criteria = Vec::new();
        
        if let Some(src_ip) = &rule.source_ip {
            criteria.push(format!("src:{}", src_ip));
        }
        if let Some(dst_ip) = &rule.dest_ip {
            criteria.push(format!("dst:{}", dst_ip));
        }
        if let Some(src_port) = rule.source_port {
            criteria.push(format!("sport:{}", src_port));
        }
        if let Some(dst_port) = rule.dest_port {
            criteria.push(format!("dport:{}", dst_port));
        }
        criteria.push(format!("proto:{}", rule.protocol));
        
        criteria.join(" ")
    }

    /// Remove a firewall rule - DISABLED
    pub fn remove_rule(&mut self, rule_id: &str) -> Result<()> {
        warn!("🚫 Firewall rule removal DISABLED - simulation only");
        
        if let Some(rule) = self.active_rules.remove(rule_id) {
            info!("🗑️ Simulating removal of firewall rule: {}", rule_id);
            
            // In real implementation, would remove from iptables/netfilter
            self.simulate_rule_removal(&rule)?;
            
            self.rule_stats.remove(rule_id);
        }
        
        Ok(())
    }

    fn simulate_rule_removal(&self, rule: &FirewallRule) -> Result<()> {
        info!("📝 Would remove {} rule for: {}", 
              match rule.action {
                  RuleAction::Allow => "ALLOW",
                  RuleAction::Block => "BLOCK", 
                  RuleAction::Log => "LOG",
                  RuleAction::RateLimit(_) => "RATE_LIMIT",
              },
              self.format_rule_criteria(rule));
        Ok(())
    }

    /// Simulate traffic matching against rules
    pub fn process_traffic(&mut self, packet_info: &PacketInfo) -> Result<RuleAction> {
        // Find matching rules
        let matching_rules: Vec<&FirewallRule> = self.active_rules
            .values()
            .filter(|rule| self.rule_matches(rule, packet_info))
            .collect();

        if matching_rules.is_empty() {
            return Ok(RuleAction::Allow); // Default allow
        }

        // Use highest confidence rule
        let best_rule = matching_rules
            .iter()
            .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())
            .unwrap();

        // Update statistics
        let rule_id = best_rule.id.clone();
        if let Some(stats) = self.rule_stats.get_mut(&rule_id) {
            stats.matches += 1;
            stats.bytes_processed += packet_info.size as u64;
            stats.last_match = Some(chrono::Utc::now());
        }
        
        // Calculate effectiveness separately to avoid borrowing issues
        if let Some(stats) = self.rule_stats.get(&rule_id) {
            let effectiveness = self.calculate_effectiveness_score(stats);
            if let Some(stats_mut) = self.rule_stats.get_mut(&rule_id) {
                stats_mut.effectiveness_score = effectiveness;
            }
        }

        info!("🎯 Traffic matched rule: {} -> {:?}", best_rule.id, best_rule.action);
        Ok(best_rule.action.clone())
    }

    fn rule_matches(&self, rule: &FirewallRule, packet: &PacketInfo) -> bool {
        // Check source IP
        if let Some(rule_src) = &rule.source_ip {
            if rule_src != &packet.source_ip {
                return false;
            }
        }

        // Check destination IP
        if let Some(rule_dst) = &rule.dest_ip {
            if rule_dst != &packet.dest_ip {
                return false;
            }
        }

        // Check source port
        if let Some(rule_sport) = rule.source_port {
            if rule_sport != packet.source_port {
                return false;
            }
        }

        // Check destination port
        if let Some(rule_dport) = rule.dest_port {
            if rule_dport != packet.dest_port {
                return false;
            }
        }

        // Check protocol
        if rule.protocol.to_lowercase() != packet.protocol.to_lowercase() {
            return false;
        }

        true
    }

    fn calculate_effectiveness_score(&self, stats: &RuleStats) -> f64 {
        // Simple effectiveness calculation based on matches and recency
        let base_score = (stats.matches as f64).log10().max(0.0);
        let recency_bonus = if let Some(last_match) = stats.last_match {
            let hours_since = chrono::Utc::now()
                .signed_duration_since(last_match)
                .num_hours() as f64;
            (24.0 - hours_since.min(24.0)) / 24.0
        } else {
            0.0
        };
        
        (base_score + recency_bonus).min(1.0)
    }

    /// Get all active rules
    pub fn get_active_rules(&self) -> &HashMap<String, FirewallRule> {
        &self.active_rules
    }

    /// Get rule statistics
    pub fn get_rule_stats(&self) -> &HashMap<String, RuleStats> {
        &self.rule_stats
    }

    /// Clear all rules - SIMULATION
    pub fn clear_all_rules(&mut self) -> Result<()> {
        warn!("🧹 Clearing all firewall rules (simulation)");
        
        for rule_id in self.active_rules.keys() {
            info!("🗑️ Removing rule: {}", rule_id);
        }
        
        self.active_rules.clear();
        self.rule_stats.clear();
        
        info!("✅ All firewall rules cleared (simulation)");
        Ok(())
    }

    pub fn get_engine_status(&self) -> serde_json::Value {
        serde_json::json!({
            "simulation_mode": self.simulation_mode,
            "active_rules_count": self.active_rules.len(),
            "total_matches": self.rule_stats.values().map(|s| s.matches).sum::<u64>(),
            "total_bytes_processed": self.rule_stats.values().map(|s| s.bytes_processed).sum::<u64>(),
            "average_effectiveness": self.rule_stats.values()
                .map(|s| s.effectiveness_score)
                .sum::<f64>() / self.rule_stats.len().max(1) as f64,
            "safety_notice": "⚠️ All firewall rule applications disabled for research safety"
        })
    }
}

#[derive(Debug, Clone)]
pub struct PacketInfo {
    pub source_ip: String,
    pub dest_ip: String,
    pub source_port: u16,
    pub dest_port: u16,
    pub protocol: String,
    pub size: usize,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RuleSource};

    fn create_test_rule() -> FirewallRule {
        FirewallRule {
            id: "test-rule-1".to_string(),
            source_ip: Some("192.168.1.100".to_string()),
            dest_ip: None,
            source_port: None,
            dest_port: Some(80),
            protocol: "TCP".to_string(),
            action: RuleAction::Block,
            confidence: 0.9,
            created_by: RuleSource::Manual,
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

    #[test]
    fn test_rule_engine_creation() {
        let engine = RuleEngine::new();
        assert!(engine.simulation_mode);
        assert_eq!(engine.active_rules.len(), 0);
    }

    #[test]
    fn test_rule_application() {
        let mut engine = RuleEngine::new();
        let rule = create_test_rule();
        
        engine.apply_rule(rule.clone()).unwrap();
        
        assert_eq!(engine.active_rules.len(), 1);
        assert!(engine.active_rules.contains_key(&rule.id));
        assert!(engine.rule_stats.contains_key(&rule.id));
    }

    #[test]
    fn test_traffic_processing() {
        let mut engine = RuleEngine::new();
        let rule = create_test_rule();
        let packet = create_test_packet();
        
        engine.apply_rule(rule).unwrap();
        
        let action = engine.process_traffic(&packet).unwrap();
        assert!(matches!(action, RuleAction::Block));
        
        // Check stats were updated
        let stats = engine.rule_stats.get("test-rule-1").unwrap();
        assert_eq!(stats.matches, 1);
        assert_eq!(stats.bytes_processed, 1024);
    }

    #[test]
    fn test_rule_removal() {
        let mut engine = RuleEngine::new();
        let rule = create_test_rule();
        
        engine.apply_rule(rule.clone()).unwrap();
        assert_eq!(engine.active_rules.len(), 1);
        
        engine.remove_rule(&rule.id).unwrap();
        assert_eq!(engine.active_rules.len(), 0);
        assert!(!engine.rule_stats.contains_key(&rule.id));
    }
}