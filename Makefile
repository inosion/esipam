.PHONY: test-init test live.test live.test-init


random_ip = $(shell python3 -c 'import random; import socket; import struct; print("{}/{}".format(socket.inet_ntoa(struct.pack(">I", random.randint(10,0x10000ffff))),random.randint(8,32)))')
random_id = $(shell cat /proc/sys/kernel/random/uuid | cut -f5 -d-)
random_uuid = $(shell cat /proc/sys/kernel/random/uuid)

live.test-init:
	curl -H "Content-Type: application/json" -X POST -d '{"uuid": "195c5076-2c8f-4bed-94ae-79b11c39968c", "id": "ipam_1234", "protocol" : "V4"}' http://127.0.0.1:9090/api/ipam
live.test:
	jq -n '{"uuid": "$(call random_uuid)", "cidr": "$(call random_ip)","id": "$(call random_id)", "sysref":null, "attributes":[ ]}' | curl -H "Content-Type: application/json" -X POST -d@- http://127.0.0.1:9090/api/ipam/195c5076-2c8f-4bed-94ae-79b11c39968c/cidrs

test:
	cargo test
	cargo deny check
	
