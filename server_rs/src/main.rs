use std::{net::IpAddr, process::Command, time::Duration};

use actix_web::{get, web, App, HttpServer, Responder};
use nix::{
    ifaddrs,
    sys::{socket::SockaddrStorage, sysinfo},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ReturnedInfo {
    ip_addresses: Vec<InterfaceAddress>,
    uptime: Duration,
    processes: ProcessesInfo,
    disk_space: DisksInfo,
}

#[derive(Debug, Serialize)]
struct InterfaceAddress {
    name: String,
    ip: IpAddr,
}

impl InterfaceAddress {
    fn new(name: String, ip: IpAddr) -> Self {
        Self { name, ip }
    }
}

#[derive(Debug, Serialize)]
struct ProcessesInfo {
    header: String,
    processes: Vec<String>,
}

#[derive(Debug, Serialize)]
struct DisksInfo {
    header: String,
    disks: Vec<String>,
}

#[derive(Copy, Clone, Debug)]
struct MyIpAddr(SockaddrStorage);
impl TryFrom<MyIpAddr> for IpAddr {
    type Error = ();

    fn try_from(val: MyIpAddr) -> Result<Self, Self::Error> {
        if let Some(ipv4) = val.0.as_sockaddr_in() {
            Ok(IpAddr::V4(ipv4.ip()))
        } else if let Some(ipv6) = val.0.as_sockaddr_in6() {
            Ok(IpAddr::V6(ipv6.ip()))
        } else {
            Err(())
        }
    }
}

fn get_uptime() -> Duration {
    let sysinfo = sysinfo::sysinfo().unwrap();
    sysinfo.uptime()
}

fn get_ip_addr_info() -> Vec<InterfaceAddress> {
    let addrs = ifaddrs::getifaddrs().unwrap();
    addrs
        .filter_map(|ifaddr| match ifaddr.address {
            Some(address) => match MyIpAddr(address).try_into() {
                Ok(ip) => Some(InterfaceAddress::new(ifaddr.interface_name, ip)),
                Err(_) => None,
            },
            None => {
                eprintln!(
                    "interface {} with unsupported address family",
                    ifaddr.interface_name
                );
                None
            }
        })
        .collect()
}

fn get_processes_info() -> ProcessesInfo {
    let ps_out = Command::new("/usr/bin/ps")
        .arg("ax")
        .arg("-o")
        .arg("pid,time,command")
        .output()
        .unwrap();
    let ps_out_str = String::from_utf8(ps_out.stdout).unwrap();
    let mut it = ps_out_str.lines().map(str::to_owned);
    let header = it.next().unwrap();
    let processes = it.collect();

    ProcessesInfo { header, processes }
}

fn get_disks_info() -> DisksInfo {
    let df_out = Command::new("/usr/bin/df").arg("-h").output().unwrap();
    let df_out_str = String::from_utf8(df_out.stdout).unwrap();
    let mut it = df_out_str.lines().map(str::to_owned);
    let header = it.next().unwrap();
    let disks = it.collect();

    DisksInfo { header, disks }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_info))
        .bind(("0.0.0.0", 8198))?
        .run()
        .await
}

#[get("/")]
async fn get_info() -> impl Responder {
    let ip_addr_info = get_ip_addr_info();
    let list_of_processes = get_processes_info();
    let disks_info = get_disks_info();
    let uptime = get_uptime();
    let results = ReturnedInfo {
        ip_addresses: ip_addr_info,
        uptime,
        processes: list_of_processes,
        disk_space: disks_info,
    };

    web::Json(results)
}
