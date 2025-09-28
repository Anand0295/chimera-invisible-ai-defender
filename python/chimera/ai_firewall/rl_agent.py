"""
Reinforcement Learning Agent for Firewall Rule Generation

⚠️ SIMULATION ONLY - All RL training and inference disabled for safety

This module implements a Deep Q-Network (DQN) agent for learning optimal
firewall rules based on network traffic patterns.
"""

import logging
import numpy as np
import torch
import torch.nn as nn
import torch.optim as optim
from typing import Dict, List, Tuple, Optional
import json
from dataclasses import dataclass
from collections import deque
import random

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

@dataclass
class TrafficState:
    """Represents the current network traffic state"""
    packet_rate: float
    byte_rate: float
    unique_sources: int
    unique_destinations: int
    port_diversity: float
    protocol_distribution: Dict[str, float]
    threat_indicators: Dict[str, float]

@dataclass
class FirewallAction:
    """Represents a firewall action decision"""
    action_type: str  # 'allow', 'block', 'rate_limit', 'log'
    confidence: float
    parameters: Dict[str, any]

class DQNNetwork(nn.Module):
    """Deep Q-Network for firewall decision making"""
    
    def __init__(self, state_size: int, action_size: int, hidden_size: int = 128):
        super(DQNNetwork, self).__init__()
        self.fc1 = nn.Linear(state_size, hidden_size)
        self.fc2 = nn.Linear(hidden_size, hidden_size)
        self.fc3 = nn.Linear(hidden_size, hidden_size)
        self.fc4 = nn.Linear(hidden_size, action_size)
        self.dropout = nn.Dropout(0.2)
        
    def forward(self, x):
        x = torch.relu(self.fc1(x))
        x = self.dropout(x)
        x = torch.relu(self.fc2(x))
        x = self.dropout(x)
        x = torch.relu(self.fc3(x))
        x = self.fc4(x)
        return x

