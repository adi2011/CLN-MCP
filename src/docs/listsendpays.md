lightning-listsendpays -- Low-level command for querying sendpay status
**listsendpays** [*bolt11*] [*payment\_hash*] [*status*] [*index* [*start*] [*limit*]] 

The **listsendpays** RPC command gets the status of all *sendpay* commands (which is also used by the *pay* command), or with *bolt11* or *payment\_hash* limits results to that specific payment. You cannot specify both. It is possible to filter the payments also by *status*.

Note that there may be more than one concurrent *sendpay* command per *pay*, so this command should be used with caution.

- **bolt11** (string, optional): Bolt11 invoice.
- **payment\_hash** (hash, optional): The hash of the payment\_preimage.
- **status** (string, optional) (one of "pending", "complete", "failed"): Whether the invoice has been paid, pending, or failed.
- **index** (string, optional) (one of "created", "updated"): If neither bolt11 or payment\_hash is specified, `index` controls ordering, by `created` (default) or `updated`. *(added v23.11)*
- **start** (u64, optional): If `index` is specified, `start` may be specified to start from that value, which is generally returned from lightning-wait(7). *(added v23.11)*
- **limit** (u32, optional): If `index` is specified, `limit` can be used to specify the maximum number of entries to return. *(added v23.11)*

