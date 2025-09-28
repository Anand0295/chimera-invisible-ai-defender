"""
Rule Generator for AI Firewall

⚠️ SIMULATION ONLY - Real rule generation disabled for safety
"""

import logging
from typing import Dict, Any
from .rl_agent import FirewallAction
from .traffic_processor import PacketInfo

logger = logging.getLogger(__name__)

class RuleGenerator:
    """Generate firewall rules from AI decisions - SIMULATION ONLY"""
    
    def __init__(self):
        logger.warning("🚫 Rule generation DISABLED - simulation only")
        self.simulation_mode = True
        self.rules_generated = 0
    
    def action_to_rule(self, action: FirewallAction, packet: PacketInfo) -> Dict[str, Any]:
        """Convert AI action to firewall rule"""
        logger.info(f"📝 Converting action {action.action_type} to rule")
        
        rule = {
            "id": f"ai_rule_{self.rules_generated}",
            "action": action.action_type,
            "confidence": action.confidence,
            "source_ip": packet.source_ip,
            "dest_port": packet.dest_port,
            "protocol": packet.protocol,
            "parameters": action.parameters,
            "simulation_only": True
        }
        
        self.rules_generated += 1
        return rule