use cqrs_es::Aggregate;
use ipnetwork::{IpNetwork, Ipv4Network, Ipv6Network};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::convert::TryFrom;
use std::mem;
use std::net::{ IpAddr , Ipv4Addr, Ipv6Addr };
use uuid::Uuid;
use crate::error::IpamError;

/* --- Common and Simple Types -----------------------------------------*/

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Label {
    key: String,
    value: String,
}

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum IPProtocolFamily {
    V4,
    V6,
}

impl Default for IPProtocolFamily {
    fn default() -> Self {
        Self::V6
    }
}

type CidrId = Box<String>;


/* --- Ipam -----------------------------------------*/
/// An Ipam is the Single Aggregate that  holds
/// a set of CidrEntry's.
///
/// ```
/// ,---------.
/// |  Ipam   |
/// |---------| Holds 1 Ipam set of CIDRs, a routing domain or 
/// |         | a logical non-duplicate CIDR tree. pinned at v4 or v6
/// `----1----'
///      |     
/// ,----*----.
/// |CidrEntry| <-- ensures all Entries
/// |---------|     are V4 or V6
/// `----1----'
///      |               
///   ,--*--.  
///   |Label|  <-- many labels for a CidrEntry
///   |-----|  
///   `-----'  
/// ```
///
/// Each Ipam is it's own non-conflicting list of CIDRs and associated meta-data.
/// 
/// The Ipam is the main object that stores a CIDR entry and attributes associted to it,
///  that:
/// - are for one or more Routing Domain/ASNs (where IP conflicts are intended to not occur)
///
///   The entries inside am Ipam can be from one or more `Routing Domain`'s - [RFC-4632](https://tools.ietf.org/html/rfc4632#section-5.4) or Autonomous Systems;
///   so long as the entries are intended to never conflict. Ipam will only ever hold one CIDR entry, never duplicates.
///   
/// - a given IP Protocol V4.
///
/// An Ipam is made up of a set of CidrEntries<Ipv4> entries.
///

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Ipam {
    pub id: String,
    pub uuid: Uuid,
    pub protocol: IPProtocolFamily,
    pub cidrs: Vec<CidrEntry>,
    pub cfg: Option<IpamConfig>,
}

impl Default for Ipam {
    fn default() -> Self {
        Ipam {
            id: Default::default(),
            uuid: Uuid::new_v4(),
            protocol: Default::default(),
            cidrs: Default::default(),
            cfg: None
        }
    }
}

trait Finder<T> {
    fn find(&self, search: T) -> Option<CidrEntry>;
}

impl Finder<&CidrId> for Ipam {
    fn find(&self, search: &CidrId) -> Option<CidrEntry> {
        self.cidrs.iter().find(|&ce| ce.id == *search).map(|r| r.clone())
    }
}

impl Finder<&IpNetwork> for Ipam {
    fn find(&self, search: &IpNetwork) -> Option<CidrEntry> {
        self.cidrs.iter().find(|&ce| ce.cidr == *search).map(|r| r.clone())
    }
}

impl Finder<&Uuid> for Ipam {
    fn find(&self, search: &Uuid) -> Option<CidrEntry> {
        self.cidrs.iter().find(|&ce| ce.uuid == *search).map(|r| r.clone())
    }
}

impl Finder<&str> for Ipam {
    fn find(&self, search: &str) -> Option<CidrEntry> {
        self.cidrs.iter().find(|&ce| 
               match search {
                   search if ce.cidr.to_string().contains(search) => true,
                   search if ce.id.to_string().contains(search) => true,
                   search if ce.uuid.to_string().contains(search) => true,
                   search if ce.sysref.as_ref().unwrap().contains(search) => true,
                   _ => false

               })
            .map(|r| r.clone())
    }
}

impl Ipam {
    fn new(id: &str) -> Self {
        Ipam {
            id: String::from(id),
            ..Default::default()
        }
    }

    fn new_with_protcol(id: &str, protocol: IPProtocolFamily) -> Self {
        Ipam {
            id: String::from(id),
            protocol,
            ..Default::default()
        }
    }

