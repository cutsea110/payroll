use anyhow;
use log::trace;

use crate::ChangeEmployee;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Employee, EmployeeId};
use payroll_factory::PayrollFactory;
use tx_app::{Response, Transaction};

// ユースケース: ChangeCommissioned トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeCommissionedTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    id: EmployeeId,
    salary: f32,
    commission_rate: f32,

    dao: T,
    payroll_factory: F,
}
impl<T, F> ChangeCommissionedTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    pub fn new(
        id: EmployeeId,
        salary: f32,
        commission_rate: f32,
        dao: T,
        payroll_factory: F,
    ) -> Self {
        Self {
            id,
            salary,
            commission_rate,
            dao,
            payroll_factory,
        }
    }
}

impl<T, F> HaveEmployeeDao for ChangeCommissionedTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T, F> ChangeEmployee for ChangeCommissionedTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn change(&self, emp: &mut Employee) -> Result<(), DaoError> {
        emp.set_classification(
            self.payroll_factory
                .mk_commissioned_classification(self.salary, self.commission_rate),
        );
        emp.set_schedule(self.payroll_factory.mk_biweekly_schedule());
        Ok(())
    }
}
// 共通インターフェースの実装
impl<T, F> Transaction for ChangeCommissionedTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("ChangeCommissionedTx::execute called");
        ChangeEmployee::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
