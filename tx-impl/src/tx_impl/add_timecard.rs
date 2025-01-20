use anyhow;
use chrono::NaiveDate;
use log::trace;

use abstract_tx::ChangeEmployee;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Employee, EmployeeId};
use payroll_impl::HourlyClassification;
use tx_app::{Response, Transaction};

// ユースケース: AddTimeCard トランザクションの実装 (struct)
#[derive(Debug)]
pub struct AddTimeCardTx<T>
where
    T: EmployeeDao,
{
    id: EmployeeId,
    date: NaiveDate,
    hours: f32,

    dao: T,
}
impl<T> AddTimeCardTx<T>
where
    T: EmployeeDao,
{
    pub fn new(id: EmployeeId, date: NaiveDate, hours: f32, dao: T) -> Self {
        Self {
            id,
            date,
            hours,
            dao,
        }
    }
}

impl<T> HaveEmployeeDao for AddTimeCardTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> ChangeEmployee for AddTimeCardTx<T>
where
    T: EmployeeDao,
{
    fn get_id(&self) -> EmployeeId {
        self.id
    }
    fn change(&self, emp: &mut Employee) -> Result<(), DaoError> {
        emp.classification()
            .borrow_mut()
            .as_any_mut()
            .downcast_mut::<HourlyClassification>()
            .ok_or(DaoError::UnexpectedError(
                "classification is not HourlyClassification".into(),
            ))?
            .add_timecard(self.date, self.hours);
        Ok(())
    }
}
// 共通インターフェースの実装
impl<T> Transaction for AddTimeCardTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("AddTimeCardTx::execute called");
        ChangeEmployee::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
