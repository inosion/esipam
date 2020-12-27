.PHONY: test-init test


random_ip = $(shell python3 -c 'import random; import socket; import struct; print("{}/{}".format(socket.inet_ntoa(struct.pack(">I", random.randint(10,0x10000ffff))),random.randint(8,32)))')
random_id = $(shell cat /proc/sys/kernel/random/uuid | cut -f5 -d-)
random_uuid = $(shell cat /proc/sys/kernel/random/uuid)


test-init:
	  curl -X POST -d '{"id": "ipam_1234", "protocol" : "V4"}' http://127.0.0.1:9090/ipam/createIpam/ipam_1234
test:
	  jq -n '{"cidr": "$(call random_ip)","id": "$(call random_id)", "sysref":null, "attributes":[ ]}' | curl -X POST -d@- http://127.0.0.1:9090/ipam/addCidrEntry/ipam_1234
	
