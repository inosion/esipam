#[cfg(test)]
mod aggregate_tests {
    use super::*;
    use cqrs_es::test::TestFramework;

    type RoutingDomainTestFramework = TestFramework<RoutingDomain,RoutingDomainEvent>;

    #[test]
    fn test_add_cidr() {
        let expected = RoutingDomainEvent::AddedNewIPAMEntry(AddedNewIPAMEntry { cidr: "192.168.6.5" });

     RoutingDomainTestFramework::default()
        .given_no_previous_events()
        .when(AddCIDR{ cidr: "192.168.6.5" })
        .then_expect_events(vec![expected]);
    }

}