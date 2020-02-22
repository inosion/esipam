use cqrs_es::{EventEnvelope, Query, QueryProcessor};
use serde::{Deserialize, Serialize};

use crate::aggregate::Ipam;
use crate::events::IpamEvent;

pub struct SimpleLoggingQueryProcessor {}

impl QueryProcessor<Ipam, IpamEvent> for SimpleLoggingQueryProcessor {
    fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Ipam, IpamEvent>]) {
        for event in events {
            let payload = serde_json::to_string_pretty(&event.payload).unwrap();
            println!("{}-{}\n{}", aggregate_id, event.sequence, payload);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamQuery {
    account_id: Option<String>,
    balance: f64,
    written_checks: Vec<String>,
}

impl Query<Ipam, IpamEvent> for IpamQuery {
    fn update(&mut self, event: &EventEnvelope<Ipam, IpamEvent>) {
        match &event.payload {
            IpamEvent::ESIPAMOpened(payload) => {
                self.account_id = Some(payload.account_id.clone());
            }
            IpamEvent::IpamEntryAdded(payload) => {
                self.balance = payload.balance;
            }
            IpamEvent::IpamEntryReleased(payload) => {
                self.balance = payload.balance;
            }
            IpamEvent::AttributesAddedToCidr(payload) => {
                self.balance = payload.balance;
                self.written_checks.push(payload.check_number.clone())
            }
        }
    }
}

impl Default for IpamQuery {
    fn default() -> Self {
        IpamQuery {
            account_id: None,
            balance: 0_f64,
            written_checks: Default::default(),
        }
    }
}

