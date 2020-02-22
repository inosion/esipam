use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

use crate::aggregate::Ipam;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IpamEvent {
    ESIPAMOpened(ESIPAMOpened),
    IpamEntryAdded(IpamEntryAdded),
    IpamEntryReleased(IpamEntryReleased),
    AttributesAddedToCidr(AttributesAddedToCidr),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ESIPAMOpened {
    pub account_id: String
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IpamEntryAdded {
    pub amount: f64,
    pub balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IpamEntryReleased {
    pub amount: f64,
    pub balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttributesAddedToCidr {
    pub check_number: String,
    pub amount: f64,
    pub balance: f64,
}

impl DomainEvent<Ipam> for IpamEvent {
    fn apply(self, account: &mut Ipam) {
        match self {
            IpamEvent::ESIPAMOpened(e) => { e.apply(account) }
            IpamEvent::IpamEntryAdded(e) => { e.apply(account) }
            IpamEvent::IpamEntryReleased(e) => { e.apply(account) }
            IpamEvent::AttributesAddedToCidr(e) => { e.apply(account) }
        }
    }
}

impl DomainEvent<Ipam> for ESIPAMOpened {
    fn apply(self, _account: &mut Ipam) {    }
}
impl DomainEvent<Ipam> for IpamEntryAdded {
    fn apply(self, account: &mut Ipam) {
        account.balance = self.balance;
    }
}

impl DomainEvent<Ipam> for IpamEntryReleased {
    fn apply(self, account: &mut Ipam) {
        account.balance = self.balance;
    }
}

impl DomainEvent<Ipam> for AttributesAddedToCidr {
    fn apply(self, account: &mut Ipam) {
        account.balance = self.balance;
    }
}