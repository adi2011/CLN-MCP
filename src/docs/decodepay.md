lightning-decodepay -- Command for decoding a bolt11 string (low-level)
=======================================================================

SYNOPSIS
--------

**decodepay** *bolt11* [*description*] 

DESCRIPTION
-----------

Command *added* in v23.05.

WARNING: deprecated: use *decode* which also handles bolt12.

The **decodepay** RPC command checks and parses a *bolt11* string as specified by the BOLT 11 specification.

- **bolt11** (string): Bolt11 invoice to decode.
- **description** (string, optional): Description of the invoice to decode.

RETURN VALUE
------------

On success, an object is returned, containing:

- **currency** (string): The BIP173 name for the currency.
- **created\_at** (u64): The UNIX-style timestamp of the invoice.
- **expiry** (u64): The number of seconds this is valid after *timestamp*.
- **payee** (pubkey): The public key of the recipient.
- **payment\_hash** (hash): The hash of the *payment\_preimage*.
- **signature** (signature): Signature of the *payee* on this invoice.
- **min\_final\_cltv\_expiry** (u32): The minimum CLTV delay for the final node.
- **amount\_msat** (msat, optional): Amount the invoice asked for.
- **description** (string, optional): The description of the purpose of the purchase.
- **description\_hash** (hash, optional): The hash of the description, in place of *description*.
- **payment\_secret** (hash, optional): The secret to hand to the payee node.
- **features** (hex, optional): The features bitmap for this invoice.
- **payment\_metadata** (hex, optional): The payment\_metadata to put in the payment.
- **fallbacks** (array of objects, optional): Onchain addresses.:
  - **type** (string) (one of "P2PKH", "P2SH", "P2WPKH", "P2WSH", "P2TR"): The address type (if known).
  - **hex** (hex): Raw encoded address.
  - **addr** (string, optional): The address in appropriate format for *type*.
- **routes** (array of arrays, optional): Route hints to the *payee*.:
  - (array of objects): Hops in the route.
    - **pubkey** (pubkey): The public key of the node.
    - **short\_channel\_id** (short\_channel\_id): A channel to the next peer.
    - **fee\_base\_msat** (msat): The base fee for payments.
    - **fee\_proportional\_millionths** (u32): The parts-per-million fee for payments.
    - **cltv\_expiry\_delta** (u32): The CLTV delta across this hop.
- **extra** (array of objects, optional): Any extra fields we didn't know how to parse.:
  - **tag** (string) (always 1 characters): The bech32 letter which identifies this field.
  - **data** (string): The bech32 data for this field.
