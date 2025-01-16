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

mod add_emp {
    use log::{debug, trace};
    use std::{cell::RefCell, rc::Rc};
    use tx_rs::Tx;

    use super::UsecaseError;
    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_domain::{
        Affiliation, Employee, EmployeeId, PaymentClassification, PaymentMethod, PaymentSchedule,
    };

    // ユースケース: AddEmployee トランザクション(抽象レベルのビジネスロジック)
    pub trait AddEmployee: HaveEmployeeDao {
        fn get_id(&self) -> EmployeeId;
        fn get_name(&self) -> &str;
        fn get_address(&self) -> &str;
        fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>>;
        fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>>;
        fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>>;
        fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>>;

        fn execute<'a>(&self) -> Result<(), UsecaseError> {
            trace!("AddEmployee::execute called");
            self.dao()
                .run_tx(|mut ctx| {
                    trace!("AddEmployee::run_tx called");
                    let emp = Employee::new(
                        self.get_id(),
                        self.get_name(),
                        self.get_address(),
                        self.get_classification(),
                        self.get_schedule(),
                        self.get_method(),
                        self.get_affiliation(),
                    );
                    debug!("AddEmployee::execute: emp={:?}", emp);
                    self.dao().insert(emp).run(&mut ctx)
                })
                .map(|_| ())
                .map_err(UsecaseError::AddEmployeeFailed)
        }
    }
}
pub use add_emp::*;

mod del_emp {
    use log::{debug, trace};
    use tx_rs::Tx;

    use super::UsecaseError;
    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_domain::EmployeeId;

    // ユースケース: DeleteEmployee トランザクション(抽象レベルのビジネスロジック)
    pub trait DeleteEmployee: HaveEmployeeDao {
        fn get_id(&self) -> EmployeeId;

        fn execute<'a>(&self) -> Result<(), UsecaseError> {
            trace!("DeleteEmployee::execute called");
            self.dao()
                .run_tx(|mut ctx| {
                    trace!("DeleteEmployee::run_tx called");
                    let emp_id = self.get_id();
                    debug!("DeleteEmployee::execute: emp_id={}", emp_id);
                    self.dao().remove(emp_id).run(&mut ctx)
                })
                .map(|_| ())
                .map_err(UsecaseError::DeleteEmployeeFailed)
        }
    }
}
pub use del_emp::*;

mod add_timecard {
    use chrono::NaiveDate;
    use log::trace;
    use tx_rs::Tx;

    use super::UsecaseError;
    use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
    use payroll_domain::EmployeeId;
    use payroll_impl::HourlyClassification;

    // ユースケース: AddTimeCard トランザクション(抽象レベルのビジネスロジック)
    pub trait AddTimeCard: HaveEmployeeDao {
        fn get_id(&self) -> EmployeeId;
        fn get_date(&self) -> NaiveDate;
        fn get_hours(&self) -> f32;

        fn execute<'a>(&self) -> Result<(), UsecaseError> {
            trace!("AddTimeCard::execute called");
            self.dao()
                .run_tx(|mut ctx| {
                    trace!("AddTimeCard::run_tx called");
                    let emp = self.dao().fetch(self.get_id()).run(&mut ctx)?;
                    emp.classification()
                        .borrow_mut()
                        .as_any_mut()
                        .downcast_mut::<HourlyClassification>()
                        .ok_or(DaoError::UnexpectedError(
                            "classification is not HourlyClassification".into(),
                        ))?
                        .add_timecard(self.get_date(), self.get_hours());
                    self.dao().update(emp).run(&mut ctx)
                })
                .map_err(UsecaseError::AddTimeCardFailed)
        }
    }
}
pub use add_timecard::*;

mod add_sales_receipt {
    use chrono::NaiveDate;
    use log::trace;
    use tx_rs::Tx;

    use super::UsecaseError;
    use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
    use payroll_domain::EmployeeId;
    use payroll_impl::CommissionedClassification;

