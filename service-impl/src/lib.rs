mod add_salaried_emp {
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use payroll_db::{PayrollDatabase, PayrollDbCtx};
    use payroll_domain::EmployeeId;
    use service::{AddEmployeeTransaction, ServiceError, Transaction};
    use usecase::UsecaseError;
    use usecase_impl::AddSalariedEmployeeImpl;

    #[derive(Debug, Clone)]
    pub struct AddSalariedEmployeeTx {
        db: Rc<RefCell<PayrollDatabase>>,
        usecase: RefCell<AddSalariedEmployeeImpl>,
    }
    impl AddSalariedEmployeeTx {
        pub fn new(
            id: EmployeeId,
            name: &str,
            address: &str,
            salary: f32,
            db: Rc<RefCell<PayrollDatabase>>,
        ) -> Self {
            Self {
                db,
                usecase: RefCell::new(AddSalariedEmployeeImpl::new(id, name, address, salary)),
            }
        }
    }

    impl<'a> AddEmployeeTransaction<'a, PayrollDbCtx<'a>> for AddSalariedEmployeeTx {
        type U = AddSalariedEmployeeImpl;

        fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
        where
            F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
        {
            let mut tx = self.db.borrow_mut();
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx).map_err(ServiceError::FailedToRegisterEmployee)
        }
    }

    impl Transaction for AddSalariedEmployeeTx {
        type T = EmployeeId;
        fn execute(&mut self) -> Result<EmployeeId, ServiceError> {
            AddEmployeeTransaction::execute(self)
        }
    }
}
pub use add_salaried_emp::*;

mod add_hourly_emp {
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use payroll_db::{PayrollDatabase, PayrollDbCtx};
    use payroll_domain::EmployeeId;
    use service::{AddEmployeeTransaction, ServiceError, Transaction};
    use usecase::UsecaseError;
    use usecase_impl::AddHourlyEmployeeImpl;

    #[derive(Debug, Clone)]
    pub struct AddHourlyEmployeeTx {
        db: Rc<RefCell<PayrollDatabase>>,
        usecase: RefCell<AddHourlyEmployeeImpl>,
    }
    impl AddHourlyEmployeeTx {
        pub fn new(
            id: EmployeeId,
            name: &str,
            address: &str,
            hourly_rate: f32,
            db: Rc<RefCell<PayrollDatabase>>,
        ) -> Self {
            Self {
                db,
                usecase: RefCell::new(AddHourlyEmployeeImpl::new(id, name, address, hourly_rate)),
            }
        }
    }

    impl<'a> AddEmployeeTransaction<'a, PayrollDbCtx<'a>> for AddHourlyEmployeeTx {
        type U = AddHourlyEmployeeImpl;

        fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
        where
            F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
        {
            let mut tx = self.db.borrow_mut();
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx).map_err(ServiceError::FailedToRegisterEmployee)
        }
    }

    impl Transaction for AddHourlyEmployeeTx {
        type T = EmployeeId;
        fn execute(&mut self) -> Result<EmployeeId, ServiceError> {
            AddEmployeeTransaction::execute(self)
        }
    }
}
pub use add_hourly_emp::*;

mod add_commissioned_emp {
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use payroll_db::{PayrollDatabase, PayrollDbCtx};
    use payroll_domain::EmployeeId;
    use service::{AddEmployeeTransaction, ServiceError, Transaction};
    use usecase::UsecaseError;
    use usecase_impl::AddCommissionedEmployeeImpl;

    #[derive(Debug, Clone)]
    pub struct AddCommissionedEmployeeTx {
        db: Rc<RefCell<PayrollDatabase>>,
        usecase: RefCell<AddCommissionedEmployeeImpl>,
    }
    impl AddCommissionedEmployeeTx {
        pub fn new(
            id: EmployeeId,
            name: &str,
            address: &str,
            salary: f32,
            commission_rate: f32,
            db: Rc<RefCell<PayrollDatabase>>,
        ) -> Self {
            Self {
                db,
                usecase: RefCell::new(AddCommissionedEmployeeImpl::new(
                    id,
                    name,
                    address,
                    salary,
                    commission_rate,
                )),
            }
        }
    }

    impl<'a> AddEmployeeTransaction<'a, PayrollDbCtx<'a>> for AddCommissionedEmployeeTx {
        type U = AddCommissionedEmployeeImpl;

        fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
        where
            F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
        {
            let mut tx = self.db.borrow_mut();
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx).map_err(ServiceError::FailedToRegisterEmployee)
        }
    }

    impl Transaction for AddCommissionedEmployeeTx {
        type T = EmployeeId;
        fn execute(&mut self) -> Result<EmployeeId, ServiceError> {
            AddEmployeeTransaction::execute(self)
        }
    }
}
pub use add_commissioned_emp::*;

mod chg_emp_name {
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use payroll_db::{PayrollDatabase, PayrollDbCtx};
    use payroll_domain::EmployeeId;
    use service::{ChgEmployeeNameTransaction, ServiceError, Transaction};
    use usecase::UsecaseError;
    use usecase_impl::ChgEmployeeNameImpl;

    #[derive(Debug, Clone)]
    pub struct ChgEmployeeNameTx {
        db: Rc<RefCell<PayrollDatabase>>,
        usecase: RefCell<ChgEmployeeNameImpl>,
    }
    impl ChgEmployeeNameTx {
        pub fn new(id: EmployeeId, new_name: &str, db: Rc<RefCell<PayrollDatabase>>) -> Self {
            Self {
                db,
                usecase: RefCell::new(ChgEmployeeNameImpl::new(id, new_name)),
            }
        }
    }

    impl<'a> ChgEmployeeNameTransaction<'a, PayrollDbCtx<'a>> for ChgEmployeeNameTx {
        type U = ChgEmployeeNameImpl;

        fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
        where
            F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
        {
            let mut tx = self.db.borrow_mut();
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx).map_err(ServiceError::FailedToChangeEmployee)
        }
    }

    impl Transaction for ChgEmployeeNameTx {
        type T = ();
        fn execute(&mut self) -> Result<(), ServiceError> {
            ChgEmployeeNameTransaction::execute(self)
        }
    }
}
pub use chg_emp_name::*;

mod chg_emp_address {
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use payroll_db::{PayrollDatabase, PayrollDbCtx};
    use payroll_domain::EmployeeId;
    use service::{ChgEmployeeAddressTransaction, ServiceError, Transaction};
    use usecase::UsecaseError;
    use usecase_impl::ChgEmployeeAddressImpl;

    #[derive(Debug, Clone)]
    pub struct ChgEmployeeAddressTx {
        db: Rc<RefCell<PayrollDatabase>>,
        usecase: RefCell<ChgEmployeeAddressImpl>,
    }
    impl ChgEmployeeAddressTx {
        pub fn new(id: EmployeeId, new_name: &str, db: Rc<RefCell<PayrollDatabase>>) -> Self {
            Self {
                db,
                usecase: RefCell::new(ChgEmployeeAddressImpl::new(id, new_name)),
            }
        }
    }

    impl<'a> ChgEmployeeAddressTransaction<'a, PayrollDbCtx<'a>> for ChgEmployeeAddressTx {
        type U = ChgEmployeeAddressImpl;

        fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
        where
            F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
        {
            let mut tx = self.db.borrow_mut();
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx).map_err(ServiceError::FailedToChangeEmployee)
        }
    }

    impl Transaction for ChgEmployeeAddressTx {
        type T = ();
        fn execute(&mut self) -> Result<(), ServiceError> {
            ChgEmployeeAddressTransaction::execute(self)
        }
    }
}
pub use chg_emp_address::*;

mod del_emp {
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use payroll_db::{PayrollDatabase, PayrollDbCtx};
    use payroll_domain::EmployeeId;
    use service::{DelEmployeeTransaction, ServiceError, Transaction};
    use usecase::UsecaseError;
    use usecase_impl::DelEmployeeImpl;

    #[derive(Debug, Clone)]
    pub struct DelEmployeeTx {
        db: Rc<RefCell<PayrollDatabase>>,
        usecase: RefCell<DelEmployeeImpl>,
    }
    impl DelEmployeeTx {
        pub fn new(id: EmployeeId, db: Rc<RefCell<PayrollDatabase>>) -> Self {
            Self {
                db,
                usecase: RefCell::new(DelEmployeeImpl::new(id)),
            }
        }
    }

    impl<'a> DelEmployeeTransaction<'a, PayrollDbCtx<'a>> for DelEmployeeTx {
        type U = DelEmployeeImpl;

        fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
        where
            F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
        {
            let mut tx = self.db.borrow_mut();
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx).map_err(ServiceError::FailedToDeleteEmployee)
        }
    }

    impl Transaction for DelEmployeeTx {
        type T = ();
        fn execute(&mut self) -> Result<(), ServiceError> {
            DelEmployeeTransaction::execute(self)
        }
    }
}
pub use del_emp::*;

mod chg_salaried_emp {
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use payroll_db::{PayrollDatabase, PayrollDbCtx};
    use payroll_domain::EmployeeId;
    use service::{ChgClassificationTransaction, ServiceError, Transaction};
    use usecase::UsecaseError;
    use usecase_impl::ChgSalariedEmployeeImpl;

    #[derive(Debug, Clone)]
    pub struct ChgSalariedClassificationTx {
        db: Rc<RefCell<PayrollDatabase>>,
        usecase: RefCell<ChgSalariedEmployeeImpl>,
    }
    impl ChgSalariedClassificationTx {
        pub fn new(id: EmployeeId, salary: f32, db: Rc<RefCell<PayrollDatabase>>) -> Self {
            Self {
                db,
                usecase: RefCell::new(ChgSalariedEmployeeImpl::new(id, salary)),
            }
        }
    }

    impl<'a> ChgClassificationTransaction<'a, PayrollDbCtx<'a>> for ChgSalariedClassificationTx {
        type U = ChgSalariedEmployeeImpl;

        fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
        where
            F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
        {
            let mut tx = self.db.borrow_mut();
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx).map_err(ServiceError::FailedToChangeClassification)
        }
    }

    impl Transaction for ChgSalariedClassificationTx {
        type T = ();
        fn execute(&mut self) -> Result<(), ServiceError> {
            ChgClassificationTransaction::execute(self)
        }
    }
}
pub use chg_salaried_emp::*;

mod chg_hourly_emp {
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use payroll_db::{PayrollDatabase, PayrollDbCtx};
    use payroll_domain::EmployeeId;
    use service::{ChgClassificationTransaction, ServiceError, Transaction};
    use usecase::UsecaseError;
    use usecase_impl::ChgHourlyEmployeeImpl;

    #[derive(Debug, Clone)]
    pub struct ChgHourlyClassificationTx {
        db: Rc<RefCell<PayrollDatabase>>,
        usecase: RefCell<ChgHourlyEmployeeImpl>,
    }
    impl ChgHourlyClassificationTx {
        pub fn new(id: EmployeeId, hourly_rate: f32, db: Rc<RefCell<PayrollDatabase>>) -> Self {
            Self {
                db,
                usecase: RefCell::new(ChgHourlyEmployeeImpl::new(id, hourly_rate)),
            }
        }
    }

    impl<'a> ChgClassificationTransaction<'a, PayrollDbCtx<'a>> for ChgHourlyClassificationTx {
        type U = ChgHourlyEmployeeImpl;

        fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
        where
            F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
        {
            let mut tx = self.db.borrow_mut();
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx).map_err(ServiceError::FailedToChangeClassification)
        }
    }

    impl Transaction for ChgHourlyClassificationTx {
        type T = ();
        fn execute(&mut self) -> Result<(), ServiceError> {
            ChgClassificationTransaction::execute(self)
        }
    }
}
pub use chg_hourly_emp::*;

mod chg_commissioned_emp {
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use payroll_db::{PayrollDatabase, PayrollDbCtx};
    use payroll_domain::EmployeeId;
    use service::{ChgClassificationTransaction, ServiceError, Transaction};
    use usecase::UsecaseError;
    use usecase_impl::ChgCommissionedEmployeeImpl;

    #[derive(Debug, Clone)]
    pub struct ChgCommissionedClassificationTx {
        db: Rc<RefCell<PayrollDatabase>>,
        usecase: RefCell<ChgCommissionedEmployeeImpl>,
    }
    impl ChgCommissionedClassificationTx {
        pub fn new(
            id: EmployeeId,
            salary: f32,
            commission_rate: f32,
            db: Rc<RefCell<PayrollDatabase>>,
        ) -> Self {
            Self {
                db,
                usecase: RefCell::new(ChgCommissionedEmployeeImpl::new(
                    id,
                    salary,
                    commission_rate,
                )),
            }
        }
    }

    impl<'a> ChgClassificationTransaction<'a, PayrollDbCtx<'a>> for ChgCommissionedClassificationTx {
        type U = ChgCommissionedEmployeeImpl;

        fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
        where
            F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
        {
            let mut tx = self.db.borrow_mut();
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx).map_err(ServiceError::FailedToChangeClassification)
        }
    }

    impl Transaction for ChgCommissionedClassificationTx {
        type T = ();
        fn execute(&mut self) -> Result<(), ServiceError> {
            ChgClassificationTransaction::execute(self)
        }
    }
}
pub use chg_commissioned_emp::*;

mod chg_hold_method {
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use payroll_db::{PayrollDatabase, PayrollDbCtx};
    use payroll_domain::EmployeeId;
    use service::{ChgMethodTransaction, ServiceError, Transaction};
    use usecase::UsecaseError;
    use usecase_impl::ChgHoldMethodImpl;

    #[derive(Debug, Clone)]
    pub struct ChgHoldMethodTx {
        db: Rc<RefCell<PayrollDatabase>>,
        usecase: RefCell<ChgHoldMethodImpl>,
    }
    impl ChgHoldMethodTx {
        pub fn new(id: EmployeeId, db: Rc<RefCell<PayrollDatabase>>) -> Self {
            Self {
                db,
                usecase: RefCell::new(ChgHoldMethodImpl::new(id)),
            }
        }
    }

    impl<'a> ChgMethodTransaction<'a, PayrollDbCtx<'a>> for ChgHoldMethodTx {
        type U = ChgHoldMethodImpl;

        fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
        where
            F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
        {
            let mut tx = self.db.borrow_mut();
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx).map_err(ServiceError::FailedToChangeMethod)
        }
    }

    impl Transaction for ChgHoldMethodTx {
        type T = ();
        fn execute(&mut self) -> Result<(), ServiceError> {
            ChgMethodTransaction::execute(self)
        }
    }
}
pub use chg_hold_method::*;

mod chg_direct_method {
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use payroll_db::{PayrollDatabase, PayrollDbCtx};
    use payroll_domain::EmployeeId;
    use service::{ChgMethodTransaction, ServiceError, Transaction};
    use usecase::UsecaseError;
    use usecase_impl::ChgDirectMethodImpl;

    #[derive(Debug, Clone)]
    pub struct ChgDirectMethodTx {
        db: Rc<RefCell<PayrollDatabase>>,
        usecase: RefCell<ChgDirectMethodImpl>,
    }
    impl ChgDirectMethodTx {
        pub fn new(
            id: EmployeeId,
            bank: &str,
            account: &str,
            db: Rc<RefCell<PayrollDatabase>>,
        ) -> Self {
            Self {
                db,
                usecase: RefCell::new(ChgDirectMethodImpl::new(id, bank, account)),
            }
        }
    }

    impl<'a> ChgMethodTransaction<'a, PayrollDbCtx<'a>> for ChgDirectMethodTx {
        type U = ChgDirectMethodImpl;

        fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
        where
            F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
        {
            let mut tx = self.db.borrow_mut();
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx).map_err(ServiceError::FailedToChangeMethod)
        }
    }

    impl Transaction for ChgDirectMethodTx {
        type T = ();
        fn execute(&mut self) -> Result<(), ServiceError> {
            ChgMethodTransaction::execute(self)
        }
    }
}
pub use chg_direct_method::*;

mod chg_mail_method {
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use payroll_db::{PayrollDatabase, PayrollDbCtx};
    use payroll_domain::EmployeeId;
    use service::{ChgMethodTransaction, ServiceError, Transaction};
    use usecase::UsecaseError;
    use usecase_impl::ChgMailMethodImpl;

    #[derive(Debug, Clone)]
    pub struct ChgMailMethodTx {
        db: Rc<RefCell<PayrollDatabase>>,
        usecase: RefCell<ChgMailMethodImpl>,
    }
    impl ChgMailMethodTx {
        pub fn new(id: EmployeeId, address: &str, db: Rc<RefCell<PayrollDatabase>>) -> Self {
            Self {
                db,
                usecase: RefCell::new(ChgMailMethodImpl::new(id, address)),
            }
        }
    }

    impl<'a> ChgMethodTransaction<'a, PayrollDbCtx<'a>> for ChgMailMethodTx {
        type U = ChgMailMethodImpl;

        fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
        where
            F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
        {
            let mut tx = self.db.borrow_mut();
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx).map_err(ServiceError::FailedToChangeMethod)
        }
    }

    impl Transaction for ChgMailMethodTx {
        type T = ();
        fn execute(&mut self) -> Result<(), ServiceError> {
            ChgMethodTransaction::execute(self)
        }
    }
}
pub use chg_mail_method::*;

