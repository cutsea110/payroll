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
    // なににも依存しない (domain は当然 ok)
    use payroll_domain::EmpId;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Tx {
        AddEmp(EmpId, String),
        ChgEmpName(EmpId, String),
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
                Tx::AddEmp(id, name) => {
                    trace!("convert Tx::AddEmp by mk_add_emp_tx called");
                    self.mk_add_emp_tx(id, &name)
                }
                Tx::ChgEmpName(id, new_name) => {
                    trace!("convert Tx::ChgEmpName by mk_chg_emp_name_tx called");
                    self.mk_chg_emp_name_tx(id, &new_name)
                }
            }
        }

        fn mk_add_emp_tx(&self, id: EmpId, name: &str) -> Box<dyn Transaction>;
        fn mk_chg_emp_name_tx(&self, id: EmpId, new_name: &str) -> Box<dyn Transaction>;
    }
}
pub use tx_factory::*;