    // ユースケース: AddSalesReceipt トランザクション(抽象レベルのビジネスロジック)
    pub trait AddSalesReceipt: HaveEmployeeDao {
        fn get_id(&self) -> EmployeeId;
        fn get_date(&self) -> NaiveDate;
        fn get_amount(&self) -> f32;

        fn execute<'a>(&self) -> Result<(), UsecaseError> {
            trace!("AddSalesReceipt::execute called");
            self.dao()
                .run_tx(|mut ctx| {
                    trace!("AddSalesReceipt::run_tx called");
                    let emp = self.dao().fetch(self.get_id()).run(&mut ctx)?;
                    emp.classification()
                        .borrow_mut()
                        .as_any_mut()
                        .downcast_mut::<CommissionedClassification>()
                        .ok_or(DaoError::UnexpectedError(
                            "classification is not CommissionedClassification".into(),
                        ))?
                        .add_sales_receipt(self.get_date(), self.get_amount());
                    self.dao().update(emp).run(&mut ctx)
                })
                .map_err(UsecaseError::AddSalesReceiptFailed)
        }
    }
}
pub use add_sales_receipt::*;

mod add_service_charge {
    use chrono::NaiveDate;
    use log::trace;
    use tx_rs::Tx;

    use super::UsecaseError;
    use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
    use payroll_domain::MemberId;
    use payroll_impl::UnionAffiliation;

    // ユースケース: AddServiceCharge トランザクション(抽象レベルのビジネスロジック)
    pub trait AddServiceCharge: HaveEmployeeDao {
        fn get_member_id(&self) -> MemberId;
        fn get_date(&self) -> NaiveDate;
        fn get_amount(&self) -> f32;

        fn execute<'a>(&self) -> Result<(), UsecaseError> {
            trace!("AddServiceCharge::execute called");
            self.dao()
                .run_tx(|mut ctx| {
                    trace!("AddServiceCharge::run_tx called");
                    let emp_id = self
                        .dao()
                        .find_union_member(self.get_member_id())
                        .run(&mut ctx)?;
                    let emp = self.dao().fetch(emp_id).run(&mut ctx)?;
                    emp.affiliation()
                        .borrow_mut()
                        .as_any_mut()
                        .downcast_mut::<UnionAffiliation>()
                        .ok_or(DaoError::UnexpectedError("didn't union affiliation".into()))?
                        .add_service_charge(self.get_date(), self.get_amount());
                    self.dao().update(emp).run(&mut ctx)
                })
                .map_err(UsecaseError::AddServiceChargeFailed)
        }
    }
}
pub use add_service_charge::*;

mod chg_name {
    use log::{debug, trace};
    use tx_rs::Tx;

    use super::UsecaseError;
    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_domain::EmployeeId;

    // ユースケース: ChangeEmployeeName トランザクション(抽象レベルのビジネスロジック)
    pub trait ChangeEmployeeName: HaveEmployeeDao {
        fn get_id(&self) -> EmployeeId;
        fn get_new_name(&self) -> &str;
        fn execute<'a>(&self) -> Result<(), UsecaseError> {
            trace!("ChangeEmployeeName::execute called");
            self.dao()
                .run_tx(|mut ctx| {
                    trace!("ChangeEmployeeName::run_tx called");
                    let mut emp = self.dao().fetch(self.get_id()).run(&mut ctx)?;
                    debug!(
                        r#"changing emp name: "{}" -> "{}""#,
                        emp.name(),
                        self.get_new_name()
                    );
                    emp.set_name(self.get_new_name());
                    debug!(r#"changed emp name="{}""#, emp.name());
                    self.dao().update(emp).run(&mut ctx)
                })
                .map_err(UsecaseError::ChangeEmployeeNameFailed)
        }
    }
}
pub use chg_name::*;

mod chg_address {
    use log::{debug, trace};
    use tx_rs::Tx;

    use super::UsecaseError;
    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_domain::EmployeeId;

