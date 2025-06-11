lightning-listaddresses -- Command to list all addresses issued by the node to date
===================================================================================
**listaddresses** [*address*] [*start*] [*limit*] 
Command *added* in v24.11.

The **listaddresses** RPC command provides a detailed list of all Bitcoin addresses that have been generated and issued by the Core Lightning node up to the current date.

- **address** (string, optional): A Bitcoin accepted type, including a bech32, address for lookup in the list of addresses issued to date.
- **start** (u64, optional): Starting key index for listing addresses or searching for a particular address. The default is 1.
- **limit** (u32, optional): The maximum number of addresses to return or search for. The default is Total number of addresses issued.