    pub (crate) fn children_of(&self, entry: IpNetwork) -> Vec<IpNetwork> {
        vec![]
    }

    pub fn filter(&self, search: &str) -> Vec<&CidrEntry> {
        self.cidrs.iter().filter(|&ce| 
                match search {
                    search if ce.cidr.to_string().contains(search) => true,
                    search if ce.id.to_string().contains(search) => true,
                    search if ce.uuid.to_string().contains(search) => true,
                    search if ce.sysref.as_ref().unwrap().contains(search) => true,
                    _ => false

                }).collect()
    }
    

    /// Returns the ID, and the CIDR of the located entry
    /// need to optimise this (Tuple struct or something, to pin the type)
    pub (crate) fn parent_of(&self, entry: IpNetwork) -> Option<CidrEntryResult> {
        match entry { 
            IpNetwork::V6(v6) => {
                for (i, e) in self.cidrs.iter().enumerate() {
                    let candidate = match e.cidr {
                        IpNetwork::V6(x) => Ok(x),
                        _ => Err(IpamError::InvalidProtocol),
                    }
                    .expect("Dead code, can't get here");

                    if v6 != candidate && v6.is_subnet_of(candidate) {
                        return Some(CidrEntryResult{ id: e.id.clone(), cidr: e.cidr})
                    }
                }
            },
            IpNetwork::V4(v4) => {
                for (i, e) in self.cidrs.iter().enumerate() {
                    let candidate = match e.cidr {
                        IpNetwork::V4(x) => Ok(x),
                        _ => Err(IpamError::InvalidProtocol),
                    }
                    .expect("Dead code, can't get here");

                    if v4 != candidate && v4.is_subnet_of(candidate) {
                        return Some(CidrEntryResult{ id: e.id.clone(), cidr: e.cidr })
                    }
                }
            }

        }
        None
    }

    pub(crate) fn add_entry(&mut self, entry: CidrEntry) -> Result<CidrEntry, IpamError> {
        // ensure the entry being added is matching the configured Ipam Protocol
        match (self.protocol.clone(), entry.cidr) {
            (IPProtocolFamily::V4, IpNetwork::V6(_)) => Err(IpamError::InvalidProtocol),
            (IPProtocolFamily::V6, IpNetwork::V4(_)) => Err(IpamError::InvalidProtocol),
            (IPProtocolFamily::V4, IpNetwork::V4(v4)) => {
                let mut c = entry;
                let cidrs = self.cidrs.clone();

                for (i, e) in cidrs.iter().enumerate() {
                    let candidate = match e.cidr {
                        IpNetwork::V4(x) => Ok(x),
                        _ => Err(IpamError::InvalidProtocol),
                    }
                    .expect("Dead code, can't get here");

                    if v4.is_subnet_of(candidate) {
                        c.parent = Some(e.id.clone());
                    }
                    if v4.is_supernet_of(candidate) {
                        let mut x = e.clone();
                        x.parent = Some(c.id.clone());
                        self.replace(i, x);
                    }
                }
                self.cidrs.push(c.to_owned());
                Ok(c)
            }

            (IPProtocolFamily::V6, IpNetwork::V6(v6)) => {
                let mut c = entry;
                let cidrs = self.cidrs.clone();

                for (i, e) in cidrs.iter().enumerate() {
                    let candidate = match e.cidr {
                        IpNetwork::V6(x) => Ok(x),
                        _ => Err(IpamError::InvalidProtocol),
                    }
                    .expect("Dead code, can't get here");

                    if v6.is_subnet_of(candidate) {
                        c.parent = Some(e.id.clone());
                    }
                    if v6.is_supernet_of(candidate) {
                        let mut x = e.clone();
                        x.parent = Some(c.id.clone());
                        self.replace(i, x);
                    }
                }
                self.cidrs.push(c.to_owned());
                Ok(c)
            }
        }
    }