class RLFirewallAgent:
    """Reinforcement Learning agent for firewall rule generation"""
    
    def __init__(self, state_size: int = 20, action_size: int = 4, learning_rate: float = 0.001):
        """Initialize the RL agent - SIMULATION ONLY"""
        logger.warning("🚫 RL Agent initialization DISABLED - simulation only")
        
        self.simulation_mode = True  # Always true for safety
        self.state_size = state_size
        self.action_size = action_size
        self.learning_rate = learning_rate
        
        # Initialize networks (but don't use them)
        self.q_network = DQNNetwork(state_size, action_size)
        self.target_network = DQNNetwork(state_size, action_size)
        self.optimizer = optim.Adam(self.q_network.parameters(), lr=learning_rate)
        
        # Experience replay buffer
        self.memory = deque(maxlen=10000)
        self.epsilon = 1.0  # Exploration rate
        self.epsilon_decay = 0.995
        self.epsilon_min = 0.01
        self.batch_size = 32
        
        # Statistics
        self.training_episodes = 0
        self.total_rewards = 0.0
        self.decisions_made = 0
        
        logger.info("📝 RL Firewall Agent created (simulation mode)")
    
    def preprocess_state(self, traffic_state: TrafficState) -> np.ndarray:
        """Convert traffic state to neural network input - SIMULATION"""
        logger.warning("🚫 State preprocessing DISABLED - simulation only")
        
        # Simulate state preprocessing
        state_vector = np.array([
            traffic_state.packet_rate / 1000.0,  # Normalize
            traffic_state.byte_rate / 1000000.0,  # Normalize
            traffic_state.unique_sources / 100.0,  # Normalize
            traffic_state.unique_destinations / 100.0,  # Normalize
            traffic_state.port_diversity,
            traffic_state.protocol_distribution.get('TCP', 0.0),
            traffic_state.protocol_distribution.get('UDP', 0.0),
            traffic_state.protocol_distribution.get('ICMP', 0.0),
            traffic_state.threat_indicators.get('port_scan', 0.0),
            traffic_state.threat_indicators.get('ddos', 0.0),
            traffic_state.threat_indicators.get('brute_force', 0.0),
            traffic_state.threat_indicators.get('anomaly', 0.0),
            # Pad with zeros to reach state_size
            *([0.0] * (self.state_size - 12))
        ])
        
        return state_vector[:self.state_size]
    
    def get_action(self, state: TrafficState) -> FirewallAction:
        """Get firewall action for current state - SIMULATION"""
        logger.warning("🚫 Action selection DISABLED - simulation only")
        
        self.decisions_made += 1
        
        # Simulate decision making
        state_vector = self.preprocess_state(state)
        
        # Simulate epsilon-greedy action selection
        if random.random() < 0.1:  # 10% random actions for simulation
            action_idx = random.randint(0, self.action_size - 1)
            confidence = 0.5
        else:
            # Simulate Q-value prediction
            action_idx = self._simulate_q_prediction(state_vector)
            confidence = 0.8
        
        # Map action index to firewall action
        actions = ['allow', 'block', 'rate_limit', 'log']
        action_type = actions[action_idx]
        
        # Generate parameters based on action type
        parameters = self._generate_action_parameters(action_type, state)
        
        action = FirewallAction(
            action_type=action_type,
            confidence=confidence,
            parameters=parameters
        )
        
        logger.info(f"🤖 Simulated action: {action_type} (confidence: {confidence:.2f})")
        return action
    
    def _simulate_q_prediction(self, state_vector: np.ndarray) -> int:
        """Simulate Q-value prediction"""
        # Simple heuristic-based simulation
        packet_rate = state_vector[0] * 1000
        ddos_score = state_vector[9]
        port_scan_score = state_vector[8]
        
        if ddos_score > 0.8 or packet_rate > 500:
            return 2  # rate_limit
        elif port_scan_score > 0.7:
            return 1  # block
        elif ddos_score > 0.3 or port_scan_score > 0.3:
            return 3  # log
        else:
            return 0  # allow
    
    def _generate_action_parameters(self, action_type: str, state: TrafficState) -> Dict[str, any]:
        """Generate parameters for the selected action"""
        parameters = {}
        
        if action_type == 'rate_limit':
            # Calculate rate limit based on current traffic
            base_rate = max(10, int(state.packet_rate * 0.1))
            parameters['packets_per_second'] = base_rate
            parameters['burst_size'] = base_rate * 2
        
        elif action_type == 'block':
            parameters['duration_seconds'] = 300  # 5 minutes
            parameters['scope'] = 'source_ip'  # Block by source IP
        
        elif action_type == 'log':
            parameters['log_level'] = 'INFO'
            parameters['include_payload'] = False
        
        # Common parameters
        parameters['rule_priority'] = 100
        parameters['expires_at'] = None  # Permanent rule
        
        return parameters
    
    def train(self, state: TrafficState, action: FirewallAction, reward: float, 
              next_state: TrafficState, done: bool) -> None:
        """Train the agent with experience - DISABLED"""
        logger.warning("🚫 RL training DISABLED - simulation only")
        
        # Simulate training statistics update
        self.total_rewards += reward
        
        if done:
            self.training_episodes += 1
            if self.epsilon > self.epsilon_min:
                self.epsilon *= self.epsilon_decay
        
        logger.info(f"📝 Would train with reward: {reward:.2f}")
    
    def save_model(self, filepath: str) -> None:
        """Save the trained model - DISABLED"""
        logger.warning("🚫 Model saving DISABLED - simulation only")
        
        # Simulate model saving
        model_info = {
            'simulation_mode': True,
            'training_episodes': self.training_episodes,
            'total_rewards': self.total_rewards,
            'decisions_made': self.decisions_made,
            'epsilon': self.epsilon,
            'safety_notice': '⚠️ This is a simulation model - not functional'
        }
        
        try:
            with open(filepath, 'w') as f:
                json.dump(model_info, f, indent=2)
            logger.info(f"📝 Simulated model save to: {filepath}")
        except Exception as e:
            logger.error(f"Failed to save simulation model info: {e}")
    
    def load_model(self, filepath: str) -> None:
        """Load a trained model - DISABLED"""
        logger.warning("🚫 Model loading DISABLED - simulation only")
        
        try:
            with open(filepath, 'r') as f:
                model_info = json.load(f)
            
            if model_info.get('simulation_mode'):
                self.training_episodes = model_info.get('training_episodes', 0)
                self.total_rewards = model_info.get('total_rewards', 0.0)
                self.decisions_made = model_info.get('decisions_made', 0)
                logger.info(f"📝 Loaded simulation model info from: {filepath}")
            else:
                logger.warning("⚠️ Attempted to load non-simulation model - ignored")
        
        except Exception as e:
            logger.error(f"Failed to load model info: {e}")
    
    def get_stats(self) -> Dict[str, any]:
        """Get agent statistics"""
        return {
            'simulation_mode': self.simulation_mode,
            'training_episodes': self.training_episodes,
            'total_rewards': self.total_rewards,
            'decisions_made': self.decisions_made,
            'epsilon': self.epsilon,
            'average_reward': self.total_rewards / max(1, self.training_episodes),
            'model_parameters': sum(p.numel() for p in self.q_network.parameters()),
            'safety_notice': '⚠️ All RL capabilities disabled for research safety'
        }
    
    def reset_stats(self) -> None:
        """Reset training statistics"""
        logger.info("🔄 Resetting agent statistics")
        self.training_episodes = 0
        self.total_rewards = 0.0
        self.decisions_made = 0
        self.epsilon = 1.0

# Example usage and testing
if __name__ == "__main__":
    logger.info("🔬 Testing RL Firewall Agent (simulation)")
    
    # Create agent
    agent = RLFirewallAgent()
    
    # Create sample traffic state
    sample_state = TrafficState(
        packet_rate=150.0,
        byte_rate=96000.0,
        unique_sources=25,
        unique_destinations=10,
        port_diversity=0.3,
        protocol_distribution={'TCP': 0.8, 'UDP': 0.2},
        threat_indicators={'port_scan': 0.6, 'ddos': 0.2, 'brute_force': 0.1, 'anomaly': 0.3}
    )
    
    # Get action
    action = agent.get_action(sample_state)
    logger.info(f"Action: {action.action_type}, Confidence: {action.confidence}")
    
    # Simulate training
    agent.train(sample_state, action, 0.5, sample_state, False)
    
    # Show stats
    stats = agent.get_stats()
    logger.info(f"Agent stats: {json.dumps(stats, indent=2)}")
    
    logger.info("✅ RL Firewall Agent simulation test completed")