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
            println!("{}-{}\n{}", aggregate_id, event.sequence, payload);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpamQuery {
    uuid: Option<Uuid>,
    ipamgroup_entries: Vec<String>,
}

impl Query<Ipam, IpamEvent> for IpamQuery {
    fn update(&mut self, event: &EventEnvelope<Ipam, IpamEvent>) {
        match &event.payload {
            IpamEvent::IpamCreated(payload) => {
                self.uuid = Some(payload.uuid.clone());
            }
        }
    }
}

impl Default for IpamQuery {
    fn default() -> Self {
        IpamQuery {
            uuid: None,
            ipamgroup_entries: Default::default(),
        }
    }
}

