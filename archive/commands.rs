use cqrs_es::{AggregateError, Command};
use serde::{Deserialize, Serialize};

use crate::aggregate::Ipam;
use crate::events::{ESIPAMOpened, IpamEvent, IpamEntryAdded, IpamEntryReleased, AttributesAddedToCidr};

#[derive(Serialize, Deserialize)]
pub struct NewIPAM {
    pub routing_domain_id: String
}

#[derive(Serialize, Deserialize)]
pub struct AddIpamEntry {
    pub cidr: String
}

#[derive(Serialize, Deserialize)]
pub struct ReleaseIpamEntry {
    pub cidr: String
}

#[derive(Serialize, Deserialize)]
pub struct AddAttributeToCidr {
    pub cidr: String,
    pub attribute: f64,
}

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