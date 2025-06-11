lightning-listhtlcs -- Command for querying HTLCs
**listhtlcs** [*id*] 
The **listhtlcs** RPC command gets all HTLCs (which, generally, we remember for as long as a channel is open, even if they've completed long ago).

- **id** (string, optional): A short channel id (e.g. 1x2x3) or full 64-byte hex channel id, it will only list htlcs for that channel (which must be known).
