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

Simple Attribute Labelling method

# Vision / Epics

- [ ] Create the REST API and ~~Datamodel~~
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
