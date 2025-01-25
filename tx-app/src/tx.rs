use anyhow;
use std::collections::HashMap;

use payroll_domain::{EmployeeId, Paycheck};

// トランザクションのインターフェース
#[derive(Debug, Clone)]
pub enum Response {
    Void,
    EmployeeId(EmployeeId),
    Paychecks(HashMap<EmployeeId, Paycheck>),
}
pub trait Transaction {
    fn execute(&self) -> Result<Response, anyhow::Error>;
}
