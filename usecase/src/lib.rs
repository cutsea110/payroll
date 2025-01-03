use chrono::NaiveDate;
use std::{cell::RefCell, rc::Rc};
use thiserror::Error;
use tx_rs::Tx;

use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
use payroll_domain::{
    Affiliation, Employee, EmployeeId, MemberId, NoAffiliation, Paycheck, PaymentClassification,
    PaymentMethod, PaymentSchedule,
};
use payroll_impl::{
    CommissionedClassification, HoldMethod, HourlyClassification, UnionAffiliation,
};

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum UsecaseError {
    #[error("failed to get: {0}")]
    FailedToGet(DaoError),
    #[error("failed to add: {0}")]
    FailedToAdd(DaoError),
    #[error("failed to update: {0}")]
    FailedToUpdate(DaoError),
    #[error("failed to delete: {0}")]
    FailedToDelete(DaoError),
    #[error("failed to get union member: {0}")]
    FailedToGetUnionMember(DaoError),
    #[error("failed to add union member: {0}")]
    FailedToAddUnionMember(DaoError),
    #[error("failed to delete union member: {0}")]
    FailedToDeleteUnionMember(DaoError),
    #[error("unexpected: {0}")]
    Unexpected(String),
}

pub trait AddEmployee<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_name(&self) -> &str;
    fn get_address(&self) -> &str;
    fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>>;
    fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>>;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = UsecaseError>
    where
        Ctx: 'a,
    {
        self.dao()
            .insert(Employee::new(
                self.get_emp_id(),
                self.get_name(),
                self.get_address(),
                self.get_classification(),
                self.get_schedule(),
                Rc::new(RefCell::new(HoldMethod)),
                Rc::new(RefCell::new(NoAffiliation)),
            ))
            .map_err(UsecaseError::FailedToAdd)
    }
}
pub trait ChgEmployeeName<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_name(&self) -> &str;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        tx_rs::with_tx(move |ctx| {
            let emp_id = self.get_emp_id();
            let mut emp = self
                .dao()
                .fetch(emp_id)
                .map_err(UsecaseError::FailedToGet)
                .run(ctx)?;
            emp.set_name(self.get_name());
            self.dao()
                .update(emp)
                .map_err(UsecaseError::FailedToUpdate)
                .run(ctx)
        })
    }
}
pub trait ChgEmployeeAddress<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_address(&self) -> &str;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        tx_rs::with_tx(move |ctx| {
            let emp_id = self.get_emp_id();
            let mut emp = self
                .dao()
                .fetch(emp_id)
                .map_err(UsecaseError::FailedToGet)
                .run(ctx)?;
            emp.set_address(self.get_address());
            self.dao()
                .update(emp)
                .map_err(UsecaseError::FailedToUpdate)
                .run(ctx)
        })
    }
}
pub trait ChgClassification<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>>;
    fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>>;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        tx_rs::with_tx(move |ctx| {
            let emp_id = self.get_emp_id();
            let mut emp = self
                .dao()
                .fetch(emp_id)
                .map_err(UsecaseError::FailedToGet)
                .run(ctx)?;
            emp.set_classification(self.get_classification());
            emp.set_schedule(self.get_schedule());
            self.dao()
                .update(emp)
                .map_err(UsecaseError::FailedToUpdate)
                .run(ctx)
        })
    }
}
pub trait ChgMethod<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>>;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        tx_rs::with_tx(move |ctx| {
            let emp_id = self.get_emp_id();
            let mut emp = self
                .dao()
                .fetch(emp_id)
                .map_err(UsecaseError::FailedToGet)
                .run(ctx)?;
            emp.set_method(self.get_method());
            self.dao()
                .update(emp)
                .map_err(UsecaseError::FailedToUpdate)
                .run(ctx)
        })
    }
}
pub trait DelEmployee<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        self.dao()
            .remove(self.get_emp_id())
            .map_err(UsecaseError::FailedToDelete)
    }
}
pub trait AddUnionAffiliation<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_member_id(&self) -> MemberId;
    fn get_emp_id(&self) -> EmployeeId;
    fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>>;

    fn record_membership(&self, ctx: &mut Ctx) -> Result<(), UsecaseError> {
        self.dao()
            .add_union_member(self.get_member_id(), self.get_emp_id())
            .run(ctx)
            .map_err(UsecaseError::FailedToAddUnionMember)
    }

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        tx_rs::with_tx(move |ctx| {
            self.record_membership(ctx)?;

            let emp_id = self.get_emp_id();
            let mut emp = self
                .dao()
                .fetch(emp_id)
                .map_err(UsecaseError::FailedToGet)
                .run(ctx)?;
            emp.set_affiliation(self.get_affiliation());
            self.dao()
                .update(emp)
                .map_err(UsecaseError::FailedToUpdate)
                .run(ctx)
        })
    }
}
pub trait DelUnionAffiliation<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>>;

    fn record_membership(&self, ctx: &mut Ctx) -> Result<(), UsecaseError> {
        let emp = self
            .dao()
            .fetch(self.get_emp_id())
            .run(ctx)
            .map_err(UsecaseError::FailedToGet)?;
        let member_id = emp
            .get_affiliation()
            .borrow()
            .as_any()
            .downcast_ref::<UnionAffiliation>()
            .map_or(
                Err(UsecaseError::Unexpected("didn't union affiliation".into())),
                |a| Ok(a.get_member_id()),
            )?;
        self.dao()
            .remove_union_member(member_id)
            .run(ctx)
            .map_err(UsecaseError::FailedToDeleteUnionMember)
    }
    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        tx_rs::with_tx(move |ctx| {
            self.record_membership(ctx)?;

            let emp_id = self.get_emp_id();
            let mut emp = self
                .dao()
                .fetch(emp_id)
                .map_err(UsecaseError::FailedToGet)
                .run(ctx)?;
            emp.set_affiliation(self.get_affiliation());
            self.dao()
                .update(emp)
                .map_err(UsecaseError::FailedToUpdate)
                .run(ctx)
        })
    }
}
pub trait AddTimeCard<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_date(&self) -> NaiveDate;
    fn get_hours(&self) -> f32;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        tx_rs::with_tx(move |ctx| {
            let emp = self
                .dao()
                .fetch(self.get_emp_id())
                .run(ctx)
                .map_err(UsecaseError::FailedToGet)?;
            emp.get_classification()
                .borrow_mut()
                .as_any_mut()
                .downcast_mut::<HourlyClassification>()
                .ok_or(UsecaseError::Unexpected(
                    "didn't hourly classification".into(),
                ))?
                .add_timecard(self.get_date(), self.get_hours());
            self.dao()
                .update(emp)
                .run(ctx)
                .map_err(UsecaseError::FailedToUpdate)
        })
    }
}
pub trait AddSalesReceipt<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_emp_id(&self) -> EmployeeId;
    fn get_date(&self) -> NaiveDate;
    fn get_amount(&self) -> f32;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        tx_rs::with_tx(move |ctx| {
            let emp = self
                .dao()
                .fetch(self.get_emp_id())
                .run(ctx)
                .map_err(UsecaseError::FailedToGet)?;
            emp.get_classification()
                .borrow_mut()
                .as_any_mut()
                .downcast_mut::<CommissionedClassification>()
                .ok_or(UsecaseError::Unexpected(
                    "didn't commissioned classification".into(),
                ))?
                .add_sales_receipt(self.get_date(), self.get_amount());
            self.dao()
                .update(emp)
                .run(ctx)
                .map_err(UsecaseError::FailedToUpdate)
        })
    }
}
pub trait AddServiceCharge<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_member_id(&self) -> MemberId;
    fn get_date(&self) -> NaiveDate;
    fn get_amount(&self) -> f32;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        tx_rs::with_tx(move |ctx| {
            let emp_id = self
                .dao()
                .find_union_member(self.get_member_id())
                .run(ctx)
                .map_err(UsecaseError::FailedToGetUnionMember)?;
            let emp = self
                .dao()
                .fetch(emp_id)
                .run(ctx)
                .map_err(UsecaseError::FailedToGet)?;
            emp.get_affiliation()
                .borrow_mut()
                .as_any_mut()
                .downcast_mut::<UnionAffiliation>()
                .ok_or(UsecaseError::Unexpected("didn't union affiliation".into()))?
                .add_service_charge(self.get_date(), self.get_amount());
            self.dao()
                .update(emp)
                .run(ctx)
                .map_err(UsecaseError::FailedToUpdate)
        })
    }
}
pub trait Payday<Ctx>: HaveEmployeeDao<Ctx> {
    fn get_pay_date(&self) -> NaiveDate;

    fn execute<'a>(&'a self) -> impl tx_rs::Tx<Ctx, Item = (), Err = UsecaseError>
    where
        Ctx: 'a,
    {
        tx_rs::with_tx(move |ctx| {
            let mut employees = self
                .dao()
                .fetch_all()
                .run(ctx)
                .map_err(UsecaseError::FailedToGet)?;
            for (emp_id, emp) in employees.iter_mut() {
                if emp.is_pay_date(self.get_pay_date()) {
                    let period = emp.get_pay_period(self.get_pay_date());
                    let mut pc = Paycheck::new(period);
                    emp.payday(&mut pc);
                    self.dao()
                        .record_paycheck(*emp_id, pc)
                        .run(ctx)
                        .map_err(UsecaseError::FailedToUpdate)?;
                }
            }
            Ok(())
        })
    }
}
