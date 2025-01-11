# delegation-study

```mermaid
graph TD

  dao --> domain
  tx-impl --> dao
  tx-impl --> domain
  tx-impl --> tx-rs
  hs-db --> dao
  hs-db --> domain
  main --> hs-db
  main --> tx-impl
```
