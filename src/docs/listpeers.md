lightning-listpeers -- Command returning data on connected lightning nodes
**listpeers** [*id*] [*level*] 

The **listpeers** RPC command returns data on nodes that are connected or are not connected but have open channels with this node.

Once a connection to another lightning node has been established, using the **connect** command, data on the node can be returned using **listpeers** and the *id* that was used with the **connect** command.

If no *id* is supplied, then data on all lightning nodes that are connected, or not connected but have open channels with this node, are returned.

If a channel is open with a node and the connection has been lost, then the node will still appear in the output of the command and the value of the *connected* attribute of the node will be "false".

The channel will remain open for a set blocktime, after which if the connection has not been re-established, the channel will close and the node will no longer appear in the command output.

- **id** (pubkey, optional): If supplied, limits the result to just the peer with the given ID, if it exists.
- **level** (string, optional) (one of "io", "trace", "debug", "info", "unusual"): Supplying level will show log entries related to that peer at the given log level.
