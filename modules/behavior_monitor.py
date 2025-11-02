"""Behavior Monitoring Module for Chimera AI Defender."""
import logging
import time
from typing import Dict, List, Optional
from collections import defaultdict

logger = logging.getLogger(__name__)

class BehaviorMonitor:
    """Monitor and analyze system/network behavior patterns."""
    
    def __init__(self, config: Optional[Dict] = None):
        self.config = config or {}
        self.behavior_log: List[Dict] = []
        self.anomaly_scores: Dict[str, float] = defaultdict(float)
        self.threshold = self.config.get('anomaly_threshold', 0.7)
        logger.info("Behavior Monitor initialized")
    
    def log_event(self, event: Dict) -> None:
        """Log a behavior event."""
        event['timestamp'] = time.time()
        self.behavior_log.append(event)
        logger.debug(f"Event logged: {event.get('type', 'unknown')}")
    
    def analyze_behavior(self, entity_id: str) -> float:
        """Analyze behavior patterns and return anomaly score."""
        # Simple anomaly detection based on event frequency
        recent_events = [e for e in self.behavior_log 
                        if e.get('entity_id') == entity_id]
        
        if len(recent_events) > 10:
            score = min(len(recent_events) / 50, 1.0)
            self.anomaly_scores[entity_id] = score
            
            if score > self.threshold:
                logger.warning(f"Anomaly detected for {entity_id}: {score:.2f}")
            return score
        return 0.0
    
    def get_anomalies(self) -> List[Dict]:
        """Get list of detected anomalies."""
        return [
            {'entity_id': eid, 'score': score}
            for eid, score in self.anomaly_scores.items()
            if score > self.threshold
        ]
    
    def clear_logs(self, older_than: Optional[float] = None) -> int:
        """Clear old behavior logs."""
        if older_than:
            cutoff = time.time() - older_than
            original_len = len(self.behavior_log)
            self.behavior_log = [e for e in self.behavior_log 
                               if e.get('timestamp', 0) > cutoff]
            cleared = original_len - len(self.behavior_log)
            logger.info(f"Cleared {cleared} old log entries")
            return cleared
        return 0

if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    monitor = BehaviorMonitor()
    for i in range(15):
        monitor.log_event({'type': 'connection', 'entity_id': 'test_host'})
    score = monitor.analyze_behavior('test_host')
    print(f"Anomaly score: {score}")
