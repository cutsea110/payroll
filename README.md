# payroll

```mermaid
graph TD

  dao --> payroll-domain
  
  tx-impl --> dao
  tx-impl --> payroll-domain
  tx-impl --> tx-app
  
  tx-app --> payroll-domain

  tx-factory --> tx-app
  tx-factory --> payroll-domain
  
  text-parser-tx-source --> tx-app
  
  hs-db --> payroll-domain
  hs-db --> dao
  
  payroll-app --> hs-db
  payroll-app --> tx-impl
  payroll-app --> text-parser-tx-source
  payroll-app --> tx-app
  payroll-app --> tx-factory
```
