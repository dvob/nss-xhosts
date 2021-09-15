extern crate libc;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate libnss;

use std::error::Error;
use std::net::Ipv4Addr;

use libnss::host::{AddressFamily, Addresses, Host, HostHooks};
use libnss::interop::Response;

struct ExtendedHostFile {}

libnss_host_hooks!(xhosts, ExtendedHostFile);

const X_HOST_FILE: &str = "/etc/xhosts";

impl HostHooks for ExtendedHostFile {
    fn get_all_entries() -> Response<Vec<Host>> {
        Response::Success(vec![])
    }

    fn get_host_by_name(name: &str, af: AddressFamily) -> Response<Host> {
        if AddressFamily::IPv6 == af {
            return Response::NotFound
        }
        let raw_records = match std::fs::read_to_string(X_HOST_FILE) {
            Ok(content) => content,
            Err(_) => return Response::NotFound,
        };
        let records = match parse_records(raw_records.as_str()) {
            Ok(records) => records,
            Err(_) => return Response::NotFound
        };
        for rec in records {
            match get_response(name, rec) {
                Some(resp) => return resp,
                None => continue,
            }
        }
        Response::NotFound
    }

    fn get_host_by_addr(_: std::net::IpAddr) -> Response<Host> {
        Response::NotFound
    }
}

#[derive(Debug,PartialEq)]
enum Record<'a> {
    Exact(&'a str,Ipv4Addr),
    Suffix(&'a str,Ipv4Addr),
}

fn get_response(lookup: &str, rec: Record) -> Option<Response<Host>> {
    match rec {
        Record::Exact(name, ip) => {
            if name == lookup {
                Some(Response::Success(Host{
                    name: lookup.to_string(),
                    aliases: vec![],
                    addresses: Addresses::V4(vec![ip]),
                }))
            } else {
                None
            }
        }
        Record::Suffix(name, ip) => {
            if lookup.ends_with(name) {
                Some(Response::Success(Host{
                    name: lookup.to_string(),
                    aliases: vec![],
                    addresses: Addresses::V4(vec![ip]),
                }))

            } else {
                None
            }
        }
    }
}

fn parse_records(raw_records: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut records = vec![];
    for line in raw_records.lines() {
        if line.starts_with('#') {
            continue
        }
        let mut line = line.split_ascii_whitespace();
        let name = match line.next() {
            Some(name) => name,
            None => continue,
        };
        let ip = match line.next() {
            Some(ip) => ip,
            None => continue,
        };
        let ip: Ipv4Addr = match ip.parse() {
            Ok(ip) => ip,
            Err(_) => continue,
        };

        if let Some(stripped) = name.strip_prefix('*') {
            records.push(Record::Suffix(stripped, ip))
        } else {
            records.push(Record::Exact(name, ip))
        }
    }

    Ok(records)
}

#[test]
fn test_parse() -> Result<(), Box<dyn Error>> {
    let file = "*.foo.ch 192.168.1.1\nabc.bla.ch 8.8.4.1";

    let records = parse_records(file)?;

    assert_eq!(records[0], Record::Suffix(".foo.ch", Ipv4Addr::new(192,168,1,1)));
    assert_eq!(records[1], Record::Exact("abc.bla.ch", Ipv4Addr::new(8,8,4,1)));

    Ok(())
}
