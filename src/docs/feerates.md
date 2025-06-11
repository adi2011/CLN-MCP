lightning-feerates -- Command for querying recommended onchain feerates
=======================================================================
**feerates** *style* 


The **feerates** command returns the feerates that CLN will use. The feerates will be based on the recommended feerates from the backend. The backend may fail to provide estimates, but if it was able to provide estimates in the past, CLN will continue to use those for a while. CLN will also smoothen feerate estimations from the backend.

Explorers often present fees in "sat/vB": 4 sat/vB is `4000perkb` or `1000perkw`.

Bitcoin transactions have non-witness and witness bytes:

* Non-witness bytes count as 4 weight, 1 virtual byte. All bytes other than SegWit witness count as non-witness bytes. * Witness bytes count as 1 weight, 0.25 virtual bytes.

Thus, all *perkb* feerates will be exactly 4 times *perkw* feerates.

To compute the fee for a transaction, multiply its weight or virtual bytes by the appropriate *perkw* or *perkw* feerate returned by this command, then divide by 1000.

There is currently no way to change these feerates from the RPC. If you need custom control over onchain feerates, you will need to provide your own plugin that replaces the `bcli` plugin backend. For commands like lightning-withdraw(7) or lightning-fundchannel(7) you can provide a preferred feerate directly as a parameter, which will override the recommended feerates returned by **feerates**.

- **style** (string) (one of "perkb", "perkw"): Fee rate style to use. This can be:
     *perkw* - provide feerate in units of satoshis per 1000 weight (e.g. the minimum fee is usually `253perkw`).
     *perkb* - provide feerate in units of satoshis per 1000 virtual bytes (eg. the minimum fee is usually `1000perkb`).


Many other commands have a *feerate* parameter. This can be:

* One of the strings to use lightningd's internal estimates:
  * *urgent* (next 6 blocks or so)
  * *normal* (next 12 blocks or so)
  * *slow* (next 100 blocks or so)
  * *minimum* for the lowest value bitcoind will currently accept (added in v23.05)

* A number, with an optional suffix:
  * *blocks* means aim for confirmation in that many blocks (added in v23.05)
  * *perkw* means the number is interpreted as satoshi-per-kilosipa (weight)
  * *perkb* means it is interpreted bitcoind-style as satoshi-per-kilobyte. 

Omitting the suffix is equivalent to *perkb*.
