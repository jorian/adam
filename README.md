# PBaaS Logos

A little app that scans the local system for natively installed PBaaS chains. It creates a RPC client for every chain and does a RPC to retrieve the name.

Initially, all chains are identified using the currencyidhex. This is a string hash of the name of the PBaaS chain that is OS safe. In general, you should assume that to be the name of a chain on a local machine. If the chain is started, a call to `getblockchaininfo` can be done to retrieve the actual name of the chain (which is Unicode and thus not OS safe).

If the chain is not started, the name will be an empty string.