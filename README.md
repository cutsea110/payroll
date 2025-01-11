# delegation-study

```mermaid
graph TD

  dao --> domain
  tx --> dao
  tx --> domain
  tx --> tx-rs
  hs-db --> domain
  hs-db --> dao
  main --> hs-db
  main --> tx
```
