use serde::{Deserialize, Serialize};
use std::collections::{HashSet};

use std::net::{Ipv4Addr, Ipv6Addr, IpAddr};
use uuid::Uuid;
use std::mem;


#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Label {
    key: String,
    value: String,
}


#[derive(Serialize, Deserialize)]
pub struct IpamConfig { 
    pub add_supernets: bool
}

impl Default for IpamConfig { 
    fn default() -> Self { 
        IpamConfig { 
            add_supernets: false
        }
    }
}


// #[derive(Hash, Eq, PartialEq, Serialize, Deserialize, Debug)]
// pub enum AttributeEntry {
//     Attr(Label),
//     SetOfAttr(HashSet<AttributeEntry>),
// }

#[derive(Serialize, Deserialize, Clone)]
pub struct CidrV4Entry {
    pub cidr: ipnetwork::Ipv4Network,
    pub id: String,
    pub uuid: Uuid,
    pub sysref: Option<String>,
    pub parent: Option<String>,
    pub attributes: HashSet<Label>, // would like to support a nested set of attributes here ideally
}
impl Default for CidrV4Entry {
    fn default() -> Self { 
        CidrV4Entry { 
            cidr: "0.0.0.0/0".parse().unwrap(),
            ..Default::default()
        }
    }
}

impl CidrV4Entry { 
    fn new(cidr: String) -> Self {
        let thecidr: ipnetwork::Ipv4Network = cidr.parse().unwrap();
        CidrV4Entry::new_from_ipnet(thecidr)
    }


    fn new_from_stdipv4(i: Ipv4Addr) -> Self {
        CidrV4Entry::new_from_ipnet(ipnetwork::Ipv4Network::from(i))
    }

    fn new_from_ipnet(i: ipnetwork::Ipv4Network) -> Self {
        let theuuid = Uuid::new_v4();

        CidrV4Entry {
          cidr: i,
          id: format!("{}_{}", theuuid, i),
          uuid: theuuid,
          sysref: None,
          parent: None,
          attributes: HashSet::default(),
        }
    }

}

/// The IpamV4 is the main object that stores CIDR entries that:
/// - are for one or more Routing Domain/ASNs (where IP conflicts are intended to not occur)
/// 
///   The entries inside am IPAM can be from one or more `Routing Domain`'s - [RFC-4632](https://tools.ietf.org/html/rfc4632#section-5.4) or Autonomous Systems;
///   so long as the entries are intended to never conflict. IPAM will only ever hold one CIDR entry, never duplicates.
///   
/// - a given IP Protocol V4. 
/// 
/// An IPAM is made up of a set of CidrEntries<Ipv4> entries.
#[derive(Serialize, Deserialize, Default)]
pub struct IpamV4 {
    pub id: String,
    pub cidrs: Vec<CidrV4Entry>,
    pub cfg: IpamConfig,
}

impl IpamV4 {
    fn new(an_id: String) -> Self {
        IpamV4 { 
            id: an_id,
            cidrs: vec![]
        }
    }

    fn add_entry(&mut self, entry: CidrV4Entry) {
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
    }

    fn replace(&mut self, idx: usize, new_entry: CidrV4Entry) -> CidrV4Entry {
        mem::replace(&mut self.cidrs[idx], new_entry)
    }

    fn size(&self) -> usize {
        self.cidrs.len()
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
    fn test_ip_addresses() { 

        let mut ipam = IpamV4::new(s!("My IPAM"));

        for _i in 0 .. 100 {
            let net4 = get_net4_address();
            let cidr_entry = CidrV4Entry::new_from_ipnet(net4);
            ipam.add_entry(cidr_entry);
        }
        assert_eq!(ipam.cidrs.len(), 100);

        let json = serde_json::to_string(&ipam).expect("Should have worked");
        println!("{}",json)

    }
}