use thiserror::Error;

use dao::DaoError;

#[derive(Debug, Clone, Error)]
pub enum UsecaseError {
    #[error("add employee failed: {0}")]
    AddEmployeeFailed(DaoError),
    #[error("employee retrieval failed: {0}")]
    EmployeeRetrievalFailed(DaoError),
    #[error("delete employee failed: {0}")]
    DeleteEmployeeFailed(DaoError),
    #[error("add timecard failed: {0}")]
    AddTimeCardFailed(DaoError),
    #[error("add sales receipt failed: {0}")]
    AddSalesReceiptFailed(DaoError),
    #[error("add service charge failed: {0}")]
    AddServiceChargeFailed(DaoError),
    #[error("change employee name failed: {0}")]
    ChangeEmployeeNameFailed(DaoError),
    #[error("change employee address failed: {0}")]
    ChangeEmployeeAddressFailed(DaoError),
    #[error("change employee payment classification failed: {0}")]
    ChangePaymentClassificationFailed(DaoError),
    #[error("change employee payment method failed: {0}")]
    ChangePaymentMethodFailed(DaoError),
    #[error("change member failed: {0}")]
    ChangeMemberFailed(DaoError),
    #[error("change no member failed: {0}")]
    ChangeNoMemberFailed(DaoError),
    #[error("payday failed: {0}")]
    PaydayFailed(DaoError),
    #[error("unexpected error: {0}")]
    UnexpectedError(String),
}