    // ユースケース: ChangeEmployeeAddress トランザクション(抽象レベルのビジネスロジック)
    pub trait ChangeEmployeeAddress: HaveEmployeeDao {
        fn get_id(&self) -> EmployeeId;
        fn get_new_address(&self) -> &str;
        fn execute<'a>(&self) -> Result<(), UsecaseError> {
            trace!("ChangeEmployeeAddress::execute called");
            self.dao()
                .run_tx(|mut ctx| {
                    trace!("ChangeEmployeeAddress::run_tx called");
                    let mut emp = self.dao().fetch(self.get_id()).run(&mut ctx)?;
                    debug!(
                        r#"changing emp address: "{}" -> "{}""#,
                        emp.address(),
                        self.get_new_address()
                    );
                    emp.set_address(self.get_new_address());
                    debug!(r#"changed emp address="{}""#, emp.address());
                    self.dao().update(emp).run(&mut ctx)
                })
                .map_err(UsecaseError::ChangeEmployeeAddressFailed)
        }
    }
}
pub use chg_address::*;

mod chg_classification {
    use log::{debug, trace};
    use std::{cell::RefCell, rc::Rc};
    use tx_rs::Tx;

    use super::UsecaseError;
    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_domain::{EmployeeId, PaymentClassification, PaymentSchedule};

    // ユースケース: ChangeClassification トランザクション(抽象レベルのビジネスロジック)
    pub trait ChangeClassification: HaveEmployeeDao {
        fn get_id(&self) -> EmployeeId;
        fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>>;
        fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>>;
        fn execute<'a>(&self) -> Result<(), UsecaseError> {
            trace!("ChangeClassification::execute called");
            self.dao()
                .run_tx(|mut ctx| {
                    trace!("ChangeClassification::run_tx called");
                    let mut emp = self.dao().fetch(self.get_id()).run(&mut ctx)?;
                    debug!(
                        "changing emp classification: {:?} -> {:?}",
                        emp.classification(),
                        self.get_classification()
                    );
                    emp.set_classification(self.get_classification());
                    debug!("changed emp classification={:?}", emp.classification());
                    debug!(
                        "changing emp schedule: {:?} -> {:?}",
                        emp.schedule(),
                        self.get_schedule()
                    );
                    emp.set_schedule(self.get_schedule());
                    debug!("changed emp schedule={:?}", emp.schedule());
                    self.dao().update(emp).run(&mut ctx)
                })
                .map_err(UsecaseError::ChangePaymentClassificationFailed)
        }
    }
}
pub use chg_classification::*;

mod chg_method {
    use log::{debug, trace};
    use std::{cell::RefCell, rc::Rc};
    use tx_rs::Tx;

    use super::UsecaseError;
    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_domain::{EmployeeId, PaymentMethod};

    // ユースケース: ChgMethod トランザクション(抽象レベルのビジネスロジック)
    pub trait ChgMethod: HaveEmployeeDao {
        fn get_id(&self) -> EmployeeId;
        fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>>;
        fn execute<'a>(&self) -> Result<(), UsecaseError> {
            trace!("ChgMethod::execute called");
            self.dao()
                .run_tx(|mut ctx| {
                    trace!("ChgMethod::run_tx called");
                    let mut emp = self.dao().fetch(self.get_id()).run(&mut ctx)?;
                    debug!(
                        "changing emp method: {:?} -> {:?}",
                        emp.method(),
                        self.get_method()
                    );
                    emp.set_method(self.get_method());
                    debug!("changed emp method={:?}", emp.method());
                    self.dao().update(emp).run(&mut ctx)
                })
                .map_err(UsecaseError::ChangePaymentMethodFailed)
        }
    }
}
pub use chg_method::*;

mod chg_member {
    use log::{debug, trace};
    use std::{cell::RefCell, rc::Rc};
    use tx_rs::Tx;

    use super::UsecaseError;
    use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
    use payroll_domain::{Affiliation, EmployeeId, MemberId};

    // ユースケース: ChangeMember トランザクション(抽象レベルのビジネスロジック)
    pub trait ChangeMember: HaveEmployeeDao {
        fn get_member_id(&self) -> MemberId;
        fn get_emp_id(&self) -> EmployeeId;
        fn get_dues(&self) -> f32;
        fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>>;

