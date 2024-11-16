# scmp-tester
Build custom SCION Control Message Protocol to test the behaviour of echo requests with custom parameters.

**Command Overview:** [`scmp-tester`↴](#scmp-tester)

**Usage:** `scmp-tester [OPTIONS] --src-isd-as <SRC_ISD_AS> --dst-isd-as <DST_ISD_AS>`

**Options:**

* `-a`, `--src-isd-as <SRC_ISD_AS>` — Source ISD-AS to use. Structure: \<ISD>-\<AS>. Example: 18-ffaa:1:117c
* `-b`, `--dst-isd-as <DST_ISD_AS>` — Destination ISD-AS to use. Structure: \<ISD>-\<AS>. Example: 17-ffaa:1:117b
* `-s`, `--ipv4-src-addr <IPV4_SRC_ADDR>` — IPv4 source address. Default value: `127.0.0.1`
* `-d`, `--ipv4-dst-addr <IPV4_DST_ADDR>` — IPv4 destination address. Default value: `127.0.0.1`
* `-p`, `--udp-src-port <UDP_SRC_PORT>` — UDP source port to use. Default value: `32766`
* `-q`, `--udp-dst-port <UDP_DST_PORT>` — UDP destination port to use. Default value: `30001`
* `-x`, `--daemon-address <DAEMON_ADDRESS>` — The SCION dameon address. Default value: `https://localhost:30255`
* `-c`, `--payload <PAYLOAD>` — Optional custom payload added to the \"Data (variable Len)\" field as per IETF draft-dekater-scion-dataplane-03
* `-n`, `--path-id <PATH_ID>` — The path ID. Default value: `0`
* `-t`, `--time <TIME>` — Time (in milliseconds) between echo requests. Default value: `1000`
