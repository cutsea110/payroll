use anyhow;
use log::trace;

use crate::ChangeEmployee;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Employee, EmployeeId};
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
impl<T> ChangeEmployee for ChangeEmployeeNameTx<T>
where
    T: EmployeeDao,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn change(&self, emp: &mut Employee) -> Result<(), DaoError> {
        emp.set_name(&self.new_name);
        Ok(())
    }
}
// 共通インターフェースの実装
impl<T> Transaction for ChangeEmployeeNameTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("ChangeEmployeeNameTx::execute called");
        ChangeEmployee::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
