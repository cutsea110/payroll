# delegation-study

```mermaid
graph TD

  dao --> domain
  tx --> dao
  tx --> domain
  tx --> tx-rs
  hs-db --> dao
  hs-db --> domain
  main --> hs-db
  main --> tx
```
