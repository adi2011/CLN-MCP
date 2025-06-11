lightning-listinvoicerequests -- Command for querying invoice\_request status
**listinvoicerequests** [*invreq\_id*] [*active\_only*] 
Command *added* in v22.11.

The **listinvoicerequests** RPC command gets the status of a specific `invoice_request`, if it exists, or the status of all `invoice_requests` if given no argument.

- **invreq\_id** (string, optional): A specific invoice can be queried by providing the `invreq_id`, which is presented by lightning-invoicerequest(7), or can be calculated from a bolt12 invoice.
- **active\_only** (boolean, optional): If it is *True* then only active invoice requests are returned. The default is *False*.
