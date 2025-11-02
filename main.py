#!/usr/bin/env python3
"""Chimera Invisible AI Defender - Main CLI"""
import argparse
import sys
import yaml
from pathlib import Path

# Import modules
from modules.firewall import AIFirewall
from modules.behavior_monitor import BehaviorMonitor
from modules.network_forensics import NetworkForensics
from modules.stealth_loader import StealthLoader
from modules.ddos_sim import DDoSSimulator


def load_config(config_path="config/config.yaml"):
    """Load configuration from YAML file"""
    try:
        with open(config_path, 'r') as f:
            return yaml.safe_load(f)
    except FileNotFoundError:
        print(f"[ERROR] Config file not found: {config_path}")
        sys.exit(1)


def run_firewall(config):
    """Run AI Firewall module"""
    fw_config = config.get('firewall', {})
    if not fw_config.get('enabled', False):
        print("[INFO] Firewall is disabled in config")
        return
    
    print("\n=== Starting AI Firewall ===")
    firewall = AIFirewall()
    firewall.start()
    print("[INFO] Firewall running...")


def run_behavior_monitor(config):
    """Run Behavior Monitor module"""
    bm_config = config.get('behavior_monitor', {})
    if not bm_config.get('enabled', False):
        print("[INFO] Behavior Monitor is disabled in config")
        return
    
    print("\n=== Starting Behavior Monitor ===")
    monitor = BehaviorMonitor(
        interval=bm_config.get('interval', 5),
        suspicious_processes=bm_config.get('suspicious_processes', [])
    )
    monitor.start_monitoring(duration=30)


def run_network_forensics(config):
    """Run Network Forensics module"""
    nf_config = config.get('network_forensics', {})
    if not nf_config.get('enabled', False):
        print("[INFO] Network Forensics is disabled in config")
        return
    
    print("\n=== Starting Network Forensics ===")
    forensics = NetworkForensics(log_file=nf_config.get('log_file', 'network.log'))
    forensics.start_capture(
        interface=nf_config.get('interface', 'eth0'),
        duration=nf_config.get('capture_duration', 60)
    )
    forensics.analyze_traffic()


def run_stealth_loader(config):
    """Run Stealth Loader module"""
    sl_config = config.get('stealth_loader', {})
    if not sl_config.get('enabled', False):
        print("[INFO] Stealth Loader is disabled in config")
        return
    
    print("\n=== Stealth Loader ===")
    loader = StealthLoader(service_name=sl_config.get('service_name', 'chimera-defender'))
    loader.install_persistence()


def run_ddos_sim(config):
    """Run DDoS Simulator module"""
    ddos_config = config.get('ddos_simulator', {})
    if not ddos_config.get('enabled', False):
        print("[INFO] DDoS Simulator is disabled in config")
        return
    
    print("\n=== DDoS Simulator (TEST MODE) ===")
    simulator = DDoSSimulator(
        target_host=ddos_config.get('target_host', 'localhost'),
        target_port=ddos_config.get('target_port', 8080)
    )
    simulator.simulate_traffic(num_requests=50, threads=5)


def main():
    """Main CLI entry point"""
    parser = argparse.ArgumentParser(
        description='Chimera Invisible AI Defender - OSS Security Suite',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s --all              Run all enabled modules
  %(prog)s --firewall         Run AI firewall only
  %(prog)s --monitor          Run behavior monitor only
  %(prog)s --forensics        Run network forensics only
  %(prog)s --config custom.yaml  Use custom config file
        """
    )
    
    parser.add_argument('--config', default='config/config.yaml', help='Config file path')
    parser.add_argument('--all', action='store_true', help='Run all enabled modules')
    parser.add_argument('--firewall', action='store_true', help='Run AI firewall')
    parser.add_argument('--monitor', action='store_true', help='Run behavior monitor')
    parser.add_argument('--forensics', action='store_true', help='Run network forensics')
    parser.add_argument('--stealth', action='store_true', help='Install stealth loader')
    parser.add_argument('--ddos-sim', action='store_true', help='Run DDoS simulator (testing)')
    parser.add_argument('--version', action='version', version='%(prog)s 0.1.0')
    
    args = parser.parse_args()
    
    # Load configuration
    config = load_config(args.config)
    
    print("╔═══════════════════════════════════════════════════╗")
    print("║   CHIMERA INVISIBLE AI DEFENDER v0.1.0 (MVP)     ║")
    print("║   Advanced AI-Powered Security Suite             ║")
    print("╚═══════════════════════════════════════════════════╝")
    
    # Determine what to run
    if args.all:
        run_firewall(config)
        run_behavior_monitor(config)
        run_network_forensics(config)
        run_stealth_loader(config)
        run_ddos_sim(config)
    else:
        if args.firewall:
            run_firewall(config)
        if args.monitor:
            run_behavior_monitor(config)
        if args.forensics:
            run_network_forensics(config)
        if args.stealth:
            run_stealth_loader(config)
        if args.ddos_sim:
            run_ddos_sim(config)
        
        # If no specific module selected, show help
        if not any([args.firewall, args.monitor, args.forensics, args.stealth, args.ddos_sim]):
            parser.print_help()
            sys.exit(0)
    
    print("\n[INFO] Chimera Defender execution complete.")


if __name__ == "__main__":
    main()
