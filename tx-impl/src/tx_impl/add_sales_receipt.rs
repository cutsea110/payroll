use anyhow;
use chrono::NaiveDate;
use log::trace;
use payroll_impl::CommissionedClassification;

use crate::ChangeEmployee;
use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Employee, EmployeeId};
use tx_app::{Response, Transaction};

// ユースケース: AddSalesReceipt トランザクションの実装 (struct)
#[derive(Debug)]
pub struct AddSalesReceiptTx<T>
where
    T: EmployeeDao,
{
    id: EmployeeId,
    date: NaiveDate,
    amount: f32,

    dao: T,
}
impl<T> AddSalesReceiptTx<T>
where
    T: EmployeeDao,
{
    pub fn new(id: EmployeeId, date: NaiveDate, amount: f32, dao: T) -> Self {
        Self {
            id,
            date,
            amount,
            dao,
        }
    }
}

impl<T> HaveEmployeeDao for AddSalesReceiptTx<T>
where
    T: EmployeeDao,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T> ChangeEmployee for AddSalesReceiptTx<T>
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
            .downcast_mut::<CommissionedClassification>()
            .ok_or(DaoError::UnexpectedError(
                "classification is not CommissionedClassification".into(),
            ))?
            .add_sales_receipt(self.date, self.amount);
        Ok(())
    }
}
// 共通インターフェースの実装
impl<T> Transaction for AddSalesReceiptTx<T>
where
    T: EmployeeDao,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("AddSalesReceiptTx::execute called");
        ChangeEmployee::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
