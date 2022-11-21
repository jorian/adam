# PBaaS Logos

A little app that scans the local system for natively installed PBaaS chains. It creates a RPC client for every chain and does a RPC to retrieve the name.

Initially, all chains are identified using the currencyidhex. This is a string hash of the name of the PBaaS chain that is OS safe. In general, you should assume that to be the name of a chain on a local machine. If the chain is started, a call to `getblockchaininfo` can be done to retrieve the actual name of the chain (which is Unicode and thus not OS safe).

If the chain is not started, the name will be an empty string.

## Special case

The verus(test) chain is a special case. It does have a currencyidhex, but it is not used as such on the local system. Its blockchain also lives in a different location (`~/.komodo/`) than the PBaaS chains (`~/.verustest/`). 

As seen in `main.rs`, for Verus chains you would need to specify its currencyidhex manually. This is `2d4eb6919e9fdb2934ff2481325e6335a29eefa6` for `vrsctest` and is automatically done in `verus.rs`.