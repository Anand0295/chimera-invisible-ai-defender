"""
Traffic Processor for AI Firewall

⚠️ SIMULATION ONLY - Real traffic processing disabled for safety
"""

import logging
import random
from typing import Dict, List, Any
from dataclasses import dataclass
from .rl_agent import TrafficState

logger = logging.getLogger(__name__)

@dataclass
class PacketInfo:
    """Network packet information"""
    source_ip: str
    dest_ip: str
    source_port: int
    dest_port: int
    protocol: str
    size: int
    timestamp: float
    suspicious: bool = False

class TrafficProcessor:
    """Process network traffic for AI analysis - SIMULATION ONLY"""
    
    def __init__(self):
        logger.warning("🚫 Traffic processing DISABLED - simulation only")
        self.simulation_mode = True
        self.processed_count = 0
    
    def generate_synthetic_traffic(self, count: int) -> List[PacketInfo]:
        """Generate synthetic traffic for testing"""
        logger.info(f"🔬 Generating {count} synthetic packets")
        
        packets = []
        source_ips = ["192.168.1.100", "10.0.0.50", "172.16.0.200", "203.0.113.10"]
        dest_ips = ["8.8.8.8", "1.1.1.1", "208.67.222.222"]
        protocols = ["TCP", "UDP", "ICMP"]
        common_ports = [80, 443, 22, 21, 25, 53, 3389]
        
        for i in range(count):
            packet = PacketInfo(
                source_ip=random.choice(source_ips),
                dest_ip=random.choice(dest_ips),
                source_port=random.randint(1024, 65535),
                dest_port=random.choice(common_ports),
                protocol=random.choice(protocols),
                size=random.randint(64, 1500),
                timestamp=1234567890.0 + i,
                suspicious=random.random() < 0.1  # 10% suspicious traffic
            )
            packets.append(packet)
        
        self.processed_count += count
        return packets
    
    def packet_to_state(self, packet: PacketInfo) -> TrafficState:
        """Convert packet info to traffic state"""
        # Simulate state conversion
        return TrafficState(
            packet_rate=100.0,  # Simulated
            byte_rate=64000.0,  # Simulated
            unique_sources=10,
            unique_destinations=5,
            port_diversity=0.3,
            protocol_distribution={"TCP": 0.8, "UDP": 0.2},
            threat_indicators={
                "port_scan": 0.1 if packet.suspicious else 0.0,
                "ddos": 0.0,
                "brute_force": 0.0,
                "anomaly": 0.1 if packet.suspicious else 0.0
            }
        )