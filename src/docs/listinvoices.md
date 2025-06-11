lightning-listinvoices -- Command for querying invoice status
**listinvoices** [*label*] [*invstring*] [*payment\_hash*] [*offer\_id*] [*index* [*start*] [*limit*]] 

The **listinvoices** RPC command gets the status of a specific invoice, if it exists, or the status of all invoices if given no argument.

Only one of the query parameters can be used from *label*, *invstring*, *payment\_hash*, or *offer\_id*.

- **label** (one of, optional): A label used a the creation of the invoice to get a specific invoice.:
  - (string)
  - (integer)
- **invstring** (string, optional): The string value to query a specific invoice.
- **payment\_hash** (hex, optional): A payment\_hash of the invoice to get the details of a specific invoice.
- **offer\_id** (string, optional): A local `offer_id` the invoice was issued for a specific invoice details.
- **index** (string, optional) (one of "created", "updated"): If neither *in\_channel* nor *out\_channel* is specified, it controls ordering. The default is `created`. *(added v23.08)*
- **start** (u64, optional): If `index` is specified, `start` may be specified to start from that value, which is generally returned from lightning-wait(7). *(added v23.08)*
- **limit** (u32, optional): If `index` is specified, `limit` can be used to specify the maximum number of entries to return. *(added v23.08)*
