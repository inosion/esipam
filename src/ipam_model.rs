use serde::{Deserialize, Serialize};
use std::collections::{HashSet};

use std::net::{Ipv4Addr, Ipv6Addr, IpAddr};
use uuid::Uuid;
use std::mem;

use ipnetwork::{IpNetwork, Ipv4Network, Ipv6Network};

use crate::IpamError;

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Label {
    key: String,
    value: String,
}

pub enum IPProtocolFamily {
    V4,
    V6
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CidrEntry {
    pub cidr: IpNetwork,
    pub id: String,
    pub uuid: Uuid,
    pub sysref: Option<String>,
    pub parent: Option<String>,
    pub attributes: HashSet<Label>, // would like to support a nested set of attributes here ideally
}

impl Default for CidrEntry {
    fn default() -> Self { 
        CidrEntry { 
            cidr: "0.0.0.0/0".parse().unwrap(),
            ..Default::default()
        }
    }
}

/// Configuration settings of a given IPAM
#[derive(Serialize, Deserialize)]
pub struct IpamConfig { 
    /// When a host CIDR is added, 10.99.99.68/24, setting this field to true 
    /// will also add 10.99.99.0/24 if it is missing
    pub add_missing_supernet: bool
}

impl Default for IpamConfig { 
    fn default() -> Self { 
        IpamConfig { 
            add_missing_supernet: false
        }
    }
}


// #[derive(Hash, Eq, PartialEq, Serialize, Deserialize, Debug)]
// pub enum AttributeEntry {
//     Attr(Label),
//     SetOfAttr(HashSet<AttributeEntry>),
// }


impl From<IpNetwork> for CidrEntry

    fn from_ipnetwork(cidr: IpNetwork) -> CidrEntry {
        let theuuid = Uuid::new_v4();

        CidrEntry {
            cidr,
            id: format!("{}_{}", theuuid, cidr),
            uuid: theuuid,
            sysref: None,
            parent: None,
            attributes: HashSet::default(),
        }
    }
}

impl TryFrom<&str> for CidrEntry {
    type Err = IpamError;
    fn try_from(s: &str) -> Result<Self, Self::Err> {
        let cidr: IpNetwork = s.parse()?;
        match cidr { 
            IpNetwork::V4(v4) =>  Ok(CidrEntry::from_ipnetwork(cidr)),
            IpNetwork::V6(v6) =>  Ok(CidrEntry::from_ipnetwork(cidr)),
        }
    
    }
}

impl From<IpAddr> for CidrEntry {
    fn from(addr: IpAddr) -> CidrEntry {
        match addr {
            IpAddr::V4(a) => CidrEntry::from(IpNetwork::V4(Ipv4Network::from(a))),
            IpAddr::V4(a) => CidrEntry::from(IpNetwork::V6(Ipv4Network::from(a))),
        }
    }        
}

/// The Ipam is the main object that stores a CIDR entry and attreibutes associted to it,
///  that:
/// - are for one or more Routing Domain/ASNs (where IP conflicts are intended to not occur)
/// 
///   The entries inside am IPAM can be from one or more `Routing Domain`'s - [RFC-4632](https://tools.ietf.org/html/rfc4632#section-5.4) or Autonomous Systems;
///   so long as the entries are intended to never conflict. IPAM will only ever hold one CIDR entry, never duplicates.
///   
/// - a given IP Protocol V4. 
/// 
/// An IPAM is made up of a set of CidrEntries<Ipv4> entries.
/// 

#[derive(Serialize, Deserialize, Default)]
pub struct Ipam {
    pub id: String,
    pub protocol: IPProtocolFamily,
    pub cidrs: Vec<CidrEntry>,
    pub cfg: IpamConfig,
}

impl Ipam {

