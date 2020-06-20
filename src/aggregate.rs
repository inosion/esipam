use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, HashMap};
use crate::ipam_model::{ IpamV4, IpamV6 };

impl Aggregate for IpamV4 {
    fn aggregate_type() -> &'static str {
        "IPAMV4"
   }
}

impl Aggregate for IpamV6 {
    fn aggregate_type() -> &'static str {
        "IPAMV6"
    }
}

#[cfg(test)]
mod aggregate_tests {
    use cqrs_es::test::TestFramework;

    use crate::aggregate::IpamV4;
    use crate::commands::{AddIPAMEntry, ReleaseIPAMEntry, AddAttributeToCidr, RemoveAttributeFromCidr};
    use crate::events::{IPAMEvent, IPAMEntryAdded, IPAMEntryReleased, AttributesAddedToCidr, AttributesRemovedDromCIDR};

    type ESIPAMTestFramework = TestFramework<IPAM, IPAMEvent>;

    #[test]
    fn test_add_entry() {
        let expected = IPAMEvent::IPAMEntryAdded(IPAMEntryAdded { cidr: "100.64.55.1" });
        ESIPAMTestFramework::default()
            .given_no_previous_events()
            .when(AddIPAMEntry {{ cidr: "100.64.55.1" })
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_add_attribute() {
        let previous = IPAMEvent::IPAMEntryAdded(IPAMEntryAdded { attributes: 200.0, balance: 200.0 });
        let expected = IPAMEvent::IPAMEntryAdded(IPAMEntryAdded { amount: 200.0, balance: 400.0 });
        ESIPAMTestFramework::default()
            .given(vec![previous])
            .when(AddIPAMEntry { amount: 200.0 })
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_withdraw_money() {
        let previous = IPAMEvent::IPAMEntryAdded(IPAMEntryAdded { amount: 200.0, balance: 200.0 });
        let expected = IPAMEvent::IPAMEntryReleased(IPAMEntryReleased { amount: 100.0, balance: 100.0 });
        ESIPAMTestFramework::default()
            .given(vec![previous])
            .when(ReleaseIPAMEntry { amount: 100.0 })
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_withdraw_money_funds_not_available() {
        ESIPAMTestFramework::default()
            .given_no_previous_events()
            .when(ReleaseIPAMEntry { amount: 200.0 })
            .then_expect_error("funds not available")
    }

    #[test]
    fn test_wrote_check() {
        let previous = IPAMEvent::IPAMEntryAdded(IPAMEntryAdded { amount: 200.0, balance: 200.0 });
        let expected = IPAMEvent::AttributesAddedToCidr(AttributesAddedToCidr { check_number: "1170".to_string(), amount: 100.0, balance: 100.0 });
        ESIPAMTestFramework::default()
            .given(vec![previous])
            .when(AddAttributeToCidr { check_number: "1170".to_string(), amount: 100.0 })
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_wrote_check_funds_not_available() {
        ESIPAMTestFramework::default()
            .given_no_previous_events()
            .when(AddAttributeToCidr { check_number: "1170".to_string(), amount: 100.0 })
            .then_expect_error("funds not available")
    }
}
