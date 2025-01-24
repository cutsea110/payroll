use thiserror::Error;

use dao::DaoError;

#[derive(Debug, Clone, Error)]
pub enum UsecaseError {
    #[error("add employee failed: {0}")]
    AddEmployeeFailed(DaoError),
    #[error("change employee failed: {0}")]
    ChangeEmployeeFailed(DaoError),
    #[error("delete employee failed: {0}")]
    DeleteEmployeeFailed(DaoError),
    #[error("change affiliation failed: {0}")]
    ChangeAffiliationFailed(DaoError),
    #[error("change member failed: {0}")]
    ChangeMemberFailed(DaoError),
    #[error("payday failed: {0}")]
    PaydayFailed(DaoError),
}
