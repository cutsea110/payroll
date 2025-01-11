# delegation-study

```mermaid
graph TD

  dao --> domain
  abstract-tx --> dao
  abstract-tx --> domain
  tx-impl --> dao
  tx-impl --> abstract-tx
  tx-impl --> domain
  tx-impl --> tx-rs
  hs-db --> domain
  hs-db --> dao
  main --> hs-db
  main --> tx-impl
```
