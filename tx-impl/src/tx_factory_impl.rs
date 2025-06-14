use chrono::NaiveDate;
use log::trace;

use crate::{
    AddCommissionedEmployeeTx, AddHourlyEmployeeTx, AddSalariedEmployeeTx, AddSalesReceiptTx,
    AddServiceChargeTx, AddTimeCardTx, ChangeCommissionedTx, ChangeDirectTx,
    ChangeEmployeeAddressTx, ChangeEmployeeNameTx, ChangeHoldTx, ChangeHourlyTx, ChangeMailTx,
    ChangeMemberTx, ChangeNoMemberTx, ChangeSalariedTx, DeleteEmployeeTx, PaydayTx,
};
use dao::EmployeeDao;
use payroll_domain::{EmployeeId, MemberId};
use payroll_factory::PayrollFactory;
use tx_app::Transaction;
use tx_factory::TxFactory;

pub struct TxFactoryImpl<T, F>
where
    T: EmployeeDao,
{
    dao: T,
    payroll_factory: F,
}
impl<T, F> TxFactoryImpl<T, F>
where
    T: EmployeeDao,
{
    pub fn new(dao: T, payroll_factory: F) -> Self {
        Self {
            dao,
            payroll_factory,
        }
    }
}
impl<T, F> TxFactory for TxFactoryImpl<T, F>
where
    T: EmployeeDao + Clone + 'static,
    F: PayrollFactory + Clone + 'static,
{
    fn mk_add_hourly_employee_tx(
        &self,
        id: EmployeeId,
        name: &str,
        address: &str,
        hourly_rate: f32,
    ) -> Box<dyn Transaction> {
        trace!("mk_add_hourly_employee_tx called");
        Box::new(AddHourlyEmployeeTx::new(
            id,
            name,
            address,
            hourly_rate,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
    fn mk_add_salaried_employee_tx(
        &self,
        id: EmployeeId,
        name: &str,
        address: &str,
        salary: f32,
    ) -> Box<dyn Transaction> {
        trace!("mk_add_salaried_employee_tx called");
        Box::new(AddSalariedEmployeeTx::new(
            id,
            name,
            address,
            salary,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
    fn mk_add_commissioned_employee_tx(
        &self,
        id: EmployeeId,
        name: &str,
        address: &str,
        salary: f32,
        commission_rate: f32,
    ) -> Box<dyn Transaction> {
        trace!("mk_add_commissioned_employee_tx called");
        Box::new(AddCommissionedEmployeeTx::new(
            id,
            name,
            address,
            salary,
            commission_rate,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
    fn mk_delete_employee_tx(&self, id: EmployeeId) -> Box<dyn Transaction> {
        trace!("mk_delete_employee_tx called");
        Box::new(DeleteEmployeeTx::new(id, self.dao.clone()))
    }
    fn mk_add_timecard_tx(
        &self,
        id: EmployeeId,
        date: NaiveDate,
        hours: f32,
    ) -> Box<dyn Transaction> {
        trace!("mk_add_timecard_tx called");
        Box::new(AddTimeCardTx::new(id, date, hours, self.dao.clone()))
    }
    fn mk_add_sales_receipt_tx(
        &self,
        id: EmployeeId,
        date: NaiveDate,
        amount: f32,
    ) -> Box<dyn Transaction> {
        trace!("mk_add_sales_receipt_tx called");
        Box::new(AddSalesReceiptTx::new(id, date, amount, self.dao.clone()))
    }
    fn mk_add_service_charge_tx(
        &self,
        id: MemberId,
        date: NaiveDate,
        amount: f32,
    ) -> Box<dyn Transaction> {
        trace!("mk_add_service_charge_tx called");
        Box::new(AddServiceChargeTx::new(id, date, amount, self.dao.clone()))
    }
    fn mk_change_employee_name_tx(&self, id: EmployeeId, new_name: &str) -> Box<dyn Transaction> {
        trace!("mk_change_name_tx called");
        Box::new(ChangeEmployeeNameTx::new(id, new_name, self.dao.clone()))
    }
    fn mk_change_employee_address_tx(
        &self,
        id: EmployeeId,
        new_address: &str,
    ) -> Box<dyn Transaction> {
        trace!("mk_change_address_tx called");
        Box::new(ChangeEmployeeAddressTx::new(
            id,
            new_address,
            self.dao.clone(),
        ))
    }
    fn mk_change_employee_hourly_tx(
        &self,
        id: EmployeeId,
        hourly_rate: f32,
    ) -> Box<dyn Transaction> {
        trace!("mk_change_employee_hourly_tx called");
        Box::new(ChangeHourlyTx::new(
            id,
            hourly_rate,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
    fn mk_change_employee_salaried_tx(&self, id: EmployeeId, salary: f32) -> Box<dyn Transaction> {
        trace!("mk_change_employee_salaried_tx called");
        Box::new(ChangeSalariedTx::new(
            id,
            salary,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
    fn mk_change_employee_commissioned_tx(
        &self,
        id: EmployeeId,
        salary: f32,
        commission_rate: f32,
    ) -> Box<dyn Transaction> {
        trace!("mk_change_employee_commissioned_tx called");
        Box::new(ChangeCommissionedTx::new(
            id,
            salary,
            commission_rate,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
    fn mk_change_employee_hold_tx(&self, id: EmployeeId) -> Box<dyn Transaction> {
        trace!("mk_change_employee_hold_tx called");
        Box::new(ChangeHoldTx::new(
            id,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
    fn mk_change_employee_direct_tx(
        &self,
        id: EmployeeId,
        bank: &str,
        account: &str,
    ) -> Box<dyn Transaction> {
        trace!("mk_change_employee_direct_tx called");
        Box::new(ChangeDirectTx::new(
            id,
            bank,
            account,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
    fn mk_change_employee_mail_tx(&self, id: EmployeeId, address: &str) -> Box<dyn Transaction> {
        trace!("mk_change_employee_mail_tx called");
        Box::new(ChangeMailTx::new(
            id,
            address,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
    fn mk_change_employee_member_tx(
        &self,
        emp_id: EmployeeId,
        member_id: MemberId,
        dues: f32,
    ) -> Box<dyn Transaction> {
        trace!("mk_change_employee_member_tx called");
        Box::new(ChangeMemberTx::new(
            member_id,
            emp_id,
            dues,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
    fn mk_change_employee_no_member_tx(&self, id: EmployeeId) -> Box<dyn Transaction> {
        trace!("mk_change_employee_no_member_tx called");
        Box::new(ChangeNoMemberTx::new(
            id,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
    fn mk_payday_tx(&self, date: NaiveDate) -> Box<dyn Transaction> {
        trace!("mk_payday_tx called");
        Box::new(PaydayTx::new(date, self.dao.clone()))
    }
}
