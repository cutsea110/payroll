use chrono::NaiveDate;

// なににも依存しない (domain は当然 ok)
use payroll_domain::{EmployeeId, MemberId};
use tx_app::Transaction;

#[derive(Debug, Clone, PartialEq)]
pub enum Tx {
    AddHourlyEmployee(EmployeeId, String, String, f32),
    AddSalariedEmployee(EmployeeId, String, String, f32),
    AddCommissionedEmployee(EmployeeId, String, String, f32, f32),
    DeleteEmployee(EmployeeId),
    AddTimeCard(EmployeeId, NaiveDate, f32),
    AddSalesReceipt(EmployeeId, NaiveDate, f32),
    AddServiceCharge(MemberId, NaiveDate, f32),
    ChangeEmployeeName(EmployeeId, String),
    ChangeEmployeeAddress(EmployeeId, String),
    ChangeEmployeeHourly(EmployeeId, f32),
    ChangeEmployeeSalaried(EmployeeId, f32),
    ChangeEmployeeCommissioned(EmployeeId, f32, f32),
    ChangeEmployeeHold(EmployeeId),
    ChangeEmployeeDirect(EmployeeId, String, String),
    ChangeEmployeeMail(EmployeeId, String),
    ChangeEmployeeMember(EmployeeId, MemberId, f32),
    ChangeEmployeeNoMember(EmployeeId),
    Payday(NaiveDate),
}

pub trait TxFactory {
    fn convert(&self, tx: Tx) -> Box<dyn Transaction> {
        match tx {
            Tx::AddHourlyEmployee(id, name, address, hourly_rate) => {
                self.mk_add_hourly_employee_tx(id, &name, &address, hourly_rate)
            }
            Tx::AddSalariedEmployee(id, name, address, salary) => {
                self.mk_add_salaried_employee_tx(id, &name, &address, salary)
            }
            Tx::AddCommissionedEmployee(id, name, address, salary, commission_rate) => {
                self.mk_add_commissioned_employee_tx(id, &name, &address, salary, commission_rate)
            }
            Tx::DeleteEmployee(id) => self.mk_delete_employee_tx(id),
            Tx::AddTimeCard(id, date, hours) => self.mk_add_timecard_tx(id, date, hours),
            Tx::AddSalesReceipt(id, date, amount) => self.mk_add_sales_receipt_tx(id, date, amount),
            Tx::AddServiceCharge(member_id, date, amount) => {
                self.mk_add_service_charge_tx(member_id, date, amount)
            }
            Tx::ChangeEmployeeName(id, new_name) => self.mk_change_employee_name_tx(id, &new_name),
            Tx::ChangeEmployeeAddress(id, new_address) => {
                self.mk_change_employee_address_tx(id, &new_address)
            }
            Tx::ChangeEmployeeHourly(id, hourly_rate) => {
                self.mk_change_employee_hourly_tx(id, hourly_rate)
            }
            Tx::ChangeEmployeeSalaried(id, salary) => {
                self.mk_change_employee_salaried_tx(id, salary)
            }
            Tx::ChangeEmployeeCommissioned(id, salary, commission_rate) => {
                self.mk_change_employee_commissioned_tx(id, salary, commission_rate)
            }
            Tx::ChangeEmployeeHold(id) => self.mk_change_employee_hold_tx(id),
            Tx::ChangeEmployeeDirect(id, bank, account) => {
                self.mk_change_employee_direct_tx(id, &bank, &account)
            }
            Tx::ChangeEmployeeMail(id, address) => self.mk_change_employee_mail_tx(id, &address),
            Tx::ChangeEmployeeMember(emp_id, member_id, dues) => {
                self.mk_change_employee_member_tx(emp_id, member_id, dues)
            }
            Tx::ChangeEmployeeNoMember(id) => self.mk_change_employee_no_member_tx(id),
            Tx::Payday(date) => self.mk_payday_tx(date),
        }
    }

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
    ) -> Box<dyn Transaction>;
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
