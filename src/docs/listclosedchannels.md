lightning-listclosedchannels -- Get data on our closed historical channels
**listclosedchannels** [*id*] 
Command *added* in v23.05.
The **listclosedchannels** RPC command returns data on channels which are otherwise forgotten (more than 100 blocks after they're completely resolved onchain).

- **id** (pubkey, optional): If no *id* is supplied, then channel data on all historical channels are given. Supplying *id* will filter the results to only match channels to that peer. Note that prior to v23.05, old peers were forgotten.
