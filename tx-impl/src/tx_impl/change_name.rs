use anyhow;
use log::trace;

use crate::ChangeEmployeeName;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::EmployeeId;
use tx_app::{Response, Transaction};

// ユースケース: ChangeEmployeeName トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeEmployeeNameTx<T>
where
    T: EmployeeDao,
{
    id: EmployeeId,
    new_name: String,
    dao: T,
}
impl<T> ChangeEmployeeNameTx<T>
where
    T: EmployeeDao,
{
    pub fn new(id: EmployeeId, new_name: &str, dao: T) -> Self {
        Self {
            id,
            new_name: new_name.to_string(),
            dao,
        }
    }
}

impl<T> HaveEmployeeDao for ChangeEmployeeNameTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> ChangeEmployeeName for ChangeEmployeeNameTx<T>
where
    T: EmployeeDao,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn get_new_name(&self) -> &str {
        &self.new_name
    }
}
// 共通インターフェースの実装
impl<T> Transaction for ChangeEmployeeNameTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("ChangeEmployeeNameTx::execute called");
        ChangeEmployeeName::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
