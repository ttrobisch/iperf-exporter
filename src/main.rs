use std::process::Command;
use std::sync;

use actix_web::{App, get, HttpServer, web};
use serde::Deserialize;
use serde_json;

mod iperf_result;

#[derive(Deserialize, Debug)]
struct ProbeOptions {
    target: String,
    #[serde(default = "default_bitrate")]
    bitrate: String,
    #[serde(default = "default_duration")]
    duration: String,
}

fn default_bitrate() -> String {
    "0".to_string()
}

fn default_duration() -> String {
    "5".to_string()
}

#[get("/probe")]
async fn probe(params: web::Query<ProbeOptions>, lock: web::Data<sync::Arc<sync::Mutex<()>>>) -> actix_web::HttpResponse {
    let _guard = lock.lock();
    let metrics = execute_iperf3(&params);
    match metrics {
        None => {
            println!("No metrics");
            actix_web::HttpResponse::NotFound().body("No metrics")
        }
        Some(metrics) => {
            println!("Metrics: {:?}", metrics);
            actix_web::HttpResponse::Ok().body(metrics.gather_metrics())
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");
    let lock = sync::Arc::new(sync::Mutex::new(()));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(lock.clone()))
            .service(probe)
    })
        .bind("0.0.0.0:3030")?
        .run()
        .await
}

fn execute_iperf3(server: &ProbeOptions) -> Option<iperf_result::IperfResult> {
    let output = Command::new("iperf3")
        .arg("-J")
        .arg("-u")
        .arg("-b")
        .arg(server.bitrate.as_str())
        .arg("-t")
        .arg(server.duration.as_str())
        .arg("-c")
        .arg(server.target.as_str())
        .output()
        .expect("Failed to execute iperf3");

    let json_string = String::from_utf8(output.stdout).expect("Invalid JSON");

    serde_json::from_str(json_string.as_str()).ok()
}

#[cfg(test)]
mod tests {
    use crate::{execute_iperf3, ProbeOptions};

    #[test]
    fn test_probe() {
        let options = ProbeOptions{ duration: "5".to_string(), bitrate: "0".to_string(), target: "172.16.0.16".to_string() };
        let iperf_result = execute_iperf3(&options).unwrap();
    }
}