    pub(crate) fn replace(&mut self, idx: usize, new_entry: CidrEntry) -> CidrEntry {
        mem::replace(&mut self.cidrs[idx], new_entry)
    }

    pub(crate) fn size(&self) -> usize {
        self.cidrs.len()
    }

    pub(crate) fn contains(&self, search: IpNetwork) -> bool {
        self.cidrs.iter().any(|ce| ce.cidr == search)
    }


    pub(crate) fn missing_supernets(&self) -> Vec<IpNetwork> {
        let mut results = vec![];
        for e in self.cidrs.iter() {
            let p = IpNetwork::new(e.cidr.network(), e.cidr.prefix()).unwrap();
            if !self.contains(p) {
                results.push(p);
            }
        }
        results
    }
}
    
impl Aggregate for Ipam {
    fn aggregate_type() -> &'static str {
        "Ipam"
    }
}

impl From<&str> for Ipam {
    fn from(s: &str) -> Ipam {
        Ipam::new(s)
    }
}

/* --- CidrEntry -----------------------------------------*/

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct CidrEntry {
    pub cidr: IpNetwork,
    pub id: CidrId,
    pub uuid: Uuid,
    pub sysref: Option<String>,
    pub parent: Option<CidrId>,
    pub attributes: HashSet<Label>, // would like to support a nested set of attributes here ideally
}

impl Default for CidrEntry {
    fn default() -> Self {
        CidrEntry {
            cidr: "::0/0".parse().unwrap(),
            ..Default::default()
        }
    }
}

impl CidrEntry { 
 
    /// Create a Cidr Entry from str
    /// Optional extras and sets of attributes can be provided
    /// also
    pub fn try_from_with_extras(
        s: &str,  
        id: Option<String>,
        sysref: Option<String>,
        attributes: HashSet<Label>    
        ) -> Result<CidrEntry, IpamError> {

        let cidr: IpNetwork = s.parse()?;
        let mut cidr_entry = match cidr {
            IpNetwork::V4(_v4) => CidrEntry::from(cidr),
            IpNetwork::V6(_v6) => CidrEntry::from(cidr),
        };

        // id is set to a default value, change if supplied
        id.into_iter().for_each(|i| cidr_entry.id = Box::new(i));
        cidr_entry.sysref = sysref;
        cidr_entry.attributes = attributes;
        Ok(cidr_entry)
    }


}

// #[derive(Hash, Eq, PartialEq, Serialize, Deserialize, Debug)]
// pub enum AttributeEntry {
//     Attr(Label),
//     SetOfAttr(HashSet<AttributeEntry>),
// }

impl From<IpNetwork> for CidrEntry {
    fn from(cidr: IpNetwork) -> CidrEntry {
        let theuuid = Uuid::new_v4();

        CidrEntry {
            cidr,
            id: Box::new(format!("{}_{}", theuuid, cidr)),
            uuid: theuuid,
            sysref: None,
            parent: None,
            attributes: HashSet::default(),
        }
    }
}

impl TryFrom<&str> for CidrEntry {
    type Error = IpamError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let cidr: IpNetwork = s.parse()?;
        match cidr {
            IpNetwork::V4(_v4) => Ok(CidrEntry::from(cidr)),
            IpNetwork::V6(_v6) => Ok(CidrEntry::from(cidr)),
        }
    }

}

impl From<IpAddr> for CidrEntry {
    fn from(addr: IpAddr) -> CidrEntry {
        match addr {
            IpAddr::V4(a) => CidrEntry::from(IpNetwork::V4(Ipv4Network::from(a))),
            IpAddr::V6(a) => CidrEntry::from(IpNetwork::V6(Ipv6Network::from(a))),
        }
    }
}

/* --- Ipam and Related Data Model -----------------------------------------*/

/// Configuration settings of a given Ipam
#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct IpamConfig {
    /// When a host CIDR is added, 10.99.99.68/24, setting this field to true
    /// will also add 10.99.99.0/24 if it is missing
    pub add_missing_supernet: bool,
}