        fn record_membership<'a>(&self, ctx: &mut Self::Ctx<'a>) -> Result<(), DaoError> {
            trace!("record_membership called");
            self.dao()
                .add_union_member(self.get_member_id(), self.get_emp_id())
                .run(ctx)
        }

        fn execute<'a>(&self) -> Result<(), UsecaseError> {
            trace!("ChangeMember::execute called");
            self.dao()
                .run_tx(|mut ctx| {
                    trace!("ChangeMember::run_tx called");
                    self.record_membership(&mut ctx)?;

                    let mut emp = self.dao().fetch(self.get_emp_id()).run(&mut ctx)?;
                    debug!(
                        "changing emp member: {:?} -> {:?}",
                        emp.affiliation(),
                        self.get_affiliation()
                    );
                    emp.set_affiliation(self.get_affiliation());
                    debug!("changed emp member={:?}", emp.affiliation());
                    self.dao().update(emp).run(&mut ctx)
                })
                .map_err(UsecaseError::ChangeMemberFailed)
        }
    }
}
pub use chg_member::*;

mod chg_no_member {
    use log::{debug, trace};
    use std::{cell::RefCell, rc::Rc};
    use tx_rs::Tx;

    use super::UsecaseError;
    use dao::{DaoError, EmployeeDao, HaveEmployeeDao};
    use payroll_domain::{EmployeeId, NoAffiliation};
    use payroll_impl::UnionAffiliation;

    // ユースケース: ChangeNoMember トランザクション(抽象レベルのビジネスロジック)
    pub trait ChangeNoMember: HaveEmployeeDao {
        fn get_emp_id(&self) -> EmployeeId;

        fn record_membership<'a>(&self, ctx: &mut Self::Ctx<'a>) -> Result<(), DaoError> {
            trace!("record_membership called");
            let emp = self.dao().fetch(self.get_emp_id()).run(ctx)?;
            let member_id = emp
                .affiliation()
                .borrow()
                .as_any()
                .downcast_ref::<UnionAffiliation>()
                .ok_or(DaoError::UnexpectedError("didn't union affiliation".into()))?
                .member_id();

            self.dao().remove_union_member(member_id).run(ctx)
        }

        fn execute<'a>(&self) -> Result<(), UsecaseError> {
            trace!("ChangeNoMember::execute called");
            self.dao()
                .run_tx(|mut ctx| {
                    trace!("ChangeNoMember::run_tx called");
                    self.record_membership(&mut ctx)?;

                    let mut emp = self.dao().fetch(self.get_emp_id()).run(&mut ctx)?;
                    debug!(
                        "changing emp member: {:?} -> NoAffiliation",
                        emp.affiliation()
                    );
                    emp.set_affiliation(Rc::new(RefCell::new(NoAffiliation)));
                    debug!("changed emp member={:?}", emp.affiliation());
                    self.dao().update(emp).run(&mut ctx)
                })
                .map_err(UsecaseError::ChangeNoMemberFailed)
        }
    }
}
pub use chg_no_member::*;

mod payday {
    use chrono::NaiveDate;
    use log::trace;
    use tx_rs::Tx;

    use super::UsecaseError;
    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_domain::Paycheck;

    // ユースケース: Payday トランザクション(抽象レベルのビジネスロジック)
    pub trait Payday: HaveEmployeeDao {
        fn get_pay_date(&self) -> NaiveDate;

        fn execute<'a>(&self) -> Result<(), UsecaseError> {
            trace!("Payday::execute called");
            self.dao()
                .run_tx(|mut ctx| {
                    trace!("Payday::run_tx called");
                    let mut emps = self.dao().fetch_all().run(&mut ctx)?;
                    for (emp_id, emp) in emps.iter_mut() {
                        if emp.is_pay_date(self.get_pay_date()) {
                            let period = emp.get_pay_period(self.get_pay_date());
                            let mut pc = Paycheck::new(period);
                            emp.payday(&mut pc);
                            self.dao().record_paycheck(*emp_id, pc).run(&mut ctx)?;
                        }
                    }
                    Ok(())
                })
                .map_err(UsecaseError::PaydayFailed)
        }
    }
}
pub use payday::*;
