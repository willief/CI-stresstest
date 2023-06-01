# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.5 (2023-05-04)

### Chore

 - <csr-id-c5b3c83c771cdc44cf304ea50b1fcc1586854072/> disable some testnet verfications and add Cargo.lock to version control
 - <csr-id-1457a453341e35ad3fbf426b4e1fa4a57a753761/> ensure testnet launch fails if build fails
 - <csr-id-de04d62f6dc155616c14e0f4a07f3b8205398b1b/> remove deps, remove EnvFilter
   We can specify log levels in the code as needed without having to bring in
   EnvFilter (and regex).
   
   Although right now regex is used elsewhere, we can hopefully remove that large dep
 - <csr-id-d748fcd6e6c3ba604fb898b3be8b73e96270e993/> fix naming
 - <csr-id-ba7c74175e7082f6a2d4afc64a85be2c56b9d8c9/> add docs + clippy fixes
 - <csr-id-f772949320519c868a5e2ffc3b611aa138567afd/> cargo fix

### New Features

 - <csr-id-a9e6906a4dfabe389a242afbe472bc7c87427b19/> update the user when nodes verification starts
 - <csr-id-7859c5ee7650ff26b2a1e7b7770aaee1af5692db/> compare nodes logs info with the info retrieved from their RPC service
 - <csr-id-5b266b8bbd1f46d8b87917d0573377ff1ecaf2f7/> exposing a gRPC interface on safenode bin/app
 - <csr-id-5ce1e89c56cebd9c61f8032c2ca86c258e5f033a/> make req/resp generic
 - <csr-id-514e8153bfc33cd5bb12e7998dd065e5f5c30c4c/> add some logging to dirs per node
 - <csr-id-e7f1da121e9b7afd2784caeab1fd8b826c47fa85/> use a random port @ startup, write config if none exists
 - <csr-id-fa4b3eacb4930749ad229cf2dbd26949b0a77a7e/> initial copy of testnet bin with basic tweaks.

### Bug Fixes

 - <csr-id-cf9a375790770deb31d88515204d09becb3c89c7/> it was reporting redundant info if it was spanned in more than one log files pere node
 - <csr-id-18241f6b280f460812acd743b601ad3c4cce5212/> add root dir to node startup
 - <csr-id-892c8b3abf332fbbe100bf04c0b04cc9e67be828/> add env filter and strip back testnet bin
 - <csr-id-5e633868773e42c13326c2f52790c94d4cd88ae0/> clippy lints
 - <csr-id-6190d222e04904baad12070f3893c2d0c425238a/> initial comms by writing 127.0.0.1 ip addre for genesis

