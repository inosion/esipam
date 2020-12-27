#[cfg(test)]
mod simple_application_tests {
    use cqrs_es::CqrsFramework;
    use cqrs_es::mem_store::MemStore;
    use uuid::Uuid;

    use crate::ipam_model::{ Ipam, IPProtocolFamily };
    use crate::commands::{ CreateNewIpam };
    use crate::events::IpamEvent;
    use crate::queries::SimpleLoggingQueryProcessor;

    #[test]
    fn test_event_store_single_command() {

        let event_store = MemStore::<Ipam, IpamEvent>::default();
        println!(">> Running the single ID line check");
        let query = SimpleLoggingQueryProcessor {};
        println!(">> Running the single ID line check");
        let cqrs = CqrsFramework::new(event_store, vec![Box::new(query)]);
        println!(">> Running the single ID line check");
        let res = cqrs.execute(Uuid::new_v4().to_string().as_str(), CreateNewIpam { id: String::from("test_ipam collection"), protocol: IPProtocolFamily::V6, cfg: None }).unwrap();
        println!(">> Running the single ID line check");
        res
    }
}