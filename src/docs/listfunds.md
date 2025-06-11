lightning-listfunds -- Command showing all funds currently managed by the Core Lightning node
**listfunds** [*spent*] 
The **listfunds** RPC command displays all funds available, either in unspent outputs (UTXOs) in the internal wallet or funds locked in currently open channels.

- **spent** (boolean, optional): If True, then the *outputs* will include spent outputs in addition to the unspent ones. The default is False.
