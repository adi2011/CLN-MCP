lightning-getroute -- Command for routing a payment (low-level)
===============================================================
**getroute** *id* *amount\_msat* *riskfactor* [*cltv*] [*fromid*] [*fuzzpercent*] [*exclude*] [*maxhops*] 

The **getroute** RPC command attempts to find the best route for the payment of *amount\_msat* to lightning node *id*, such that the payment will arrive at *id* with *cltv*.

There are two considerations for how good a route is: how low the fees are, and how long your payment will get stuck in a delayed output if a node goes down during the process. .

- **id** (pubkey): Node pubkey to find the best route for the payment.
- **amount\_msat** (msat): Amount to send. It can be a whole number, or a whole number ending in *msat* or *sat*, or a number with three decimal places ending in *sat*, or a number with 1 to 11 decimal places ending in *btc*. The 0 value is special: it ignores any *htlc\_minimum\_msat* setting on channels, and simply returns a possible route (if any) which is useful for simple probing.
- **riskfactor** (u64): A non-negative floating-point field controls this tradeoff; it is the annual cost of your funds being stuck (as a percentage). For example, if you thought the convenience of keeping your funds liquid (not stuck) was worth 20% per annum interest, *riskfactor* would be 20. If you didn't care about risk, *riskfactor* would be zero.
- **cltv** (u32, optional): Cltv-blocks to spare. The default is 9.
- **fromid** (pubkey, optional): The node to start the route from. The default is this node.
- **fuzzpercent** (u32, optional): Used to distort fees to provide some randomization to the route generated, but it was not properly implemented and is ignored.
- **exclude** (array of strings, optional): A JSON array of short-channel-id/direction (e.g. ['564334x877x1/0', '564195x1292x0/1' ]) or node-id which should be excluded from consideration for routing. Note if the source or destination is excluded, the command result is undefined. The default is not to exclude any channels or nodes.:
  - (string, optional)
- **maxhops** (u32, optional): The maximum number of channels to return. The default is 20.

RISKFACTOR EFFECT ON ROUTING
----------------------------

The risk factor is treated as if it were an additional fee on the route, for the purposes of comparing routes.

The formula used is the following approximation:

    risk-fee = amount x blocks-timeout x per-block-cost

We are given a *riskfactor* expressed as a percentage. There are 52596 blocks per year, thus *per-block-cost* is *riskfactor* divided by 5,259,600.

The final result is:

    risk-fee = amount x blocks-timeout x riskfactor / 5259600

RECOMMENDED RISKFACTOR VALUES
-----------------------------

The default *fuzz* factor is 5%, so as you can see from the table above, that tends to overwhelm the effect of *riskfactor* less than about 5.

1 is a conservative value for a stable lightning network with very few failures.

1000 is an aggressive value for trying to minimize timeouts at all costs.

The default for lightning-pay(7) is 10, which starts to become a major factor for larger amounts, and is basically ignored for tiny ones.