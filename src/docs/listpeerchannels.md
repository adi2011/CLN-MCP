lightning-listpeerchannels -- Command returning data on channels of connected lightning nodes
**listpeerchannels** [*id*] 

Command *added* in v23.02.

The **listpeerchannels** RPC command returns list of this node's channels, with the possibility to filter them by peer's node id.

If no *id* is supplied, then channel data on all lightning nodes that are connected, or not connected but have open channels with this node, are returned.

- **id** (pubkey, optional): If supplied, limits the channels to just the peer with the given ID, if it exists.