# delegation-study

```mermaid
graph TD

  dao --> domain
  tx --> dao
  tx --> domain
  tx --> tx-rs
  tx-factory --> dao
  tx-factory --> tx
  tx-factory --> domain
  hs-db --> domain
  hs-db --> dao
  main --> hs-db
  main --> tx-factory
  main --> tx
```
