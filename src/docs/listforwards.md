lightning-listforwards -- Command showing all htlcs and their information
**listforwards** [*status*] [*in\_channel*] [*out\_channel*] [*index* [*start*] [*limit*]] 
The **listforwards** RPC command displays all htlcs that have been attempted to be forwarded by the Core Lightning node.

- **status** (string, optional) (one of "offered", "settled", "local\_failed", "failed"): If specified, then only the forwards with the given status are returned.
- **in\_channel** (short\_channel\_id, optional): Only the matching forwards on the given inbound channel are returned.
- **out\_channel** (short\_channel\_id, optional): Only the matching forwards on the given outbount channel are returned.
- **index** (string, optional) (one of "created", "updated"): If neither *in\_channel* nor *out\_channel* is specified, it controls ordering. The default is `created`. *(added v23.11)*
- **start** (u64, optional): If `index` is specified, `start` may be specified to start from that value, which is generally returned from lightning-wait(7). *(added v23.11)*
- **limit** (u32, optional): If `index` is specified, `limit` can be used to specify the maximum number of entries to return. *(added v23.11)*
