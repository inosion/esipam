use cqrs_es::{EventEnvelope, Query, QueryProcessor};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ipam_model::Ipam;
use crate::events::IpamEvent;

pub struct SimpleLoggingQueryProcessor {}

impl QueryProcessor<Ipam, IpamEvent> for SimpleLoggingQueryProcessor {
    fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Ipam, IpamEvent>]) {
        for event in events {
            let payload = serde_json::to_string_pretty(&event.payload).unwrap();
            println!(":: <QueryProcessor> :{}-{}", aggregate_id, event.sequence);
            println!("::        <Payload> :\n{}", payload);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamSummaryView {
    uuid: Option<Uuid>,
    total_cidr_entries: u64,
}

impl Query<Ipam, IpamEvent> for IpamSummaryView {
    fn update(&mut self, event: &EventEnvelope<Ipam, IpamEvent>) {
        
        match &event.payload {
            IpamEvent::IpamCreated(payload) => {
                self.uuid = Some(payload.uuid.clone());
                println!(":: <Query<Ipam, IpamEvent> for IpamSummaryView> : IpamCreated {}", payload.uuid);
            },
            IpamEvent::CidrEntryAdded(p) => {
                println!(":: <Query<Ipam, IpamEvent> for IpamSummaryView> : CidrEntryAdded {}",p.cidr_entry.id);                
                self.total_cidr_entries = self.total_cidr_entries + 1;
            }
        }
    }
}

impl Default for IpamSummaryView {
    fn default() -> Self {
        IpamSummaryView {
            uuid: None,
            total_cidr_entries: Default::default(),
        }
    }
}

