use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

use crate::ipam_model::{CidrEntry, IPProtocolFamily, Ipam, IpamConfig};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IpamEvent {
    IpamCreated(IpamCreated),
    CidrEntryAdded(CidrEntryAdded),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IpamCreated {
    pub uuid: Uuid,
    pub id: String,
    pub protocol: IPProtocolFamily,
    pub cfg: Option<IpamConfig>,
}

impl DomainEvent<Ipam> for IpamCreated {
    fn apply(self, ipam: &mut Ipam) {
        ipam.id = self.id;
        ipam.protocol = self.protocol
    }
}

impl DomainEvent<Ipam> for IpamEvent {
    fn apply(self, ipam: &mut Ipam) {
        match self {
            IpamEvent::IpamCreated(e) => e.apply(ipam),
            IpamEvent::CidrEntryAdded(e) => e.apply(ipam),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CidrEntryAdded {
    pub cidr_entry: CidrEntry,
}

impl DomainEvent<Ipam> for CidrEntryAdded {
    fn apply(self, ipam: &mut Ipam) {
        
        match ipam.add_entry(self.cidr_entry) {
            Err(e) => (),// deal with it,
            Ok(ce) => { 
                // get the potential children of this entry
                let children = ipam.children_of(ce.cidr);
                // for each child, fire new commands to update the parent
                for c in children.iter() {
                    // need a new command to set a parent
                    println!(":: need to set parent of {} to {:?}", c, ce);
                }
            }
        }

    }
}
