extern crate mdns;

use mdns::{Record, RecordKind};
use std::net::IpAddr;

const SERVICE_NAME: &'static str = "_presciense._poltergeist._tcp.local";

fn main() {
    for response in mdns::discover::all(SERVICE_NAME).unwrap() {
        let response = response.unwrap();

        let addr = response.records()
                           .filter_map(self::to_ip_addr)
                           .next();

        if let Some(addr) = addr {
            println!("found poltergeist device at {} from {:?}", addr, response);
        } else {
            println!("poltergeist device does not advertise address");
        }

        ::std::thread::sleep(::std::time::Duration::from_secs(10));
    }
}

fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    }
}