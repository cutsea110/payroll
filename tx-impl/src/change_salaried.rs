use anyhow;
use log::{debug, trace};

use abstract_tx::ChangeEmployee;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Employee, EmployeeId};
use payroll_factory::PayrollFactory;
use tx_app::{Response, Transaction};

// ユースケース: ChangeSalaried トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeSalariedTx<T, F>
where
    T: EmployeeDao,
{
    id: EmployeeId,
    salary: f32,

    dao: T,
    payroll_factory: F,
}
impl<T, F> ChangeSalariedTx<T, F>
where
    T: EmployeeDao,
{
    pub fn new(id: EmployeeId, salary: f32, dao: T, payroll_factory: F) -> Self {
        Self {
            id,
            salary,
            dao,
            payroll_factory,
        }
    }
}

impl<T, F> HaveEmployeeDao for ChangeSalariedTx<T, F>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T, F> ChangeEmployee for ChangeSalariedTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn change(&self, emp: &mut Employee) -> Result<(), DaoError> {
        trace!("change called");
        emp.set_classification(self.payroll_factory.mk_salaried_classification(self.salary));
        debug!("classification changed: {:?}", emp.classification());
        emp.set_schedule(self.payroll_factory.mk_monthly_schedule());
        debug!("schedule changed: {:?}", emp.schedule());
        Ok(())
    }
}
// 共通インターフェースの実装
impl<T, F> Transaction for ChangeSalariedTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("execute called");
        ChangeEmployee::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
