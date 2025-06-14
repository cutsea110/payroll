use chrono::NaiveDate;

use payroll_domain::{EmployeeId, MemberId};
use tx_app::Transaction;

pub trait AddSalariedEmployeeTxFactory {
    fn mk_tx(&self, id: EmployeeId, name: &str, address: &str, salary: f32)
        -> Box<dyn Transaction>;
}

pub trait TxFactory: AddSalariedEmployeeTxFactory {
    fn mk_add_hourly_employee_tx(
        &self,
        id: EmployeeId,
        name: &str,
        address: &str,
        hourly_rate: f32,
    ) -> Box<dyn Transaction>;
    fn mk_add_salaried_employee_tx(
        &self,
        id: EmployeeId,
        name: &str,
        address: &str,
        salary: f32,
    ) -> Box<dyn Transaction> {
        AddSalariedEmployeeTxFactory::mk_tx(self, id, name, address, salary)
    }
    fn mk_add_commissioned_employee_tx(
        &self,
        id: EmployeeId,
        name: &str,
        address: &str,
        salary: f32,
        commission_rate: f32,
    ) -> Box<dyn Transaction>;
    fn mk_delete_employee_tx(&self, id: EmployeeId) -> Box<dyn Transaction>;
    fn mk_add_timecard_tx(
        &self,
        id: EmployeeId,
        date: NaiveDate,
        hours: f32,
    ) -> Box<dyn Transaction>;
    fn mk_add_sales_receipt_tx(
        &self,
        id: EmployeeId,
        date: NaiveDate,
        amount: f32,
    ) -> Box<dyn Transaction>;
    fn mk_add_service_charge_tx(
        &self,
        member_id: MemberId,
        date: NaiveDate,
        amount: f32,
    ) -> Box<dyn Transaction>;
    fn mk_change_employee_name_tx(&self, id: EmployeeId, new_name: &str) -> Box<dyn Transaction>;
    fn mk_change_employee_address_tx(
        &self,
        id: EmployeeId,
        new_address: &str,
    ) -> Box<dyn Transaction>;
    fn mk_change_employee_hourly_tx(
        &self,
        id: EmployeeId,
        hourly_rate: f32,
    ) -> Box<dyn Transaction>;
    fn mk_change_employee_salaried_tx(&self, id: EmployeeId, salary: f32) -> Box<dyn Transaction>;
    fn mk_change_employee_commissioned_tx(
        &self,
        id: EmployeeId,
        salary: f32,
        commission_rate: f32,
    ) -> Box<dyn Transaction>;
    fn mk_change_employee_hold_tx(&self, id: EmployeeId) -> Box<dyn Transaction>;
    fn mk_change_employee_direct_tx(
        &self,
        id: EmployeeId,
        bank: &str,
        account: &str,
    ) -> Box<dyn Transaction>;
    fn mk_change_employee_mail_tx(&self, id: EmployeeId, address: &str) -> Box<dyn Transaction>;
    fn mk_change_employee_member_tx(
        &self,
        id: EmployeeId,
        member_id: MemberId,
        dues: f32,
    ) -> Box<dyn Transaction>;
    fn mk_change_employee_no_member_tx(&self, id: EmployeeId) -> Box<dyn Transaction>;
    fn mk_payday_tx(&self, date: NaiveDate) -> Box<dyn Transaction>;
}
