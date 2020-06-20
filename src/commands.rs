use cqrs_es::{AggregateError, Command};
use serde::{Deserialize, Serialize};

use std::collections::HashSet;
use crate::aggregate::Ipam;
use crate::ipam_model::{IpamV4, IPProtocolFamily, Label};
use crate::events::{ESIPAMOpened, IpamEvent, IpamEntryAdded, IpamEntryReleased, AttributesAddedToCidr};

#[derive(Serialize, Deserialize)]
pub struct CreateANewIPAM {
    pub id: String,
    pub protocol: IPProtocolFamily
}

#[derive(Serialize, Deserialize)]
pub struct AddCidrEntry {
    pub cidr: String,
    pub id: Option<String>,
    pub sysref: Option<String>,
    pub parent: Option<String>,
    pub attributes: HashSet<Label>    
}

#[derive(Serialize, Deserialize)]
pub struct ReleaseIpamEntry {
    pub cidr: String
}

#[derive(Serialize, Deserialize)]
pub struct AddAttributeToCidr {
    pub cidr: String,
    pub attribute: Label,
}

#[derive(Serialize, Deserialize)]
pub struct AddAttributeToCidrById {
    pub id: String,
    pub attribute: Label,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveAttributeByKeyFromCidr {
    pub cidr: String,
    pub key: String,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveAttributeByKeyFromCidrById {
    pub id: String,
    pub attribute: Label,
}

// TODO from here down

impl Command<Ipam, IpamEvent> for OpenESIPAM {
    fn handle(self, _account: &Ipam) -> Result<Vec<IpamEvent>, AggregateError> {
        let event_payload = ESIPAMOpened  {
            account_id: self.account_id
        };
        Ok(vec![IpamEvent::ESIPAMOpened(event_payload)])
    }
}

impl Command<Ipam, IpamEvent> for AddIpamEntry {
    fn handle(self, account: &Ipam) -> Result<Vec<IpamEvent>, AggregateError> {
        let balance = account.balance + self.amount;
        let event_payload = IpamEntryAdded {
            amount: self.amount,
            balance,
        };
        Ok(vec![IpamEvent::IpamEntryAdded(event_payload)])
    }
}

impl Command<Ipam, IpamEvent> for ReleaseIpamEntry {
    fn handle(self, account: &Ipam) -> Result<Vec<IpamEvent>, AggregateError> {
        let balance = account.balance - self.amount;
        if balance < 0_f64 {
            return Err(AggregateError::new("funds not available"));
        }
        let event_payload = IpamEntryReleased {
            amount: self.amount,
            balance,
        };
        Ok(vec![IpamEvent::IpamEntryReleased(event_payload)])
    }
}

impl Command<Ipam, IpamEvent> for AddAttributeToCidr {
    fn handle(self, account: &Ipam) -> Result<Vec<IpamEvent>, AggregateError> {
        let balance = account.balance - self.amount;
        if balance < 0_f64 {
            return Err(AggregateError::new("funds not available"));
        }
        let event_payload = AttributesAddedToCidr {
            check_number: self.check_number,
            amount: self.amount,
            balance,
        };
        Ok(vec![IpamEvent::AttributesAddedToCidr(event_payload)])
    }
}
