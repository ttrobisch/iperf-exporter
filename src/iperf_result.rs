use prometheus::{Encoder, GaugeVec, opts, Registry, TextEncoder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IperfResult {
    end: End,
}

impl IperfResult {
    pub fn gather_metrics(&self) -> String {
        // Create a new registry
        let registry = Registry::new();

        // Define a gauge vector with a "unit" label for metrics
        let metrics = GaugeVec::new(
            opts!("iperf_metrics", "Metrics from iperf3"),
            &["direction", "field", "unit"],
        ).unwrap();

        // Register the gauge vector with the registry
        registry.register(Box::new(metrics.clone())).unwrap();

        // Set values for sum_sent
        metrics.with_label_values(&["sent", "bytes", "B"]).set(self.end.sum_sent.bytes as f64);
        metrics.with_label_values(&["sent", "bitrate", "bit/s"]).set(self.end.sum_sent.bits_per_second);
        metrics.with_label_values(&["sent", "jitter", "ms"]).set(self.end.sum_sent.jitter_ms);
        metrics.with_label_values(&["sent", "lost_packets", "packets"]).set(self.end.sum_sent.lost_packets as f64);
        metrics.with_label_values(&["sent", "packets", "packets"]).set(self.end.sum_sent.packets as f64);
        metrics.with_label_values(&["sent", "lost_percent", "%"]).set(self.end.sum_sent.lost_percent as f64);

        // Set values for sum_received
        metrics.with_label_values(&["received", "bytes", "B"]).set(self.end.sum_received.bytes as f64);
        metrics.with_label_values(&["received", "bitrate", "bit/s"]).set(self.end.sum_received.bits_per_second);
        metrics.with_label_values(&["received", "jitter", "ms"]).set(self.end.sum_received.jitter_ms);
        metrics.with_label_values(&["received", "lost_packets", "packets"]).set(self.end.sum_received.lost_packets as f64);
        metrics.with_label_values(&["received", "packets", "packets"]).set(self.end.sum_received.packets as f64);
        metrics.with_label_values(&["received", "lost_percent", "%"]).set(self.end.sum_received.lost_percent as f64);

        // Gather the metrics and return as a string
        let mut buffer = vec![];
        let encoder = TextEncoder::new();

        let metrics_families = registry.gather();
        encoder.encode(&metrics_families, &mut buffer).unwrap();

        String::from_utf8(buffer).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct End {
    sum_sent: SumSent,
    sum_received: SumReceived,
}

#[derive(Debug, Serialize, Deserialize)]
struct SumSent {
    start: f64,
    end: f64,
    seconds: f64,
    bytes: i64,
    bits_per_second: f64,
    jitter_ms: f64,
    lost_packets: i32,
    packets: i32,
    lost_percent: f64,
    sender: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct SumReceived {
    start: f64,
    end: f64,
    seconds: f64,
    bytes: i64,
    bits_per_second: f64,
    jitter_ms: f64,
    lost_packets: i32,
    packets: i32,
    lost_percent: f64,
    sender: bool,
}

impl From<&str> for IperfResult {
    fn from(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::iperf_result::IperfResult;

    #[test]
    fn test_result_to_prometheus() {
        let result = IperfResult::from(include_str!("../test_files/result.json"));
        let metrics_string = result.gather_metrics();

        assert_eq!(201954188.06585097, result.end.sum_received.bits_per_second);
        assert_eq!(0.057079530215429858, result.end.sum_received.jitter_ms);
        assert_eq!(0.0, result.end.sum_received.lost_percent);

        metrics_string.lines().for_each(|line| println!("{}", line));
    }
}