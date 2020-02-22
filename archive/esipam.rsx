extern crate cqrs_es;


#[derive(Serialize, Deserialize)]
struct RoutingDomain {
    cidrs: Vec<str>
    name: str,
    network_protocol: str,
}

impl cqrs_es::Aggregate for RoutingDomain {
    fn aggregate_type() -> &'static str {
        "routing_domain"
    }
}

impl Default for RoutingDomain {
    fn default() -> Self {
        RoutingDomain {
            name: "default routing domain",
            network_protocol: "ipv4",
            cidrs: vec!["192.168.7.4/32", "192.168.7.0/23"]
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum RoutingDomainEvent {
    AddedNewIPAMEntry(AddedNewIPAMEntry),
    RemovedIPAMEntry(RemovedIPAMEntry),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct AddedNewIPAMEntry {
    cidr: str,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct RemovedIPAMEntry {
    cidr: str,
}

impl DomainEvent<RoutingDomain> for RoutingDomainEvent {
    fn apply(self, routing_domain: &mut RoutingDomain) {
        match self {
            RoutingDomainEvent::AddedNewIPAMEntry(e) => {e.apply(routing_domain)},
            RoutingDomainEvent::RemovedIPAMEntry(e) => {e.apply(routing_domain)},
        }
    }
}

impl DomainEvent<RoutingDomain> for AddedNewIPAMEntry {
    fn apply(self, routing_domain: &mut RoutingDomain) {
        routing_domain.cidrs = self.cidrs;
    }
}
impl DomainEvent<RoutingDomain> for RemovedIPAMEntry {
    fn apply(self, routing_domain: &mut RoutingDomain) {
        routing_domain.cidrs = self.cidrs;
    }
}


struct AddCIDR {
    cidr: str
}

struct RemoveCIDR {
    cidr: str
}


impl Command<RoutingDomain, RoutingDomainEvent> for AddCIDR {
    fn handle(self, routing_domain: &RoutingDomain) -> Result<Vec<RoutingDomainEvent>, AggregateError> {
        Ok(vec![])
    }
}

impl Command<RoutingDomain, RoutingDomainEvent> for RemoveCIDR {
    fn handle(self, routing_domain: &RoutingDomain) -> Result<Vec<RoutingDomainEvent>, AggregateError> {
        Ok(vec![])
    }
}s