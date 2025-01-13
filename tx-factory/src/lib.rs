// TxFacotry の具体的な実装
use log::trace;

// tx_app にのみ依存 (domain は当然 ok)
use payroll_domain::EmpId;
use tx_app::{Transaction, TxFactory};

pub struct TxFactoryImpl<'a> {
    pub add_salaried_emp: &'a dyn Fn(EmpId, &str, &str, f32) -> Box<dyn Transaction>,
    pub add_hourly_emp: &'a dyn Fn(EmpId, &str, &str, f32) -> Box<dyn Transaction>,
    pub add_commissioned_emp: &'a dyn Fn(EmpId, &str, &str, f32, f32) -> Box<dyn Transaction>,
    pub chg_emp_name: &'a dyn Fn(EmpId, &str) -> Box<dyn Transaction>,
    pub chg_emp_address: &'a dyn Fn(EmpId, &str) -> Box<dyn Transaction>,
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
    fn mk_chg_emp_name_tx(&self, id: EmpId, new_name: &str) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_chg_emp_name_tx called");
        (self.chg_emp_name)(id, new_name)
    }
    fn mk_chg_emp_address_tx(&self, id: EmpId, new_address: &str) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_chg_emp_address_tx called");
        (self.chg_emp_address)(id, new_address)
    }
}
