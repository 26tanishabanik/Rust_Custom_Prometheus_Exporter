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
use sysinfo::{System, SystemExt, DiskExt};



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

    let disk1_usage_metric = register_gauge!("disk1_usage","show disk 1 usage").expect("unable to create disk 1 usage metric");
    let disk2_usage_metric = register_gauge!("disk2_usage","show disk 2 usage").expect("unable to create disk 2 usage metric");
    let memory_usage_metric = register_gauge!("memory_usage","show disk usage").expect("unable to create memory usage metric");



    loop {
            {
                let _guard = exporter.wait_duration(duration);
                let sys = System::new_all();
               

                info!("Updating metrics");

                let (document_folder_metrics, document_files_count) = calculate_folder_size_and_count("/home/tanisha/Documents");
                let (download_folder_metrics, download_files_count) = calculate_folder_size_and_count("/home/tanisha/Downloads");
                let total_memory = (sys.total_memory() / 1_000) as f64;
                let used_memory = (sys.used_memory() / 1_000) as f64;

                let percentage_memory_used: f64 = (used_memory/total_memory) as f64 * 100.00;
                
                let total: f64 = DiskExt::total_space(&sys.disks()[0]) as f64;
                let usage: f64 = DiskExt::available_space(&sys.disks()[0]) as f64;
                disk1_usage_metric.set((usage / total) as f64 * 100.0);
                if (usage / total) as f64 * 100.0 > 80.0 {
                    Notification::new()
                    .summary(DiskExt::name(&sys.disks()[0]).to_str().unwrap())
                    .sound_name(SOUND)
                    .body("Disk Space greater than 80%")
                    .icon("firefox")
                    .show().unwrap();
                }
                let total: f64 = DiskExt::total_space(&sys.disks()[1]) as f64;
                let usage: f64 = DiskExt::available_space(&sys.disks()[1]) as f64;
                disk2_usage_metric.set((usage / total) as f64 * 100.0);
                if (usage / total) as f64 * 100.0 > 80.0 {
                    Notification::new()
                    .summary(DiskExt::name(&sys.disks()[1]).to_str().unwrap())
                    .sound_name(SOUND)
                    .body("Disk Space greater than 80%")
                    .icon("firefox")
                    .show().unwrap();
                }
                



                if document_files_count > 8.0 {
                    Notification::new()
                    .summary("Document File count")
                    .sound_name(SOUND)
                    .body("File Count greater than 10")
                    .icon("firefox")
                    .show().unwrap();
                    
                    
                }
                if  percentage_memory_used >  80.0{
                    Notification::new()
                    .summary("Memory Usage")
                    .sound_name(SOUND)
                    .body("Memory used is over 80%")
                    .icon("firefox")
                    .show().unwrap();
                    
                    
                }
                document_folder_size.set(document_folder_metrics);
                document_folder_file_count.set(document_files_count);
                
                
                
                memory_usage_metric.set(percentage_memory_used);
                
                info!("{:.32}",percentage_memory_used);
                
                
                download_folder_size.set(download_folder_metrics);
                download_folder_file_count.set(download_files_count);
        }
    }
}