use thiserror::Error;
use tx_rs::Tx;

use payroll_domain::EmployeeId;
use usecase::{
    AddEmployee, AddSalesReceipt, AddServiceCharge, AddTimeCard, AddUnionAffiliation,
    ChgClassification, ChgEmployeeAddress, ChgEmployeeName, ChgMethod, DelEmployee,
    DelUnionAffiliation, Payday, UsecaseError,
};

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum ServiceError {
    #[error("failed to register employee: {0}")]
    FailedToRegisterEmployee(UsecaseError),
    #[error("failed to change employee: {0}")]
    FailedToChangeEmployee(UsecaseError),
    #[error("failed to delete employee: {0}")]
    FailedToDeleteEmployee(UsecaseError),
    #[error("failed to change classification: {0}")]
    FailedToChangeClassification(UsecaseError),
    #[error("failed to change method: {0}")]
    FailedToChangeMethod(UsecaseError),
    #[error("failed to register union member: {0}")]
    FailedToRegisterUnionMember(UsecaseError),
    #[error("failed to unregister union member: {0}")]
    FailedToUnregisterUnionMember(UsecaseError),
    #[error("failed to add time card: {0}")]
    FailedToAddTimeCard(UsecaseError),
    #[error("failed to add sales receipt: {0}")]
    FailedToAddSalesReceipt(UsecaseError),
    #[error("failed to add service charge: {0}")]
    FailedToAddServiceCharge(UsecaseError),
    #[error("failed to payday: {0}")]
    FailedToPayday(UsecaseError),
}

pub trait AddEmployeeTransaction<'a, Ctx>
where
    Ctx: 'a,
{
    type U: AddEmployee<Ctx>;

    fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
    where
        F: FnOnce(&mut Self::U, &mut Ctx) -> Result<T, UsecaseError>;

    fn execute(&'a mut self) -> Result<EmployeeId, ServiceError> {
        self.run_tx(move |usecase, ctx| usecase.execute().run(ctx))
    }
}
pub trait ChgEmployeeNameTransaction<'a, Ctx>
where
    Ctx: 'a,
{
    type U: ChgEmployeeName<Ctx>;

    fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
    where
        F: FnOnce(&mut Self::U, &mut Ctx) -> Result<T, UsecaseError>;

    fn execute(&'a mut self) -> Result<(), ServiceError> {
        self.run_tx(move |usecase, ctx| usecase.execute().run(ctx))
    }
}
pub trait ChgEmployeeAddressTransaction<'a, Ctx>
where
    Ctx: 'a,
{
    type U: ChgEmployeeAddress<Ctx>;

    fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
    where
        F: FnOnce(&mut Self::U, &mut Ctx) -> Result<T, UsecaseError>;

    fn execute(&'a mut self) -> Result<(), ServiceError> {
        self.run_tx(move |usecase, ctx| usecase.execute().run(ctx))
    }
}
pub trait ChgClassificationTransaction<'a, Ctx>
where
    Ctx: 'a,
{
    type U: ChgClassification<Ctx>;

    fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
    where
        F: FnOnce(&mut Self::U, &mut Ctx) -> Result<T, UsecaseError>;

    fn execute(&'a mut self) -> Result<(), ServiceError> {
        self.run_tx(move |usecase, ctx| usecase.execute().run(ctx))
    }
}
pub trait ChgMethodTransaction<'a, Ctx>
where
    Ctx: 'a,
{
    type U: ChgMethod<Ctx>;

    fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
    where
        F: FnOnce(&mut Self::U, &mut Ctx) -> Result<T, UsecaseError>;

    fn execute(&'a mut self) -> Result<(), ServiceError> {
        self.run_tx(move |usecase, ctx| usecase.execute().run(ctx))
    }
}
pub trait DelEmployeeTransaction<'a, Ctx>
where
    Ctx: 'a,
{
    type U: DelEmployee<Ctx>;

    fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
    where
        F: FnOnce(&mut Self::U, &mut Ctx) -> Result<T, UsecaseError>;

    fn execute(&'a mut self) -> Result<(), ServiceError> {
        self.run_tx(|usecase, ctx| usecase.execute().run(ctx))
    }
}
pub trait AddUnionAffiliationTransaction<'a, Ctx>
where
    Ctx: 'a,
{
    type U: AddUnionAffiliation<Ctx>;

    fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
    where
        F: FnOnce(&mut Self::U, &mut Ctx) -> Result<T, UsecaseError>;

    fn execute(&'a mut self) -> Result<(), ServiceError> {
        self.run_tx(|usecase, ctx| usecase.execute().run(ctx))
    }
}
pub trait DelUnionAffiliationTransaction<'a, Ctx>
where
    Ctx: 'a,
{
    type U: DelUnionAffiliation<Ctx>;

    fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
    where
        F: FnOnce(&mut Self::U, &mut Ctx) -> Result<T, UsecaseError>;

    fn execute(&'a mut self) -> Result<(), ServiceError> {
        self.run_tx(|usecase, ctx| usecase.execute().run(ctx))
    }
}
pub trait AddTimeCardTransaction<'a, Ctx>
where
    Ctx: 'a,
{
    type U: AddTimeCard<Ctx>;

    fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
    where
        F: FnOnce(&mut Self::U, &mut Ctx) -> Result<T, UsecaseError>;

    fn execute(&'a mut self) -> Result<(), ServiceError> {
        self.run_tx(|usecase, ctx| usecase.execute().run(ctx))
    }
}
pub trait AddSalesReceiptTransaction<'a, Ctx>
where
    Ctx: 'a,
{
    type U: AddSalesReceipt<Ctx>;

    fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
    where
        F: FnOnce(&mut Self::U, &mut Ctx) -> Result<T, UsecaseError>;

    fn execute(&'a mut self) -> Result<(), ServiceError> {
        self.run_tx(|usecase, ctx| usecase.execute().run(ctx))
    }
}
pub trait AddServiceChargeTransaction<'a, Ctx>
where
    Ctx: 'a,
{
    type U: AddServiceCharge<Ctx>;

    fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
    where
        F: FnOnce(&mut Self::U, &mut Ctx) -> Result<T, UsecaseError>;

    fn execute(&'a mut self) -> Result<(), ServiceError> {
        self.run_tx(|usecase, ctx| usecase.execute().run(ctx))
    }
}
pub trait PaydayTransaction<'a, Ctx>
where
    Ctx: 'a,
{
    type U: Payday<Ctx>;

    fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
    where
        F: FnOnce(&mut Self::U, &mut Ctx) -> Result<T, UsecaseError>;

    fn execute(&'a mut self) -> Result<(), ServiceError> {
        self.run_tx(|usecase, ctx| usecase.execute().run(ctx))
    }
}

pub trait Transaction {
    type T;
    fn execute(&mut self) -> Result<Self::T, ServiceError>;
}
