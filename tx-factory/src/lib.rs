use chrono::NaiveDate;

use payroll_domain::{EmployeeId, MemberId};
use tx_app::Transaction;

pub trait AddSalariedEmployeeTxFactory {
    fn mk_tx(&self, id: EmployeeId, name: &str, address: &str, salary: f32)
        -> Box<dyn Transaction>;
}
pub trait AddHourlyEmployeeTxFactory {
    fn mk_tx(
        &self,
        id: EmployeeId,
        name: &str,
        address: &str,
        hourly_rate: f32,
    ) -> Box<dyn Transaction>;
}
pub trait AddCommissionedEmployeeTxFactory {
    fn mk_tx(
        &self,
        id: EmployeeId,
        name: &str,
        address: &str,
        salary: f32,
        commission_rate: f32,
    ) -> Box<dyn Transaction>;
}
pub trait DeleteEmployeeTxFactory {
    fn mk_tx(&self, id: EmployeeId) -> Box<dyn Transaction>;
}
pub trait AddTimecardTxFactory {
    fn mk_tx(&self, id: EmployeeId, date: NaiveDate, hours: f32) -> Box<dyn Transaction>;
}
pub trait AddSalesReceiptTxFactory {
    fn mk_tx(&self, id: EmployeeId, date: NaiveDate, amount: f32) -> Box<dyn Transaction>;
}
pub trait AddServiceChargeTxFactory {
    fn mk_tx(&self, member_id: MemberId, date: NaiveDate, amount: f32) -> Box<dyn Transaction>;
}
pub trait ChangeEmployeeNameTxFactory {
    fn mk_tx(&self, id: EmployeeId, new_name: &str) -> Box<dyn Transaction>;
}
pub trait ChangeEmployeeAddressTxFactory {
    fn mk_tx(&self, id: EmployeeId, new_address: &str) -> Box<dyn Transaction>;
}
pub trait ChangeEmployeeSalariedTxFactory {
    fn mk_tx(&self, id: EmployeeId, salary: f32) -> Box<dyn Transaction>;
}
pub trait ChangeEmployeeHourlyTxFactory {
    fn mk_tx(&self, id: EmployeeId, hourly_rate: f32) -> Box<dyn Transaction>;
}
pub trait ChangeEmployeeCommissionedTxFactory {
    fn mk_tx(&self, id: EmployeeId, salary: f32, commission_rate: f32) -> Box<dyn Transaction>;
}
pub trait ChangeEmployeeHoldTxFactory {
    fn mk_tx(&self, id: EmployeeId) -> Box<dyn Transaction>;
}
pub trait ChangeEmployeeDirectTxFactory {
    fn mk_tx(&self, id: EmployeeId, bank: &str, account: &str) -> Box<dyn Transaction>;
}
pub trait ChangeEmployeeMailTxFactory {
    fn mk_tx(&self, id: EmployeeId, address: &str) -> Box<dyn Transaction>;
}
pub trait ChangeEmployeeMemberTxFactory {
    fn mk_tx(&self, id: EmployeeId, member_id: MemberId, dues: f32) -> Box<dyn Transaction>;
}
pub trait ChangeEmployeeNoMemberTxFactory {
    fn mk_tx(&self, id: EmployeeId) -> Box<dyn Transaction>;
}
pub trait PaydayTxFactory {
    fn mk_tx(&self, date: NaiveDate) -> Box<dyn Transaction>;
}