impl Default for IpamConfig {
    fn default() -> Self {
        IpamConfig {
            add_missing_supernet: false,
        }
    }
}


pub struct CidrEntryResult{
    pub cidr: IpNetwork,
    pub id: CidrId,
}


/* --- Tests -----------------------------------------*/
#[cfg(test)]
mod tests {

    use super::*;
    use crate::common;
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
        let x = CidrEntry::try_from("10.2.2.1/21").expect("failure abound");

        assert_eq!(x.id, Box::new(format!("{}_{}", x.uuid, "10.2.2.1/21")));
        assert_eq!(x.cidr.is_ipv4(), true);
    }

    #[test]
    #[should_panic]
    fn test_fail_for_invalid_cidr_entry() {
        let x = CidrEntry::try_from("1NVALID_CidrEntry_10.2.2.1/21").expect("failure abound");
    }

    #[test]
    #[should_panic]
    fn test_fail_for_invalid_cidr_v4_entry() {
        // IP 910. is invalid
        let x = CidrEntry::try_from("910.2.2.1/21").expect("failure abound");
    }

    #[test]
    fn test_valid_cidr_v6_entry() {
        let x = CidrEntry::try_from("fe80::cafe:babe/64").expect("failure abound");
        assert_eq!(x.cidr.is_ipv6(), true);
    }

    #[test]
    #[should_panic]
    fn test_fail_for_invalid_cidr_mask() {
        let _ = CidrEntry::try_from("99.2.2.0/33").expect("failure abound");
    }

    #[test]
    fn test_missing_supernets() {
        let mut ipam = Ipam::new_with_protcol("My Ipam", IPProtocolFamily::V4);
        let cidr_entry = CidrEntry::try_from("192.168.5.3/24").expect("failure");
        let _ = ipam.add_entry(cidr_entry);
        let expected_result = vec![IpNetwork::try_from("192.168.5.0/24").unwrap()];
        assert_eq!(ipam.missing_supernets()[0], expected_result[0])
    }


    #[test]
    fn test_find() {
        let mut ipam = Ipam::new_with_protcol("My Ipam", IPProtocolFamily::V4);

        let special_one = Box::new(String::from("Network_Object_12345"));
        for i in 100..110 {
            let net4 = get_net4_address();
            let mut cidr_entry = CidrEntry::from(IpNetwork::from(net4));
            cidr_entry.sysref = Some(format!("xtref::some_id_{}",i));
            if (i == 5) {
                cidr_entry.id = special_one.clone();
                cidr_entry.sysref = None;
            }
            let _ = ipam.add_entry(cidr_entry);

        }

        // check we can find one by some string
        assert!(ipam.find("some_id_107").is_some());

        // check we can get "all" of them
        assert_eq!(ipam.filter("ref::some").len(),10);

        assert_eq!(ipam.filter("xtref::some_id_").len(),10);

        // lets find a single one by an Id
        let ce = ipam.find(&special_one);
        assert!(ce.is_some());

        let found_special = ce.unwrap_or_default();
        assert_eq!(None, found_special.sysref);
        assert_eq!(special_one, found_special.id);

    }

    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_ip_addresses() {
        let mut ipam = Ipam::new_with_protcol("My Ipam", IPProtocolFamily::V4);

        for _i in 0..100 {
            let net4 = get_net4_address();
            let cidr_entry = CidrEntry::from(IpNetwork::from(net4));
            let _ = ipam.add_entry(cidr_entry);
        }
        assert_eq!(ipam.cidrs.len(), 100);

        let json = serde_json::to_string_pretty(&ipam).expect("Should have worked");
        let mut f = File::create("assets/sample_ipam.json").expect("can't write the new file");
        f.write_all(json.as_bytes()).expect("Was meant to error");

        let missing =
            serde_json::to_string_pretty(&ipam.missing_supernets()).expect("Failure bro!");
        let mut f =
            File::create("assets/missing_supernets.json").expect("can't write the new file");
        f.write_all(missing.as_bytes()).expect("Was meant to error")
    }
}
