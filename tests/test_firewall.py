"""Test suite for the AI Firewall module."""

import unittest
from unittest.mock import Mock, patch
import sys
import os

# Add parent directory to path for imports
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))

from modules.firewall import AIFirewall


class TestAIFirewall(unittest.TestCase):
    """Unit tests for AIFirewall class."""

    def setUp(self):
        """Set up test fixtures."""
        self.firewall = AIFirewall(mode='monitor')

    def test_firewall_initialization(self):
        """Test firewall initializes correctly."""
        self.assertIsNotNone(self.firewall)
        self.assertEqual(self.firewall.mode, 'monitor')
        self.assertEqual(len(self.firewall.rules), 0)

    def test_add_rule(self):
        """Test adding firewall rules."""
        rule = {'action': 'block', 'pattern': 'malicious.*'}
        self.firewall.add_rule(rule)
        self.assertEqual(len(self.firewall.rules), 1)
        self.assertIn(rule, self.firewall.rules)

    def test_process_packet_monitor_mode(self):
        """Test packet processing in monitor mode."""
        packet = {'src': '192.168.1.1', 'dst': '10.0.0.1', 'payload': 'test'}
        result = self.firewall.process_packet(packet)
        self.assertTrue(result)  # Monitor mode allows all

    def test_process_packet_block_mode(self):
        """Test packet processing in block mode."""
        self.firewall.mode = 'block'
        rule = {'action': 'block', 'pattern': '192\\.168\\.1\\..*'}
        self.firewall.add_rule(rule)
        
        packet = {'src': '192.168.1.100', 'dst': '10.0.0.1', 'payload': 'test'}
        result = self.firewall.process_packet(packet)
        self.assertFalse(result)  # Should be blocked

    def test_threat_classification(self):
        """Test AI threat classification."""
        benign_packet = {'src': '10.0.0.1', 'payload': 'normal traffic'}
        malicious_packet = {'src': '1.2.3.4', 'payload': 'DROP TABLE users;'}
        
        benign_score = self.firewall.classify_threat(benign_packet)
        malicious_score = self.firewall.classify_threat(malicious_packet)
        
        self.assertLess(benign_score, malicious_score)

    def test_statistics_tracking(self):
        """Test that firewall tracks statistics."""
        packet = {'src': '192.168.1.1', 'dst': '10.0.0.1', 'payload': 'test'}
        self.firewall.process_packet(packet)
        
        stats = self.firewall.get_stats()
        self.assertGreater(stats['packets_processed'], 0)

    def test_rule_validation(self):
        """Test that invalid rules are rejected."""
        invalid_rule = {'invalid_key': 'value'}
        with self.assertRaises(ValueError):
            self.firewall.add_rule(invalid_rule)

    def tearDown(self):
        """Clean up after tests."""
        if hasattr(self.firewall, 'stop'):
            self.firewall.stop()


if __name__ == '__main__':
    unittest.main()
