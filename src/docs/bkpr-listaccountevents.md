lightning-bkpr-listaccountevents -- Command for listing recorded bookkeeping events
===================================================================================

SYNOPSIS
--------

**bkpr-listaccountevents** [*account*] [*payment\_id*] 

DESCRIPTION
-----------

The **bkpr-listaccountevents** RPC command is a list of all bookkeeping events that have been recorded for this node.

If the optional parameter **account** is set, we only emit events for the specified account, if exists.

If the optional parameter **payment\_id** is set, we only emit events which have that value as payment hash or as transaction id.

The parameters **account** and **payment\_id** are mutually exclusive.

Note that the type **onchain\_fees** that are emitted are of opposite credit/debit than as they appear in **listincome**, as **listincome** shows all events from the perspective of the node, whereas **listaccountevents** just dumps the event data as we've got it. Onchain fees are updated/recorded as we get more information about input and output spends -- the total onchain fees that were recorded for a transaction for an account can be found by summing all onchain fee events and taking the difference between the **credit\_msat** and **debit\_msat** for these events. We do this so that successive calls to **listaccountevents** always produce the same list of events -- no previously emitted event will be subsequently updated, rather we add a new event to the list.

- **account** (string, optional): Receive events for the specified account.
- **payment\_id** (string, optional): Receive events for the specified payment id. *(added v24.08)*

RETURN VALUE
------------

On success, an object containing **events** is returned. It is an array of objects, where each object contains:

- **account** (string): The account name. If the account is a channel, the channel\_id.
- **type** (string) (one of "onchain\_fee", "chain", "channel"): Coin movement type.
- **tag** (string): Description of movement.
- **credit\_msat** (msat): Amount credited.
- **debit\_msat** (msat): Amount debited.
- **currency** (string): Human-readable bech32 part for this coin type.
- **timestamp** (u32): Timestamp this event was recorded by the node. For consolidated events such as onchain\_fees, the most recent timestamp.

If **type** is "chain":
  - **outpoint** (string): The txid:outnum for this event.
  - **blockheight** (u32): For chain events, blockheight this occured at.
  - **description** (string, optional): The description of this event.
  - **origin** (string, optional): The account this movement originated from.
  - **payment\_id** (hex, optional): Lightning payment identifier. For an htlc, this will be the preimage.
  - **txid** (txid, optional): The txid of the transaction that created this event.

If **type** is "onchain\_fee":
  - **txid** (txid): The txid of the transaction that created this event.

If **type** is "channel":
  - **fees\_msat** (msat, optional): Amount paid in fees.
  - **is\_rebalance** (boolean, optional): Is this payment part of a rebalance.
  - **payment\_id** (hex, optional): Lightning payment identifier. For an htlc, this will be the preimage.
  - **part\_id** (u32, optional): Counter for multi-part payments.