mod add_union_member {
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use payroll_db::{PayrollDatabase, PayrollDbCtx};
    use payroll_domain::EmployeeId;
    use service::{AddUnionAffiliationTransaction, ServiceError, Transaction};
    use usecase::UsecaseError;
    use usecase_impl::AddUnionMemberImpl;

    #[derive(Debug, Clone)]
    pub struct AddUnionMemberTx {
        db: Rc<RefCell<PayrollDatabase>>,
        usecase: RefCell<AddUnionMemberImpl>,
    }
    impl AddUnionMemberTx {
        pub fn new(
            member_id: EmployeeId,
            emp_id: EmployeeId,
            dues: f32,
            db: Rc<RefCell<PayrollDatabase>>,
        ) -> Self {
            Self {
                db,
                usecase: RefCell::new(AddUnionMemberImpl::new(member_id, emp_id, dues)),
            }
        }
    }

    impl<'a> AddUnionAffiliationTransaction<'a, PayrollDbCtx<'a>> for AddUnionMemberTx {
        type U = AddUnionMemberImpl;

        fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
        where
            F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
        {
            let mut tx = self.db.borrow_mut();
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx).map_err(ServiceError::FailedToRegisterUnionMember)
        }
    }

    impl Transaction for AddUnionMemberTx {
        type T = ();
        fn execute(&mut self) -> Result<(), ServiceError> {
            AddUnionAffiliationTransaction::execute(self)
        }
    }
}
pub use add_union_member::*;

mod del_union_member {
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use payroll_db::{PayrollDatabase, PayrollDbCtx};
    use payroll_domain::EmployeeId;
    use service::{DelUnionAffiliationTransaction, ServiceError, Transaction};
    use usecase::UsecaseError;
    use usecase_impl::DelUnionMemberImpl;

    #[derive(Debug, Clone)]
    pub struct DelUnionMemberTx {
        db: Rc<RefCell<PayrollDatabase>>,
        usecase: RefCell<DelUnionMemberImpl>,
    }
    impl DelUnionMemberTx {
        pub fn new(member_id: EmployeeId, db: Rc<RefCell<PayrollDatabase>>) -> Self {
            Self {
                db,
                usecase: RefCell::new(DelUnionMemberImpl::new(member_id)),
            }
        }
    }

    impl<'a> DelUnionAffiliationTransaction<'a, PayrollDbCtx<'a>> for DelUnionMemberTx {
        type U = DelUnionMemberImpl;

        fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
        where
            F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
        {
            let mut tx = self.db.borrow_mut();
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx).map_err(ServiceError::FailedToUnregisterUnionMember)
        }
    }

    impl Transaction for DelUnionMemberTx {
        type T = ();
        fn execute(&mut self) -> Result<(), ServiceError> {
            DelUnionAffiliationTransaction::execute(self)
        }
    }
}
pub use del_union_member::*;

mod add_timecard {
    use chrono::NaiveDate;
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use payroll_db::{PayrollDatabase, PayrollDbCtx};
    use payroll_domain::EmployeeId;
    use service::{AddTimeCardTransaction, ServiceError, Transaction};
    use usecase::UsecaseError;
    use usecase_impl::AddTimecardImpl;

    #[derive(Debug, Clone)]
    pub struct AddTimecardTx {
        db: Rc<RefCell<PayrollDatabase>>,
        usecase: RefCell<AddTimecardImpl>,
    }
    impl AddTimecardTx {
        pub fn new(
            emp_id: EmployeeId,
            date: NaiveDate,
            hours: f32,
            db: Rc<RefCell<PayrollDatabase>>,
        ) -> Self {
            Self {
                db,
                usecase: RefCell::new(AddTimecardImpl::new(emp_id, date, hours)),
            }
        }
    }

    impl<'a> AddTimeCardTransaction<'a, PayrollDbCtx<'a>> for AddTimecardTx {
        type U = AddTimecardImpl;

        fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
        where
            F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
        {
            let mut tx = self.db.borrow_mut();
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx).map_err(ServiceError::FailedToAddTimeCard)
        }
    }

    impl Transaction for AddTimecardTx {
        type T = ();
        fn execute(&mut self) -> Result<(), ServiceError> {
            AddTimeCardTransaction::execute(self)
        }
    }
}
pub use add_timecard::*;

