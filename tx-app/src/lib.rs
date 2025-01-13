use log::trace;

pub struct TxApp<S, F>
where
    S: TxSource,
    F: TxFactory,
{
    tx_source: S,
    tx_factory: F,
}
impl<S, F> TxApp<S, F>
where
    S: TxSource,
    F: TxFactory,
{
    pub fn new(tx_source: S, tx_factory: F) -> Self {
        Self {
            tx_source,
            tx_factory,
        }
    }
    pub fn run(&self) -> Result<(), anyhow::Error> {
        trace!("TxApp::run called");
        while let Some(tx_src) = self.tx_source.get_tx_source() {
            trace!("get tx_source={:?}", tx_src);
            let tx = self.tx_factory.convert(tx_src);
            tx.execute()?;
        }
        Ok(())
    }
}

mod tx {
    use anyhow;
    // なににも依存しない (domain は当然 ok)
    use payroll_domain::EmpId;

    // トランザクションのインターフェース
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Response {
        Void,
        EmpId(EmpId),
    }
    pub trait Transaction {
        fn execute(&self) -> Result<Response, anyhow::Error>;
    }
}
pub use tx::*;

mod tx_source {
    use chrono::NaiveDate;

    // なににも依存しない (domain は当然 ok)
    use payroll_domain::{EmpId, MemberId};

    #[derive(Debug, Clone, PartialEq)]
    pub enum Tx {
        AddSalariedEmp(EmpId, String, String, f32),
        AddHourlyEmp(EmpId, String, String, f32),
        AddCommissionedEmp(EmpId, String, String, f32, f32),
        DelEmp(EmpId),
        TimeCard(EmpId, NaiveDate, f32),
        SalesReceipt(EmpId, NaiveDate, f32),
        ServiceCharge(EmpId, NaiveDate, f32),
        ChgEmpName(EmpId, String),
        ChgEmpAddress(EmpId, String),
        ChgSalaried(EmpId, f32),
        ChgHourly(EmpId, f32),
        ChgCommissioned(EmpId, f32, f32),
        ChgHold(EmpId),
        ChgDirect(EmpId, String, String),
        ChgMail(EmpId, String),
        ChgMember(EmpId, MemberId, f32),
        ChgNoMember(EmpId),
        Payday(NaiveDate),
    }
    pub trait TxSource {
        fn get_tx_source(&self) -> Option<Tx>;
    }
}
pub use tx_source::*;

mod tx_factory {
    use log::trace;

    // なににも依存しない (domain は当然 ok)
    use super::{Transaction, Tx};
    use payroll_domain::EmpId;

    pub trait TxFactory {
        fn convert(&self, src: Tx) -> Box<dyn Transaction> {
            trace!("TxFactory::convert called");
            match src {
                Tx::AddSalariedEmp(id, name, address, salary) => {
                    trace!("convert Tx::AddSalariedEmp by mk_add_salaried_emp_tx called");
                    self.mk_add_salaried_emp_tx(id, &name, &address, salary)
                }
                Tx::AddHourlyEmp(id, name, address, hourly_rate) => {
                    trace!("convert Tx::AddHourlyEmp by mk_add_hourly_emp_tx called");
                    self.mk_add_hourly_emp_tx(id, &name, &address, hourly_rate)
                }
                Tx::AddCommissionedEmp(id, name, address, salary, commission_rate) => {
                    trace!("convert Tx::AddCommissionedEmp by mk_add_commissioned_emp_tx called");
                    self.mk_add_commissioned_emp_tx(id, &name, &address, salary, commission_rate)
                }
                Tx::ChgEmpName(id, new_name) => {
                    trace!("convert Tx::ChgEmpName by mk_chg_emp_name_tx called");
                    self.mk_chg_emp_name_tx(id, &new_name)
                }
                Tx::DelEmp(id) => {
                    trace!("convert Tx::DelEmp by mk_del_emp_tx called");
                    self.mk_del_emp_tx(id)
                }
                Tx::ChgEmpAddress(id, new_address) => {
                    trace!("convert Tx::ChgEmpAddress by mk_chg_emp_address_tx called");
                    self.mk_chg_emp_address_tx(id, &new_address)
                }
                Tx::ChgSalaried(id, salary) => {
                    trace!("convert Tx::ChgSalaried by mk_chg_salaried_tx called");
                    self.mk_chg_salaried_tx(id, salary)
                }
                Tx::ChgHourly(id, hourly_rate) => {
                    trace!("convert Tx::ChgHourly by mk_chg_hourly_tx called");
                    self.mk_chg_hourly_tx(id, hourly_rate)
                }
                Tx::ChgCommissioned(id, salary, commission_rate) => {
                    trace!("convert Tx::ChgCommissioned by mk_chg_commissioned_tx called");
                    self.mk_chg_commissioned_tx(id, salary, commission_rate)
                }
                Tx::ChgHold(id) => {
                    trace!("convert Tx::ChgHoldMethod by mk_chg_hold_method_tx called");
                    self.mk_chg_hold_method_tx(id)
                }
                Tx::ChgDirect(id, bank, account) => {
                    trace!("convert Tx::ChgDirectMethod by mk_chg_direct_method_tx called");
                    self.mk_chg_direct_method_tx(id, &bank, &account)
                }
                Tx::ChgMail(id, address) => {
                    trace!("convert Tx::ChgMailMethod by mk_chg_mail_method_tx called");
                    self.mk_chg_mail_method_tx(id, &address)
                }
                _ => unimplemented!(),
            }
        }

        fn mk_add_salaried_emp_tx(
            &self,
            id: EmpId,
            name: &str,
            address: &str,
            salary: f32,
        ) -> Box<dyn Transaction>;
        fn mk_add_hourly_emp_tx(
            &self,
            id: EmpId,
            name: &str,
            address: &str,
            hourly_rate: f32,
        ) -> Box<dyn Transaction>;
        fn mk_add_commissioned_emp_tx(
            &self,
            id: EmpId,
            name: &str,
            address: &str,
            salary: f32,
            commission_rate: f32,
        ) -> Box<dyn Transaction>;
        fn mk_del_emp_tx(&self, id: EmpId) -> Box<dyn Transaction>;
        fn mk_chg_emp_name_tx(&self, id: EmpId, new_name: &str) -> Box<dyn Transaction>;
        fn mk_chg_emp_address_tx(&self, id: EmpId, new_address: &str) -> Box<dyn Transaction>;
        fn mk_chg_salaried_tx(&self, id: EmpId, salary: f32) -> Box<dyn Transaction>;
        fn mk_chg_hourly_tx(&self, id: EmpId, hourly_rate: f32) -> Box<dyn Transaction>;
        fn mk_chg_commissioned_tx(
            &self,
            id: EmpId,
            salary: f32,
            commission_rate: f32,
        ) -> Box<dyn Transaction>;
        fn mk_chg_hold_method_tx(&self, id: EmpId) -> Box<dyn Transaction>;
        fn mk_chg_direct_method_tx(
            &self,
            id: EmpId,
            bank: &str,
            account: &str,
        ) -> Box<dyn Transaction>;
        fn mk_chg_mail_method_tx(&self, id: EmpId, address: &str) -> Box<dyn Transaction>;
    }
}
pub use tx_factory::*;
