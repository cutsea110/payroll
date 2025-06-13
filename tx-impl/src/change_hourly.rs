use anyhow;
use log::{debug, trace};

use abstract_tx::ChangeEmployee;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Employee, EmployeeId};
use payroll_factory::PayrollFactory;
use tx_app::{Response, Transaction};

// ユースケース: ChangeHourly トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeHourlyTx<T, F>
where
    T: EmployeeDao,
{
    id: EmployeeId,
    hourly_rate: f32,

    dao: T,
    payroll_factory: F,
}
impl<T, F> ChangeHourlyTx<T, F>
where
    T: EmployeeDao,
{
    pub fn new(id: EmployeeId, hourly_rate: f32, dao: T, payroll_factory: F) -> Self {
        Self {
            id,
            hourly_rate,
            dao,
            payroll_factory,
        }
    }
}

impl<T, F> HaveEmployeeDao for ChangeHourlyTx<T, F>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T, F> ChangeEmployee for ChangeHourlyTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn change(&self, emp: &mut Employee) -> Result<(), DaoError> {
        trace!("change called");
        emp.set_classification(
            self.payroll_factory
                .mk_hourly_classification(self.hourly_rate),
        );
        debug!("classification changed: {:?}", emp.classification());
        emp.set_schedule(self.payroll_factory.mk_weekly_schedule());
        debug!("schedule changed: {:?}", emp.schedule());
        Ok(())
    }
}
// 共通インターフェースの実装
impl<T, F> Transaction for ChangeHourlyTx<T, F>
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
