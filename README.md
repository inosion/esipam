# Event Sourced IPAM

Event Sourced, CQRS REST API to store your CIDR entries (IP Addresses and Subnets).

- Full Audit History
  - Which IPs did that System use , on what days? 
  - How many Systems did we have in 2015 ?
  - Which systems are currently active in AWS ?

- REST API
- Web Service
- Human UI
- Distributed, Eventually Consistent
- `Partition Tolerant` & `Available`, in CAP / Brewer's theorem.

Simple Attribute Labelling
```
      "attributes": [
        {
          "key": "mykey",
          "value": "1234"
        }
      ]
      
```

## Ipam Data Structure

The Ipam Data structure is simple enough.

```plantuml

class Ipam {
  id
  name
  protocol # v4 or v6
  cidrs: Vec<CidrEntry>
}
class CidrEntry { 
  id
  uuid
  cidr # v4 or v6
  sysref
  parent
  attributes: Set<Label>
}

class Label {
  Key=Value
}

Ipam "1" *-- "many" CidrEntry : contains Vector of
CidrEntry "1" *-- "many" Label : contains Set of
CidrEntry --> CidrEntry : on parent (id of)

```

# Vision / Epics

- [X] Create the REST API and Datamodel
- [ ] Create a Simple single site Aggregate Root for the ES-CQRS
- [ ] Create a CLI
- [ ] Create a UI to interact with
- [ ] Create application/system/provider specific plugins
  - [ ] Kubernetes 
  - [ ] Calico
  - [ ] DNSMasq DHCP Server / PFSense / OPNSense
  - [ ] AWS vPC
  - [ ] Azure vNet
  - [ ] Juniper
  - [ ] Cisco
  - [ ] Palo Alto
