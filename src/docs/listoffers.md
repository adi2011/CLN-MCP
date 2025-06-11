lightning-listoffers -- Command for listing offers
**listoffers** [*offer\_id*] [*active\_only*] 

The **listoffers** RPC command list all offers, or with `offer_id`, only the offer with that offer\_id (if it exists).

- **offer\_id** (hash, optional): Offer\_id to get details for (if it exists).
- **active\_only** (boolean, optional): If set and is true, only offers with `active` true are returned.
