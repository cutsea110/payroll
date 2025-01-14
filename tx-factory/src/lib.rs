// TxFacotry の具体的な実装
use chrono::NaiveDate;
use log::trace;

// tx_app にのみ依存 (domain は当然 ok)
use payroll_domain::{EmployeeId, MemberId};
use tx_app::{Transaction, TxFactory};

pub struct TxFactoryImpl<'a> {
    pub add_salaried_employee: &'a dyn Fn(EmployeeId, &str, &str, f32) -> Box<dyn Transaction>,
    pub add_hourly_employee: &'a dyn Fn(EmployeeId, &str, &str, f32) -> Box<dyn Transaction>,
    pub add_commissioned_employee:
        &'a dyn Fn(EmployeeId, &str, &str, f32, f32) -> Box<dyn Transaction>,
    pub delete_employee: &'a dyn Fn(EmployeeId) -> Box<dyn Transaction>,
    pub add_timecard: &'a dyn Fn(EmployeeId, NaiveDate, f32) -> Box<dyn Transaction>,
    pub add_sales_receipt: &'a dyn Fn(EmployeeId, NaiveDate, f32) -> Box<dyn Transaction>,
    pub change_employee_name: &'a dyn Fn(EmployeeId, &str) -> Box<dyn Transaction>,
    pub change_employee_address: &'a dyn Fn(EmployeeId, &str) -> Box<dyn Transaction>,
    pub change_employee_salaried: &'a dyn Fn(EmployeeId, f32) -> Box<dyn Transaction>,
    pub change_employee_hourly: &'a dyn Fn(EmployeeId, f32) -> Box<dyn Transaction>,
    pub change_employee_commissioned: &'a dyn Fn(EmployeeId, f32, f32) -> Box<dyn Transaction>,
    pub change_method_hold: &'a dyn Fn(EmployeeId) -> Box<dyn Transaction>,
    pub change_method_direct: &'a dyn Fn(EmployeeId, &str, &str) -> Box<dyn Transaction>,
    pub change_method_mail: &'a dyn Fn(EmployeeId, &str) -> Box<dyn Transaction>,
    pub add_union_member: &'a dyn Fn(EmployeeId, MemberId, f32) -> Box<dyn Transaction>,
    pub delete_union_member: &'a dyn Fn(EmployeeId) -> Box<dyn Transaction>,
    pub add_service_charge: &'a dyn Fn(MemberId, NaiveDate, f32) -> Box<dyn Transaction>,
    pub payday: &'a dyn Fn(NaiveDate) -> Box<dyn Transaction>,
}
impl<'a> TxFactory for TxFactoryImpl<'a> {
    fn mk_add_salaried_employee_tx(
        &self,
        id: EmployeeId,
        name: &str,
        address: &str,
        salary: f32,
    ) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_add_salaried_employee_tx called");
        (self.add_salaried_employee)(id, name, address, salary)
    }
    fn mk_add_hourly_employee_tx(
        &self,
        id: EmployeeId,
        name: &str,
        address: &str,
        hourly_rate: f32,
    ) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_add_hourly_emp_tx called");
        (self.add_hourly_employee)(id, name, address, hourly_rate)
    }
    fn mk_add_commissioned_employee_tx(
        &self,
        id: EmployeeId,
        name: &str,
        address: &str,
        salary: f32,
        commission_rate: f32,
    ) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_add_commissioned_employee_tx called");
        (self.add_commissioned_employee)(id, name, address, salary, commission_rate)
    }
    fn mk_delete_employee_tx(&self, id: EmployeeId) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_delete_employee_tx called");
        (self.delete_employee)(id)
    }
    fn mk_add_timecard_tx(
        &self,
        id: EmployeeId,
        date: NaiveDate,
        hours: f32,
    ) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_add_timecard_tx called");
        (self.add_timecard)(id, date, hours)
    }
    fn mk_add_sales_receipt_tx(
        &self,
        id: EmployeeId,
        date: NaiveDate,
        amount: f32,
    ) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_add_sales_receipt_tx called");
        (self.add_sales_receipt)(id, date, amount)
    }
    fn mk_change_employee_name_tx(&self, id: EmployeeId, new_name: &str) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_change_employee_name_tx called");
        (self.change_employee_name)(id, new_name)
    }
    fn mk_change_employee_address_tx(
        &self,
        id: EmployeeId,
        new_address: &str,
    ) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_change_employee_address_tx called");
        (self.change_employee_address)(id, new_address)
    }
    fn mk_change_employee_salaried_tx(&self, id: EmployeeId, salary: f32) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_change_employee_salaried_tx called");
        (self.change_employee_salaried)(id, salary)
    }
    fn mk_change_employee_hourly_tx(
        &self,
        id: EmployeeId,
        hourly_rate: f32,
    ) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_change_employee_hourly_tx called");
        (self.change_employee_hourly)(id, hourly_rate)
    }
    fn mk_change_employee_commissioned_tx(
        &self,
        id: EmployeeId,
        salary: f32,
        commission_rate: f32,
    ) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_change_employee_commissioned_tx called");
        (self.change_employee_commissioned)(id, salary, commission_rate)
    }
    fn mk_change_method_hold_tx(&self, id: EmployeeId) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_change_method_hold_tx called");
        (self.change_method_hold)(id)
    }
    fn mk_change_method_direct_tx(
        &self,
        id: EmployeeId,
        bank: &str,
        account: &str,
    ) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_change_method_direct_tx called");
        (self.change_method_direct)(id, bank, account)
    }
    fn mk_change_method_mail_tx(&self, id: EmployeeId, address: &str) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_change_method_mail_tx called");
        (self.change_method_mail)(id, address)
    }
    fn mk_add_union_member_tx(
        &self,
        id: EmployeeId,
        member_id: MemberId,
        dues: f32,
    ) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_add_union_member_tx called");
        (self.add_union_member)(id, member_id, dues)
    }
    fn mk_delete_union_member_tx(&self, id: EmployeeId) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_delete_union_member_tx called");
        (self.delete_union_member)(id)
    }
    fn mk_add_service_charge_tx(
        &self,
        member_id: MemberId,
        date: NaiveDate,
        amount: f32,
    ) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_add_service_charge_tx called");
        (self.add_service_charge)(member_id, date, amount)
    }
    fn mk_payday_tx(&self, date: NaiveDate) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_payday_tx called");
        (self.payday)(date)
    }
}