    fn add_entry(&mut self, entry: CidrEntry) -> Result<IpamError> {

        // ensure the entry being added is matching the configured Ipam Protocol
        match (self.protocol, entry.cidr) {
            (IPProtocolFamily::V4, IpNetwork::V6(_) => Err(IpamError::InvalidProtocol())
            (IPProtocolFamily::V6, IpNetwork::V4(_) => Err(IpamError::InvalidProtocol())
            _ => {

                let mut c = entry.clone();
                let cidrs = self.cidrs.clone();
                for (i, e) in cidrs.iter().enumerate() {
                    if e.cidr.is_supernet_of(entry.cidr) {
                        c.parent = Some(e.id.clone());
                    }
                    if e.cidr.is_subnet_of(entry.cidr) {
                        let mut x = e.clone();
                        x.parent = Some(c.id.clone());
                        self.replace(i, x);
                    }
                }
                self.cidrs.push(c);
                Ok(entry.id)

            }
        }
    }

    fn replace(&mut self, idx: usize, new_entry: T) -> T {
        mem::replace(&mut self.cidrs[idx], new_entry)
    }

    fn size(&self) -> usize {
        self.cidrs.len()
    }

    fn contains(&self, search: Ipv4Network) -> bool {
        for i in self.cidrs.iter() {
           if i.cidr == search {
               return true;
           }
        }
        false
    }

    fn missing_supernets(&self) -> Vec<IpNetwork> {
        let mut results = vec![];
        for e in self.cidrs.iter() {
            let p = match self.protocol {
                IPProtocolFamily::V4 => Ipv4Network::new(e.cidr.nth(0).unwrap(), e.cidr.prefix()).unwrap();
                IPProtocolFamily::V6 => Ipv6Network::new(e.cidr.nth(0).unwrap(), e.cidr.prefix()).unwrap();
            }
            
            if !self.contains(p) {
                results.push(p);
            }
        }
        results

    }
}


impl Ipam {
    fn new(id: String, protocol: IPProtocolFamily) -> Self {
        IpamV4 { 
            id,
            cidrs: vec![],
            protocol,
            cfg: IpamConfig::default(),
        }
    }
}

// impl Aggregate for IpamV4 {
//     fn aggregate_type() -> &'static str {
//         "IpamV4"
//     }
// }

/* -----------------------------------------
 *  Tests
 * ----------------------------------------- 
 */
#[cfg(test)]
mod tests { 
    macro_rules! s {
        ($s:expr) => {
            String::from($s)
        };
    }

    use super::*;
    use rand::Rng;

    fn get_net4_address() -> ipnetwork::Ipv4Network {
        let mut rng = rand::thread_rng();
        let x0 = rng.gen_range(1..255);
        let x1 = rng.gen_range(1..255);
        let x2 = rng.gen_range(1..255);
        let x3 = rng.gen_range(1..255);
        let msk = rng.gen_range(4..32);

        ipnetwork::Ipv4Network::new(Ipv4Addr::from([x0, x1, x2, x3]), msk).unwrap()
    }

    #[test]
    fn test_basic_cidr_entry() {
        let x = CidrV4Entry::new(s!("10.2.2.1/21"));
        assert_eq!(x.id,format!("{}_{}",x.uuid,"10.2.2.1/21"));
    }
    #[test]
    #[should_panic]
    fn test_fail_for_invalid_cidr_entry() {
        let _x = CidrV4Entry::new(s!("INVALID_CidrV4Entry_10.2.2.sz1/21"));
    }

    #[test]
    #[should_panic]
    fn test_fail_for_invalid_cidr_entry2() {
        let _x = CidrV4Entry::new(s!("299.2.2.0/21"));
    }

    #[test]
    #[should_panic]
    fn test_fail_for_invalid_cidr_entry3() {
        let _x = CidrV4Entry::new(s!("99.2.2.0/33"));
    }

    #[test]
    fn test_missing_supernets() {
        let mut ipam = IpamV4::new(s!("My IPAM"));
        let cidr_entry = CidrV4Entry::new(s!("192.168.5.3/24"));
        ipam.add_entry(cidr_entry);
        let expected_result = vec![ ipnetwork::Ipv4Network::new(Ipv4Addr::from([192, 168, 5, 0]), 24).unwrap() ];
        assert_eq!(ipam.missing_supernets(), expected_result )

    }

    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_ip_addresses() { 

        let mut ipam = IpamV4::new(s!("My IPAM"));

        for _i in 0 .. 100 {
            let net4 = get_net4_address();
            let cidr_entry = CidrV4Entry::new_from_ipnet(net4);
            ipam.add_entry(cidr_entry);
        }
        assert_eq!(ipam.cidrs.len(), 100);

        let json = serde_json::to_string_pretty(&ipam).expect("Should have worked");
        let mut f = File::create("assets/sample_ipam.json").expect("can't write the new file");
        f.write_all(json.as_bytes()).expect("Was meant to error");

        let missing = serde_json::to_string_pretty(&ipam.missing_supernets()).expect("Failure bro!");
        let mut f = File::create("assets/missing_supernets.json").expect("can't write the new file");
        f.write_all(missing.as_bytes()).expect("Was meant to error")


    }
}