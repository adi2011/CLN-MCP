lightning-bkpr-listincome -- Command for listing all income impacting events
============================================================================

SYNOPSIS
--------

**bkpr-listincome** [*consolidate\_fees*] [*start\_time*] [*end\_time*] 

DESCRIPTION
-----------

Command *added* in pre-v0.10.1.

The **bkpr-listincome** RPC command is a list of all income impacting events that the bookkeeper plugin has recorded for this node.

- **consolidate\_fees** (boolean, optional): If true, we emit a single, consolidated event for any onchain-fees for a txid and account. Otherwise, events for every update to the onchain fee calculation for this account and txid will be printed. Note that this means that the events emitted are non-stable, i.e. calling **listincome** twice may result in different onchain fee events being emitted, depending on how much information we've logged for that transaction. The default is True.
- **start\_time** (u32, optional): UNIX timestamp (in seconds) that filters events after the provided timestamp. The default is zero.
- **end\_time** (u32, optional): UNIX timestamp (in seconds) that filters events up to and at the provided timestamp. The default is max-int.

RETURN VALUE
------------

On success, an object containing **income\_events** is returned. It is an array of objects, where each object contains:

- **account** (string): The account name. If the account is a channel, the channel\_id.
- **tag** (string): Type of income event.
- **credit\_msat** (msat): Amount earned (income).
- **debit\_msat** (msat): Amount spent (expenses).
- **currency** (string): Human-readable bech32 part for this coin type.
- **timestamp** (u32): Timestamp this event was recorded by the node. For consolidated events such as onchain\_fees, the most recent timestamp.
- **description** (string, optional): More information about this event. If a `invoice` type, typically the bolt11/bolt12 description.
- **outpoint** (string, optional): The txid:outnum for this event, if applicable.
- **txid** (txid, optional): The txid of the transaction that created this event, if applicable.
- **payment\_id** (hex, optional): Lightning payment identifier. For an htlc, this will be the preimage.