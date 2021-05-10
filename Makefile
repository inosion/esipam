.DEFAULT_GOAL = all
.PHONY: test-init test live.test live.test-init



random_ip   = $(shell python3 -c 'import random; import socket; import struct; print("{}/{}".format(socket.inet_ntoa(struct.pack(">I", random.randint(10,0x10000ffff))),random.randint(8,32)))')
random_id   = $(shell cat /proc/sys/kernel/random/uuid | cut -f5 -d-)
random_uuid = $(shell cat /proc/sys/kernel/random/uuid)


test_ipam_uuid_file = target/data/ipam.uuid
test_ipam_uuid      = $(shell cat $(test_ipam_uuid_file))

$(test_ipam_uuid_file):
	$(dirguard)
	echo $(random_uuid) > $@

.PHONY: e2e.test.init e2e.test
e2e.test.init: $(test_ipam_uuid_file) ## @e2e-testing Initialise an IPAM Repo
target/data/ipam.inited: $(test_ipam_uuid_file)
	curl -H "Content-Type: application/json" -X POST -d '{"uuid": "'$(test_ipam_uuid)'", "id": "ipam_1234", "protocol" : "V4"}' http://127.0.0.1:9090/api/ipam > $@

e2e.test: target/data/ipam.inited ## @e2e-testing Create a Random CIDR Entry
	jq -n '{"uuid": "$(call random_uuid)", "cidr": "$(call random_ip)","id": "$(call random_id)", "sysref":null, "attributes":[ ]}' | curl -H "Content-Type: application/json" -X POST -d@- http://127.0.0.1:9090/api/ipam/$(test_ipam_uuid)/cidrs

test: ## @test Run all Tests
	cargo test
	cargo deny check

include help.mk
include common.mk
