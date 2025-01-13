// TxFacotry の具体的な実装
use log::trace;

// tx_app にのみ依存 (domain は当然 ok)
use payroll_domain::EmpId;
use tx_app::{Transaction, TxFactory};

pub struct TxFactoryImpl<'a> {
    pub add_emp: &'a dyn Fn(EmpId, &str, &str, f32) -> Box<dyn Transaction>,
    pub chg_emp_name: &'a dyn Fn(EmpId, &str) -> Box<dyn Transaction>,
}
impl<'a> TxFactory for TxFactoryImpl<'a> {
    fn mk_add_emp_tx(
        &self,
        id: EmpId,
        name: &str,
        address: &str,
        salary: f32,
    ) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_add_emp_tx called");
        (self.add_emp)(id, name, address, salary)
    }
    fn mk_chg_emp_name_tx(&self, id: EmpId, new_name: &str) -> Box<dyn Transaction> {
        trace!("TxFactoryImpl::mk_chg_emp_name_tx called");
        (self.chg_emp_name)(id, new_name)
    }
}
