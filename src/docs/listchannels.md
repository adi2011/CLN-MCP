lightning-listchannels -- Command to query active lightning channels in the entire network
**listchannels** [*short\_channel\_id*] [*source*] [*destination*] 
The **listchannels** RPC command returns data on channels that are known to the node. Because channels may be bidirectional, up to 2 objects will be returned for each channel (one for each direction).

Only one of *short\_channel\_id*, *source* or *destination* can be supplied. If nothing is supplied, data on all lightning channels known to this node, are returned. These can be local channels or public channels broadcast on the gossip network.

- **short\_channel\_id** (short\_channel\_id, optional): If short\_channel\_id is a short channel id, then only known channels with a matching short\_channel\_id are returned. Otherwise, it must be null.
- **source** (pubkey, optional): If source is a node id, then only channels leading from that node id are returned.
- **destination** (pubkey, optional): If destination is a node id, then only channels leading to that node id are returned.