### Test

 - <csr-id-d8fc275020bdff5c0d555ae0d0dcd59c3d63a65c/> CI network churning test

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 22 commits contributed to the release over the course of 41 calendar days.
 - 19 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - It was reporting redundant info if it was spanned in more than one log files pere node ([`cf9a375`](https://github.com/maidsafe/safe_network/commit/cf9a375790770deb31d88515204d09becb3c89c7))
    - Disable some testnet verfications and add Cargo.lock to version control ([`c5b3c83`](https://github.com/maidsafe/safe_network/commit/c5b3c83c771cdc44cf304ea50b1fcc1586854072))
    - Update the user when nodes verification starts ([`a9e6906`](https://github.com/maidsafe/safe_network/commit/a9e6906a4dfabe389a242afbe472bc7c87427b19))
    - Compare nodes logs info with the info retrieved from their RPC service ([`7859c5e`](https://github.com/maidsafe/safe_network/commit/7859c5ee7650ff26b2a1e7b7770aaee1af5692db))
    - Add root dir to node startup ([`18241f6`](https://github.com/maidsafe/safe_network/commit/18241f6b280f460812acd743b601ad3c4cce5212))
    - CI network churning test ([`d8fc275`](https://github.com/maidsafe/safe_network/commit/d8fc275020bdff5c0d555ae0d0dcd59c3d63a65c))
    - Exposing a gRPC interface on safenode bin/app ([`5b266b8`](https://github.com/maidsafe/safe_network/commit/5b266b8bbd1f46d8b87917d0573377ff1ecaf2f7))
    - Ensure testnet launch fails if build fails ([`1457a45`](https://github.com/maidsafe/safe_network/commit/1457a453341e35ad3fbf426b4e1fa4a57a753761))
    - Remove deps, remove EnvFilter ([`de04d62`](https://github.com/maidsafe/safe_network/commit/de04d62f6dc155616c14e0f4a07f3b8205398b1b))
    - Fix naming ([`d748fcd`](https://github.com/maidsafe/safe_network/commit/d748fcd6e6c3ba604fb898b3be8b73e96270e993))
    - Add docs + clippy fixes ([`ba7c741`](https://github.com/maidsafe/safe_network/commit/ba7c74175e7082f6a2d4afc64a85be2c56b9d8c9))
    - Make req/resp generic ([`5ce1e89`](https://github.com/maidsafe/safe_network/commit/5ce1e89c56cebd9c61f8032c2ca86c258e5f033a))
    - Add env filter and strip back testnet bin ([`892c8b3`](https://github.com/maidsafe/safe_network/commit/892c8b3abf332fbbe100bf04c0b04cc9e67be828))
    - Clippy lints ([`5e63386`](https://github.com/maidsafe/safe_network/commit/5e633868773e42c13326c2f52790c94d4cd88ae0))
    - 25 nodes and a couple of searches ([`1a22722`](https://github.com/maidsafe/safe_network/commit/1a22722198b5aecaca00dc167c7084d06f39160b))
    - Merge pull request #8 from joshuef/RandomPortNodes ([`34b2bfb`](https://github.com/maidsafe/safe_network/commit/34b2bfb7746fcd16f08aa2431181a502135b2865))
    - Initial comms by writing 127.0.0.1 ip addre for genesis ([`6190d22`](https://github.com/maidsafe/safe_network/commit/6190d222e04904baad12070f3893c2d0c425238a))
    - Add some logging to dirs per node ([`514e815`](https://github.com/maidsafe/safe_network/commit/514e8153bfc33cd5bb12e7998dd065e5f5c30c4c))
    - Cargo fix ([`f772949`](https://github.com/maidsafe/safe_network/commit/f772949320519c868a5e2ffc3b611aa138567afd))
    - Use a random port @ startup, write config if none exists ([`e7f1da1`](https://github.com/maidsafe/safe_network/commit/e7f1da121e9b7afd2784caeab1fd8b826c47fa85))
    - Merge pull request #6 from joshuef/AddTestnetBin ([`874c014`](https://github.com/maidsafe/safe_network/commit/874c01401acf980a226839247514e4bd69a58273))
    - Initial copy of testnet bin with basic tweaks. ([`fa4b3ea`](https://github.com/maidsafe/safe_network/commit/fa4b3eacb4930749ad229cf2dbd26949b0a77a7e))
</details>

## v0.1.4 (2023-03-23)

### New Features

 - <csr-id-16bb3389cdd665fe9a577587d9b7a6e8d21a3028/> exposing a gRPC interface on safenode bin/app
   - The safenode RPC service is exposed only when built with 'rpc-service' feature.
- The safenode RPC service code is generated automatically using gRPC (`tonic` crate)
   from a `proto` file with messages definitions added to sn_interface.
- The RPC is exposed at the same address as the node's address used for network connections,
   but using the subsequent port number.
- A new final step was implemented for the sn_testnet tool, to run a check on the launched nodes,
   verifying their names and network knowledge are the expected for the launched testnet.
- The new sn_testnet tool step is run only if built with 'verify-nodes' feature.
- Running the `verify-nodes` check of sn_testnet in CI previous to sn_client e2e tests.

## v0.1.3 (2023-03-22)

<csr-id-b0627339e2458fd762084cc4805d7adedfd8c05e/>
<csr-id-c9f3e7ccad8836c609193f1c6b53f351e5705805/>
<csr-id-50f6ede2104025bd79de8922ca7f27c742cf52bb/>
<csr-id-807d69ef609decfe94230e2086144afc5cc56d7b/>
<csr-id-1a8b9c9ba5b98c0f1176a0ccbce53d4acea8c84c/>
<csr-id-d3c6c9727a69389f4204b746c54a537cd783232c/>
<csr-id-22c6e341d28c913a3acaaeae0ceeb8c0a1ef4d4e/>

### Chore

 - <csr-id-b0627339e2458fd762084cc4805d7adedfd8c05e/> sn_testnet-0.1.3/sn_interface-0.20.7/sn_comms-0.6.4/sn_client-0.82.4/sn_node-0.80.1/sn_api-0.80.3/sn_cli-0.74.2
 - <csr-id-c9f3e7ccad8836c609193f1c6b53f351e5705805/> sn_node-0.80.0
 - <csr-id-50f6ede2104025bd79de8922ca7f27c742cf52bb/> sn_interface-0.20.6/sn_comms-0.6.3/sn_client-0.82.3/sn_node-0.79.0/sn_cli-0.74.1
 - <csr-id-807d69ef609decfe94230e2086144afc5cc56d7b/> sn_interface-0.20.6/sn_comms-0.6.3/sn_client-0.82.3/sn_node-0.79.0/sn_cli-0.74.1
 - <csr-id-1a8b9c9ba5b98c0f1176a0ccbce53d4acea8c84c/> safenode renaming

### Chore

 - <csr-id-22c6e341d28c913a3acaaeae0ceeb8c0a1ef4d4e/> sn_testnet-0.1.3/sn_interface-0.20.7/sn_comms-0.6.4/sn_client-0.82.4/sn_node-0.80.1/sn_api-0.80.3/sn_cli-0.74.2

### Refactor

 - <csr-id-d3c6c9727a69389f4204b746c54a537cd783232c/> remove unused wiremsg-debuginfo ft

## v0.1.2 (2023-03-16)

<csr-id-50f6ede2104025bd79de8922ca7f27c742cf52bb/>
<csr-id-807d69ef609decfe94230e2086144afc5cc56d7b/>
<csr-id-1a8b9c9ba5b98c0f1176a0ccbce53d4acea8c84c/>

### Chore

 - <csr-id-50f6ede2104025bd79de8922ca7f27c742cf52bb/> sn_interface-0.20.6/sn_comms-0.6.3/sn_client-0.82.3/sn_node-0.79.0/sn_cli-0.74.1
 - <csr-id-807d69ef609decfe94230e2086144afc5cc56d7b/> sn_interface-0.20.6/sn_comms-0.6.3/sn_client-0.82.3/sn_node-0.79.0/sn_cli-0.74.1
 - <csr-id-1a8b9c9ba5b98c0f1176a0ccbce53d4acea8c84c/> safenode renaming

## v0.1.1 (2023-03-16)

<csr-id-807d69ef609decfe94230e2086144afc5cc56d7b/>
<csr-id-1a8b9c9ba5b98c0f1176a0ccbce53d4acea8c84c/>

### Chore

 - <csr-id-807d69ef609decfe94230e2086144afc5cc56d7b/> sn_interface-0.20.6/sn_comms-0.6.3/sn_client-0.82.3/sn_node-0.79.0/sn_cli-0.74.1
 - <csr-id-1a8b9c9ba5b98c0f1176a0ccbce53d4acea8c84c/> safenode renaming

## v0.1.0 (2023-03-16)

<csr-id-4f04bd1a5d1c747bfc6b5d39824dd108f8546b7b/>
<csr-id-1c621d13b5edfc21ed85da7498d24c5db038795a/>

### Chore

 - <csr-id-4f04bd1a5d1c747bfc6b5d39824dd108f8546b7b/> rename testnet crate to sn_testnet
   Even though the `testnet` crate name is not taken on crates.io, I think it makes sense to prefix
   this crate with `sn_`, as per our other crates. The name of the binary does not change. This crate
   needs to be published because `sn_client` has a dependency on it.
   
   This also provides a README for the crate, which was necessary to have it published.

### Other

 - <csr-id-1c621d13b5edfc21ed85da7498d24c5db038795a/> temporarily prevent workflows running
   I want to temporarily disable the version bump and release workflows from running so that I can
   manually publish the new testnet crate and delete the tags from the last bad release.

