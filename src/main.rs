use std::process::Command;
use std::sync;

use actix_web::{App, get, HttpServer, web};
use serde::Deserialize;
use serde_json;

mod iperf_result;

#[derive(Deserialize)]
struct QueryInfo {
    target: String,
}

#[get("/probe")]
async fn probe(params: web::Query<QueryInfo>, lock: web::Data<sync::Arc<sync::Mutex<()>>>) -> actix_web::HttpResponse {
    let _guard = lock.lock();
    let metrics = execute_iperf3(params.target.clone().as_str());
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
    HttpServer::new(|| {
        let lock = sync::Arc::new(sync::Mutex::new(()));
        App::new()
            .app_data(web::Data::new(lock.clone()))
            .service(probe)
    })
        .bind("127.0.0.1:3030")?
        .run()
        .await
}

fn execute_iperf3<'a>(server: &'a str) -> Option<iperf_result::IperfResult> {
    let output = Command::new("iperf3")
        .arg("-J")
        .arg("-u")
        .arg("-b")
        .arg("0")
        .arg("-t")
        .arg("5")
        .arg("-c")
        .arg(server)
        .output()
        .expect("Failed to execute iperf3");

    let output = String::from_utf8(output.stdout).expect("Invalid JSON");

    serde_json::from_str(output.as_str()).ok()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_probe() {
    }
}
