getinfo -- Command to receive all information about the Core Lightning node.

The **getinfo** gives a summary of the current running node.
Returns node information including:
- Identity: id (pubkey), alias (string), color (hex)
- Channel stats: num_peers, num_pending_channels, num_active_channels, num_inactive_channels
- Node info: version, lightning-dir, blockheight, network, fees_collected_msat
- Network: address (announced addresses), binding (listening addresses)
- Features: our_features (init, node, channel, invoice)

Warnings:
- warning_bitcoind_sync: When bitcoind is not up-to-date
- warning_lightningd_sync: When still loading blocks