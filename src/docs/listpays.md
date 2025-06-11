lightning-listpays -- Command for querying payment status

**listpays** [*bolt11*] [*payment\_hash*] [*status*] [*index*] [*start*] [*limit*] 

The **listpays** RPC command gets the status of all *pay* commands (by combining results from listsendpays which lists every payment part), or a single one if either *bolt11* or *payment\_hash* was specified.

- **bolt11** (string, optional): Bolt11 string to get the payment details.
- **payment\_hash** (hash, optional): Payment hash to get the payment details.
- **status** (string, optional) (one of "pending", "complete", "failed"): To filter the payment by status.
- **index** (string, optional) (one of "created", "updated"): If neither *in\_channel* nor *out\_channel* is specified, it controls ordering, by `created` or `updated`. *(added v24.11)*
- **start** (u64, optional): If `index` is specified, `start` may be specified to start from that value, which is generally returned from lightning-wait(7).
 NOTE: if this is used, `amount_sent_msat` and `number_of_parts` fields may be lower than expected, as not all payment parts will be considered *(added v24.11)*
- **limit** (u32, optional): If `index` is specified, `limit` can be used to specify the maximum number of entries to return.
 NOTE: if this is used, `amount_sent_msat` and `number_of_parts` fields may be lower than expected, as not all payment parts will be considered
 NOTE: the actual number returned may be less than the limit, as individual payment parts are combined together *(added v24.11)*
