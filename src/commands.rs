use cqrs_es::{AggregateError, Command};
use serde::{Deserialize, Serialize};

use std::collections::HashSet;
// use std::convert::TryFrom;
use std::str::FromStr;
use ipnetwork::IpNetwork;
use uuid::Uuid;

use crate::ipam_model::{Ipam, IPProtocolFamily, Label, IpamConfig, CidrEntry};
use crate::events::{IpamEvent, IpamCreated, CidrEntryAdded};
// use crate::error::IpamError;

// #[derive(Serialize, Deserialize)]
// pub struct AddAttributeToCidrEntry {
//     pub cidr: String,
//     pub attribute: Label,
// }

// #[derive(Serialize, Deserialize)]
// pub struct AddAttributeToCidrById {
//     pub id: String,
//     pub attribute: Label,
// }

// #[derive(Serialize, Deserialize)]
// pub struct RemoveAttributeByKeyFromCidr {
//     pub cidr: String,
//     pub key: String,
// }

// #[derive(Serialize, Deserialize)]
// pub struct RemoveAttributeByKeyFromCidrById {
//     pub id: String,
//     pub key: String,
// }

// #[derive(Serialize, Deserialize)]
// pub struct RemoveAttributeFromCidr {
//     pub cidr: String,
//     pub attribute: Label,
// }

// #[derive(Serialize, Deserialize)]
// pub struct RemoveAttributeFromCidrById {
//     pub id: String,
//     pub attribute: Label,
// }

/* ---- Creating new Ipam ------------------------ */
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CreateNewIpam {
    pub id: String,
    pub uuid: Uuid,
    pub protocol: IPProtocolFamily,
    pub cfg: Option<IpamConfig>,
}

impl Command<Ipam, IpamEvent> for CreateNewIpam {
    fn handle(self, ipam: &Ipam) -> Result<Vec<IpamEvent>, AggregateError> {

        


        println!(":: Create new IPAM [{}, {}]",self.id, self.uuid);
        
        let event_payload = IpamCreated  {
            uuid: self.uuid,
            id: self.id,
            protocol: self.protocol,
            cfg: self.cfg
        };
        Ok(vec![IpamEvent::IpamCreated(event_payload)])
    }
}

// impl Command<Ipam, IpamEvent> for CreateNewIpam {
//     fn handle(self, ig: &Ipam) -> Result<Vec<IpamEvent>, AggregateError> {

//         let ig.name = self.name;
        
//         let event_payload = IpamCreated  {
//             uuid: self.uuid,
//             name: self.name
//         };
//         Ok(vec![IpamEvent::IpamCreated(event_payload)])
//     }
// }

/* ---- Adding and Removing Cidr Entries ------------------------ */
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AddCidrEntry {
    pub cidr: String,
    pub uuid: Uuid,
    pub id: Option<String>,
    pub sysref: Option<String>,
    pub attributes: HashSet<Label>    
}

impl Command<Ipam, IpamEvent> for AddCidrEntry {
    fn handle(self, ipam: &Ipam) -> Result<Vec<IpamEvent>, AggregateError> {
        
        println!(":: Add Cidr Entry");

        let cidr = IpNetwork::from_str(self.cidr.as_str());
        if cidr.is_err() {
            return Err(AggregateError::TechnicalError(String::from("format was wrong")))
        } 

        if ipam.contains(cidr.unwrap()) {
            return Err(AggregateError::TechnicalError(String::from("cidr already exists")))
        }

        let mut cidr_entry = CidrEntry::try_from_with_extras(
            self.cidr.as_str(),
            self.id,
            self.sysref,
            self.attributes)?;

        // find the parent of this entry, we just want the id
        cidr_entry.parent = ipam.parent_of(cidr_entry.cidr).map(|r| r.id);

        // Create the event
        let event_payload = CidrEntryAdded { cidr_entry };
        Ok(vec![IpamEvent::CidrEntryAdded(event_payload)])
    }
}


// // #[derive(Serialize, Deserialize)]
// // pub struct ReleaseCidrEntry {
// //     pub cidr: String
// // }

// impl Command<Ipam, IpamEvent> for AddCidrEntry {
//     fn handle(self, ipam: &Ipam) -> Result<Vec<IpamEvent>, AggregateError> {
//         let initial = CidrEntry::try_from(self.cidr.as_str())
//         let entry = CidrEntry { 
//             ,id: self.id.or(Some(initial.id)).unwrap()
//             ,sysref: self.sysref
//             ,attributes: self.attributes
//             ..initial.clone()
//         }
//         ipam.
//         let event_payload = CidrEntryAdded { entry };
//         Ok(vec![IpamEvent::IpamEntryAdded(event_payload)])
//     }
// }

// impl Command<Ipam, IpamEvent> for ReleaseIpamEntry {
//     fn handle(self, ipam: &Ipam) -> Result<Vec<IpamEvent>, AggregateError> {
//         let balance = account.balance - self.amount;
//         if balance < 0_f64 {
//             return Err(AggregateError::new("funds not available"));
//         }
//         let event_payload = IpamEntryReleased {
//             amount: self.amount,
//             balance,
//         };
//         Ok(vec![IpamEvent::IpamEntryReleased(event_payload)])
//     }
// }

// impl Command<Ipam, IpamEvent> for AddAttributeToCidr {
//     fn handle(self, ipam: &Ipam) -> Result<Vec<IpamEvent>, AggregateError> {
//         let balance = account.balance - self.amount;
//         if balance < 0_f64 {
//             return Err(AggregateError::new("funds not available"));
//         }
//         let event_payload = AttributesAddedToCidr {
//             check_number: self.check_number,
//             amount: self.amount,
//             balance,
//         };
//         Ok(vec![IpamEvent::AttributesAddedToCidr(event_payload)])
//     }
// }
