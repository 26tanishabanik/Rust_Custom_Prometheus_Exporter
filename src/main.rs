extern crate sysinfo;
use notify_rust::Notification;
use env_logger::{
    Builder,
    Env,
};
use log::info;
extern crate fs_extra;
use fs_extra::dir::get_size;
use prometheus_exporter::prometheus::register_gauge;
use std::net::SocketAddr;
use std::fs::read_dir;
use sysinfo::{System, SystemExt};


#[cfg(all(unix, not(target_os = "macos")))]
static SOUND: &str = "message-new-instant";

fn calculate_folder_size_and_count(path: &str) -> (f64, f64) {
    let paths = read_dir(path).unwrap();
    
    let folder_size = get_size(path).unwrap();
    return (folder_size as f64, paths.count() as f64)
}

fn main() {

    Builder::from_env(Env::default().default_filter_or("info")).init();


    let addr_raw = "0.0.0.0:9050";
    let addr: SocketAddr = addr_raw.parse().expect("unable to parse listen addr");


    let exporter = prometheus_exporter::start(addr).expect("unable to start the exporter");
    let duration = std::time::Duration::from_millis(10000);

    
    let document_folder_size = register_gauge!("document_folder_size", "calculates document folder size")
        .expect("can not create gauge random_value_metric");
    let document_folder_file_count = register_gauge!("document_file_count", "calculates document file count")
        .expect("can not create gauge random_value_metric");
    let download_folder_size = register_gauge!("download_folder_size", "calculates download folder size")
        .expect("can not create gauge random_value_metric");
    let download_folder_file_count = register_gauge!("download_file_count", "calculates download file count")
        .expect("can not create gauge random_value_metric");

    let disk_usage_metric = register_gauge!("disk_usage","show disk usage").expect("unable to create disk usage metric");



    loop {
            {
                let _guard = exporter.wait_duration(duration);
                let sys = System::new_all();

                info!("Updating metrics");

                let (document_folder_metrics, document_files_count) = calculate_folder_size_and_count("/home/tanisha/Documents");
                let (download_folder_metrics, download_files_count) = calculate_folder_size_and_count("/home/tanisha/Downloads");

                
                
                if document_files_count > 8.0 {
                    Notification::new()
                    .summary("Document File count")
                    .sound_name(SOUND)
                    .body("File Count greater than 10")
                    .icon("firefox")
                    .timeout(20)
                    .show().unwrap();
                    
                    
                }
                document_folder_size.set(document_folder_metrics);
                document_folder_file_count.set(document_files_count);
                disk_usage_metric.set((sys.used_memory() / 2_048) as f64);
                download_folder_size.set(download_folder_metrics);
                download_folder_file_count.set(download_files_count);
        }
    }
}