mod add_sales_receipt {
    use chrono::NaiveDate;
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use payroll_db::{PayrollDatabase, PayrollDbCtx};
    use payroll_domain::EmployeeId;
    use service::{AddSalesReceiptTransaction, ServiceError, Transaction};
    use usecase::UsecaseError;
    use usecase_impl::AddSalesReceiptImpl;

    #[derive(Debug, Clone)]
    pub struct AddSalesReceiptTx {
        db: Rc<RefCell<PayrollDatabase>>,
        usecase: RefCell<AddSalesReceiptImpl>,
    }
    impl AddSalesReceiptTx {
        pub fn new(
            emp_id: EmployeeId,
            date: NaiveDate,
            amount: f32,
            db: Rc<RefCell<PayrollDatabase>>,
        ) -> Self {
            Self {
                db,
                usecase: RefCell::new(AddSalesReceiptImpl::new(emp_id, date, amount)),
            }
        }
    }

    impl<'a> AddSalesReceiptTransaction<'a, PayrollDbCtx<'a>> for AddSalesReceiptTx {
        type U = AddSalesReceiptImpl;

        fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
        where
            F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
        {
            let mut tx = self.db.borrow_mut();
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx).map_err(ServiceError::FailedToAddSalesReceipt)
        }
    }
    impl Transaction for AddSalesReceiptTx {
        type T = ();
        fn execute(&mut self) -> Result<(), ServiceError> {
            AddSalesReceiptTransaction::execute(self)
        }
    }
}
pub use add_sales_receipt::*;

mod add_service_charge {
    use chrono::NaiveDate;
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use payroll_db::{PayrollDatabase, PayrollDbCtx};
    use payroll_domain::MemberId;
    use service::{AddServiceChargeTransaction, ServiceError, Transaction};
    use usecase::UsecaseError;
    use usecase_impl::AddServiceChargeImpl;

    #[derive(Debug, Clone)]
    pub struct AddServiceChargeTx {
        db: Rc<RefCell<PayrollDatabase>>,
        usecase: RefCell<AddServiceChargeImpl>,
    }
    impl AddServiceChargeTx {
        pub fn new(
            member_id: MemberId,
            date: NaiveDate,
            amount: f32,
            db: Rc<RefCell<PayrollDatabase>>,
        ) -> Self {
            Self {
                db,
                usecase: RefCell::new(AddServiceChargeImpl::new(member_id, date, amount)),
            }
        }
    }

    impl<'a> AddServiceChargeTransaction<'a, PayrollDbCtx<'a>> for AddServiceChargeTx {
        type U = AddServiceChargeImpl;

        fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
        where
            F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
        {
            let mut tx = self.db.borrow_mut();
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx).map_err(ServiceError::FailedToAddServiceCharge)
        }
    }
    impl Transaction for AddServiceChargeTx {
        type T = ();
        fn execute(&mut self) -> Result<(), ServiceError> {
            AddServiceChargeTransaction::execute(self)
        }
    }
}
pub use add_service_charge::*;

mod payday {
    use chrono::NaiveDate;
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use payroll_db::{PayrollDatabase, PayrollDbCtx};
    use service::{PaydayTransaction, ServiceError, Transaction};
    use usecase::UsecaseError;
    use usecase_impl::PaydayImpl;

    #[derive(Debug, Clone)]
    pub struct PaydayTx {
        db: Rc<RefCell<PayrollDatabase>>,
        usecase: RefCell<PaydayImpl>,
    }
    impl PaydayTx {
        pub fn new(pay_date: NaiveDate, db: Rc<RefCell<PayrollDatabase>>) -> Self {
            Self {
                db,
                usecase: RefCell::new(PaydayImpl::new(pay_date)),
            }
        }
    }

    impl<'a> PaydayTransaction<'a, PayrollDbCtx<'a>> for PaydayTx {
        type U = PaydayImpl;

        fn run_tx<T, F>(&'a self, f: F) -> Result<T, ServiceError>
        where
            F: FnOnce(&mut Self::U, &mut PayrollDbCtx<'a>) -> Result<T, UsecaseError>,
        {
            let mut tx = self.db.borrow_mut();
            let mut usecase = self.usecase.borrow_mut();
            f(&mut usecase, &mut tx).map_err(ServiceError::FailedToPayday)
        }
    }
    impl Transaction for PaydayTx {
        type T = ();
        fn execute(&mut self) -> Result<(), ServiceError> {
            PaydayTransaction::execute(self)
        }
    }
}
pub use payday::*;
