// TxFacotry の具体的な実装
use chrono::NaiveDate;
use log::trace;

// tx_app にのみ依存 (domain は当然 ok)
use payroll_domain::{EmpId, MemberId};
use tx_app::{Transaction, TxFactory};

pub struct TxFactoryImpl<'a> {
    pub add_salaried_emp: &'a dyn Fn(EmpId, &str, &str, f32) -> Box<dyn Transaction>,
    pub add_hourly_emp: &'a dyn Fn(EmpId, &str, &str, f32) -> Box<dyn Transaction>,
    pub add_commissioned_emp: &'a dyn Fn(EmpId, &str, &str, f32, f32) -> Box<dyn Transaction>,
    pub del_emp: &'a dyn Fn(EmpId) -> Box<dyn Transaction>,
    pub timecard: &'a dyn Fn(EmpId, NaiveDate, f32) -> Box<dyn Transaction>,
    pub sales_receipt: &'a dyn Fn(EmpId, NaiveDate, f32) -> Box<dyn Transaction>,
    pub chg_emp_name: &'a dyn Fn(EmpId, &str) -> Box<dyn Transaction>,
    pub chg_emp_address: &'a dyn Fn(EmpId, &str) -> Box<dyn Transaction>,
    pub chg_salaried: &'a dyn Fn(EmpId, f32) -> Box<dyn Transaction>,
    pub chg_hourly: &'a dyn Fn(EmpId, f32) -> Box<dyn Transaction>,
    pub chg_commissioned: &'a dyn Fn(EmpId, f32, f32) -> Box<dyn Transaction>,
    pub chg_hold_method: &'a dyn Fn(EmpId) -> Box<dyn Transaction>,
    pub chg_direct_method: &'a dyn Fn(EmpId, &str, &str) -> Box<dyn Transaction>,
    pub chg_mail_method: &'a dyn Fn(EmpId, &str) -> Box<dyn Transaction>,
    pub add_union_member: &'a dyn Fn(EmpId, MemberId, f32) -> Box<dyn Transaction>,
    pub del_union_member: &'a dyn Fn(EmpId) -> Box<dyn Transaction>,
    pub service_charge: &'a dyn Fn(MemberId, NaiveDate, f32) -> Box<dyn Transaction>,
    pub payday: &'a dyn Fn(NaiveDate) -> Box<dyn Transaction>,
}
impl<'a> TxFactory for TxFactoryImpl<'a> {
    fn mk_add_salaried_emp_tx(
        &self,
        id: EmpId,
        name: &str,
        address: &str,
        salary: f32,
    ) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_add_emp_tx called");
        (self.add_salaried_emp)(id, name, address, salary)
    }
    fn mk_add_hourly_emp_tx(
        &self,
        id: EmpId,
        name: &str,
        address: &str,
        hourly_rate: f32,
    ) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_add_hourly_emp_tx called");
        (self.add_hourly_emp)(id, name, address, hourly_rate)
    }
    fn mk_add_commissioned_emp_tx(
        &self,
        id: EmpId,
        name: &str,
        address: &str,
        salary: f32,
        commission_rate: f32,
    ) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_add_commissioned_emp_tx called");
        (self.add_commissioned_emp)(id, name, address, salary, commission_rate)
    }
    fn mk_del_emp_tx(&self, id: EmpId) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_del_emp_tx called");
        (self.del_emp)(id)
    }
    fn mk_timecard_tx(&self, id: EmpId, date: NaiveDate, hours: f32) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_timecard_tx called");
        (self.timecard)(id, date, hours)
    }
    fn mk_sales_receipt_tx(&self, id: EmpId, date: NaiveDate, amount: f32) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_sales_receipt_tx called");
        (self.sales_receipt)(id, date, amount)
    }
    fn mk_chg_emp_name_tx(&self, id: EmpId, new_name: &str) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_chg_emp_name_tx called");
        (self.chg_emp_name)(id, new_name)
    }
    fn mk_chg_emp_address_tx(&self, id: EmpId, new_address: &str) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_chg_emp_address_tx called");
        (self.chg_emp_address)(id, new_address)
    }
    fn mk_chg_salaried_tx(&self, id: EmpId, salary: f32) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_chg_salaried_tx called");
        (self.chg_salaried)(id, salary)
    }
    fn mk_chg_hourly_tx(&self, id: EmpId, hourly_rate: f32) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_chg_hourly_tx called");
        (self.chg_hourly)(id, hourly_rate)
    }
    fn mk_chg_commissioned_tx(
        &self,
        id: EmpId,
        salary: f32,
        commission_rate: f32,
    ) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_chg_commissioned_tx called");
        (self.chg_commissioned)(id, salary, commission_rate)
    }
    fn mk_chg_hold_method_tx(&self, id: EmpId) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_chg_hold_method_tx called");
        (self.chg_hold_method)(id)
    }
    fn mk_chg_direct_method_tx(
        &self,
        id: EmpId,
        bank: &str,
        account: &str,
    ) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_chg_direct_method_tx called");
        (self.chg_direct_method)(id, bank, account)
    }
    fn mk_chg_mail_method_tx(&self, id: EmpId, address: &str) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_chg_mail_method_tx called");
        (self.chg_mail_method)(id, address)
    }
    fn mk_chg_union_member_tx(
        &self,
        id: EmpId,
        member_id: MemberId,
        dues: f32,
    ) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_chg_union_member_tx called");
        (self.add_union_member)(id, member_id, dues)
    }
    fn mk_chg_unaffiliated_tx(&self, id: EmpId) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_chg_unaffiliated_tx called");
        (self.del_union_member)(id)
    }
    fn mk_service_charge_tx(
        &self,
        member_id: MemberId,
        date: NaiveDate,
        amount: f32,
    ) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_service_charge_tx called");
        (self.service_charge)(member_id, date, amount)
    }
    fn mk_payday_tx(&self, date: NaiveDate) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_payday_tx called");
        (self.payday)(date)
    }
}
