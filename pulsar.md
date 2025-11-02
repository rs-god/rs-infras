# pulsar in docker
https://pulsar.apache.org/docs/2.9.x/getting-started-docker/
run pulsar in docker
```shell
docker run -itd \
-p 6650:6650 \
-p 8080:8080 \
--mount source=pulsardata,target=/pulsar/data \
--mount source=pulsarconf,target=/pulsar/conf \
apachepulsar/pulsar:2.9.4 \
bin/pulsar standalone
```

if you want see pulsar exec logs,please run cmd like this:
```shell
docker run -it \
-p 6650:6650 \
-p 8080:8080 \
--mount source=pulsardata,target=/pulsar/data \
--mount source=pulsarconf,target=/pulsar/conf \
apachepulsar/pulsar:2.9.4 \
bin/pulsar standalone
```

see pulsar exec status
```shell
docker ps | grep pulsar
% docker exec -it c5cf87d1411dffa98b62e71a68945abc637ebfcf52d787a1a269710c26849a65 /bin/bash
root@c5cf87d1411d:/pulsar# ls
LICENSE  README  conf  examples   lib       logs
NOTICE   bin     data  instances  licenses  pulsar-client
```

# pulsar message publish test
```shell
cargo test -v message_publish -- --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.74s
     Running unittests src/lib.rs (target/debug/deps/rs_infras-4de767efee3bc447)

running 1 test
producer connection ok
sent msg:Message { data: "hello: 0" }
1 messages
sent msg:Message { data: "hello: 1" }
2 messages
sent msg:Message { data: "hello: 2" }
3 messages
sent msg:Message { data: "hello: 3" }
4 messages
sent msg:Message { data: "hello: 4" }
5 messages
sent msg:Message { data: "hello: 5" }
6 messages
sent msg:Message { data: "hello: 6" }
7 messages
sent msg:Message { data: "hello: 7" }
8 messages
sent msg:Message { data: "hello: 8" }
9 messages
sent msg:Message { data: "hello: 9" }
10 messages
sent msg:Message { data: "hello: 10" }
11 messages
sent msg:Message { data: "hello: 11" }
12 messages
sent msg:Message { data: "hello: 12" }
13 messages
sent msg:Message { data: "hello: 13" }
14 messages
sent msg:Message { data: "hello: 14" }
15 messages
sent msg:Message { data: "hello: 15" }
16 messages
sent msg:Message { data: "hello: 16" }
17 messages
sent msg:Message { data: "hello: 17" }
18 messages
sent msg:Message { data: "hello: 18" }
19 messages
sent msg:Message { data: "hello: 19" }
20 messages
sent msg:Message { data: "hello: 20" }
21 messages
sent msg:Message { data: "hello: 21" }
22 messages
sent msg:Message { data: "hello: 22" }
23 messages
sent msg:Message { data: "hello: 23" }
24 messages
sent msg:Message { data: "hello: 24" }
25 messages
sent msg:Message { data: "hello: 25" }
26 messages
sent msg:Message { data: "hello: 26" }
27 messages
sent msg:Message { data: "hello: 27" }
28 messages
sent msg:Message { data: "hello: 28" }
29 messages
sent msg:Message { data: "hello: 29" }
30 messages
sent msg:Message { data: "hello: 30" }
31 messages
sent msg:Message { data: "hello: 31" }
32 messages
sent msg:Message { data: "hello: 32" }
33 messages
sent msg:Message { data: "hello: 33" }
34 messages
sent msg:Message { data: "hello: 34" }
35 messages
sent msg:Message { data: "hello: 35" }
36 messages
sent msg:Message { data: "hello: 36" }
37 messages
sent msg:Message { data: "hello: 37" }
38 messages
sent msg:Message { data: "hello: 38" }
39 messages
sent msg:Message { data: "hello: 39" }
40 messages
sent msg:Message { data: "hello: 40" }
41 messages
sent msg:Message { data: "hello: 41" }
42 messages
sent msg:Message { data: "hello: 42" }
43 messages
sent msg:Message { data: "hello: 43" }
44 messages
sent msg:Message { data: "hello: 44" }
45 messages
sent msg:Message { data: "hello: 45" }
46 messages
sent msg:Message { data: "hello: 46" }
47 messages
sent msg:Message { data: "hello: 47" }
48 messages
sent msg:Message { data: "hello: 48" }
49 messages
sent msg:Message { data: "hello: 49" }
50 messages
sent msg:Message { data: "hello: 50" }
51 messages
sent msg:Message { data: "hello: 51" }
52 messages
sent msg:Message { data: "hello: 52" }
53 messages
sent msg:Message { data: "hello: 53" }
54 messages
sent msg:Message { data: "hello: 54" }
55 messages
sent msg:Message { data: "hello: 55" }
56 messages
sent msg:Message { data: "hello: 56" }
57 messages
sent msg:Message { data: "hello: 57" }
58 messages
sent msg:Message { data: "hello: 58" }
59 messages
sent msg:Message { data: "hello: 59" }
60 messages
sent msg:Message { data: "hello: 60" }
61 messages
sent msg:Message { data: "hello: 61" }
62 messages
sent msg:Message { data: "hello: 62" }
63 messages
sent msg:Message { data: "hello: 63" }
64 messages
sent msg:Message { data: "hello: 64" }
65 messages
sent msg:Message { data: "hello: 65" }
66 messages
sent msg:Message { data: "hello: 66" }
67 messages
sent msg:Message { data: "hello: 67" }
68 messages
sent msg:Message { data: "hello: 68" }
69 messages
sent msg:Message { data: "hello: 69" }
70 messages
sent msg:Message { data: "hello: 70" }
71 messages
sent msg:Message { data: "hello: 71" }
72 messages
sent msg:Message { data: "hello: 72" }
73 messages
sent msg:Message { data: "hello: 73" }
74 messages
sent msg:Message { data: "hello: 74" }
75 messages
sent msg:Message { data: "hello: 75" }
76 messages
sent msg:Message { data: "hello: 76" }
77 messages
sent msg:Message { data: "hello: 77" }
78 messages
sent msg:Message { data: "hello: 78" }
79 messages
sent msg:Message { data: "hello: 79" }
80 messages
sent msg:Message { data: "hello: 80" }
81 messages
sent msg:Message { data: "hello: 81" }
82 messages
sent msg:Message { data: "hello: 82" }
83 messages
sent msg:Message { data: "hello: 83" }
84 messages
sent msg:Message { data: "hello: 84" }
85 messages
sent msg:Message { data: "hello: 85" }
86 messages
sent msg:Message { data: "hello: 86" }
87 messages
sent msg:Message { data: "hello: 87" }
88 messages
sent msg:Message { data: "hello: 88" }
89 messages
sent msg:Message { data: "hello: 89" }
90 messages
sent msg:Message { data: "hello: 90" }
91 messages
sent msg:Message { data: "hello: 91" }
92 messages
sent msg:Message { data: "hello: 92" }
93 messages
sent msg:Message { data: "hello: 93" }
94 messages
sent msg:Message { data: "hello: 94" }
95 messages
sent msg:Message { data: "hello: 95" }
96 messages
sent msg:Message { data: "hello: 96" }
97 messages
sent msg:Message { data: "hello: 97" }
98 messages
sent msg:Message { data: "hello: 98" }
99 messages
sent msg:Message { data: "hello: 99" }
100 messages
test xpulsar::xpulsar::tests::message_publish ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 1.84s
```

# pulsar message consumer test
```shell
% cargo test -v message_consumer -- --nocapture
metadata:MessageIdData { ledger_id: 1644, entry_id: 1098, partition: Some(-1), batch_index: None, ack_set: [], batch_size: None, first_chunk_message_id: None }
id:MessageIdData { ledger_id: 1644, entry_id: 1098, partition: Some(-1), batch_index: None, ack_set: [], batch_size: None, first_chunk_message_id: None }
got message data:hello: 98
got 99 messages
metadata:MessageIdData { ledger_id: 1644, entry_id: 1099, partition: Some(-1), batch_index: None, ack_set: [], batch_size: None, first_chunk_message_id: None }
id:MessageIdData { ledger_id: 1644, entry_id: 1099, partition: Some(-1), batch_index: None, ack_set: [], batch_size: None, first_chunk_message_id: None }
got message data:hello: 99
got 100 messages
```
