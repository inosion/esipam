use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

use crate::ipam_model::{Ipam, IPProtocolFamily, IpamConfig};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IpamEvent {
    IpamCreated(IpamCreated),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IpamCreated {
    pub uuid: Uuid,
    pub id: String,
    pub protocol: IPProtocolFamily,
    pub cfg: Option<IpamConfig>
}

impl DomainEvent<Ipam> for IpamCreated {
    fn apply(self, ipam: &mut Ipam) {  
        ipam.id = self.id
      }
}

impl DomainEvent<Ipam> for IpamEvent {
    fn apply(self, ipam: &mut Ipam) {
        match self {
            IpamEvent::IpamCreated(e) => { e.apply(ipam) }
        }
    }
}
