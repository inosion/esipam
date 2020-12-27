use cqrs_es::{AggregateError, Command};
use serde::{Deserialize, Serialize};

use std::collections::HashSet;
use crate::ipam_model::{Ipam, IPProtocolFamily, Label, IpamConfig};
use crate::events::{IpamEvent, IpamCreated};

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
#[derive(Serialize, Deserialize)]
pub struct CreateNewIpam {
    pub id: String,
    pub protocol: IPProtocolFamily,
    pub cfg: Option<IpamConfig>,
}

impl Command<Ipam, IpamEvent> for CreateNewIpam {
    fn handle(self, ipam: &Ipam) -> Result<Vec<IpamEvent>, AggregateError> {

        println!("Handling new entry");
        
        let event_payload = IpamCreated  {
            uuid: ipam.uuid,
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

// /* ---- Adding and Removing Cidr Entries ------------------------ */
// #[derive(Serialize, Deserialize)]
// pub struct AddCidrEntry {
//     pub cidr: String,
//     pub id: Option<String>,
//     pub sysref: Option<String>,
//     pub attributes: HashSet<Label>    
// }

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
