# payroll

```mermaid
graph TD

  dao --> domain
  
  tx --> dao
  tx --> domain
  tx --> tx-rs
  tx --> tx-app
  
  tx-app --> domain

  tx-factory --> tx-app
  tx-factory --> domain
  
  text-parser-tx-source --> tx-app
  
  hs-db --> domain
  hs-db --> dao
  
  main --> hs-db
  main --> tx
  main --> text-parser-tx-source
  main --> tx-app
  main --> tx-factory
```
