# SOLANA SIMD-47 testing

this is a test project just to print the syscall and sysvar data for the last restart slot.

Restart slot like this->
```
solana-ledger-tool -l ledger latest-optimistic-slots
```

gives

```
Slot                                         Hash                        Timestamp    Vote Only?
100370 BDwsHqN6Aio3LPxUivZgmb3BSr9CgGHBRpMZL8Q6uwDv    2023-06-07T09:59:57.773+00:00          true
```

Stop running validator and create a hard fork snapshot

```
solana-ledger-tool -l ledger create-snapshot 100370 ledger --hard-fork 100370

```

Patch the solana to include the feature to deploy program, deploy and tests.
