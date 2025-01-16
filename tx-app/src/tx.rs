use anyhow;

use payroll_domain::EmployeeId;

// トランザクションのインターフェース
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    Void,
    EmployeeId(EmployeeId),
}
pub trait Transaction {
    fn execute(&self) -> Result<Response, anyhow::Error>;
}
