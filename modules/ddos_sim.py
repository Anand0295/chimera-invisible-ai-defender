"""DDoS Simulator Module - Traffic Testing"""
import socket
import threading
import time
from datetime import datetime


class DDoSSimulator:
    """Simulates various network traffic patterns for testing"""

    def __init__(self, target_host="localhost", target_port=8080):
        self.target_host = target_host
        self.target_port = target_port
        self.threads = []
        self.running = False

    def simulate_traffic(self, num_requests=100, threads=10):
        """Simulate normal traffic load"""
        print(f"[DDoSSimulator] Simulating {num_requests} requests with {threads} threads")
        print(f"[DDoSSimulator] Target: {self.target_host}:{self.target_port}")
        
        self.running = True
        requests_per_thread = num_requests // threads
        
        for i in range(threads):
            thread = threading.Thread(
                target=self._send_requests,
                args=(requests_per_thread, i)
            )
            thread.start()
            self.threads.append(thread)
        
        for thread in self.threads:
            thread.join()
        
        self.running = False
        print(f"[DDoSSimulator] Simulation complete")

    def _send_requests(self, count, thread_id):
        """Send HTTP-like requests (simulated)"""
        for i in range(count):
            if not self.running:
                break
            
            timestamp = datetime.now().strftime("%H:%M:%S")
            print(f"[Thread-{thread_id}] Request #{i+1} at {timestamp}")
            time.sleep(0.1)  # Simulate network delay

    def simulate_flood(self, duration=10):
        """Simulate flood attack (for testing defenses)"""
        print(f"[DDoSSimulator] Simulating flood attack for {duration}s")
        print("[WARNING] This is for testing defensive capabilities only")
        
        start_time = time.time()
        request_count = 0
        
        while time.time() - start_time < duration:
            request_count += 1
            if request_count % 100 == 0:
                print(f"[DDoSSimulator] Sent {request_count} flood requests")
            time.sleep(0.01)
        
        print(f"[DDoSSimulator] Flood simulation complete: {request_count} requests")
        return request_count

    def stop(self):
        """Stop all simulations"""
        print("[DDoSSimulator] Stopping all traffic")
        self.running = False


if __name__ == "__main__":
    simulator = DDoSSimulator()
    simulator.simulate_traffic(num_requests=50, threads=5)
