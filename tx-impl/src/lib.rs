mod interface {
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
            Affiliation, Employee, EmployeeId, PaymentClassification, PaymentMethod,
            PaymentSchedule,
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
}
pub use interface::*;

mod tx_impl {
    mod add_hourly_emp_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        use super::super::AddEmployee;
        use dao::{EmployeeDao, HaveEmployeeDao};
        use payroll_domain::{
            Affiliation, EmployeeId, NoAffiliation, PaymentClassification, PaymentMethod,
            PaymentSchedule,
        };
        use payroll_impl::{HoldMethod, HourlyClassification, WeeklySchedule};
        use tx_app::{Response, Transaction};

        // ユースケース: AddHourlyEmployee トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct AddHourlyEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            id: EmployeeId,
            name: String,
            address: String,
            hourly_rate: f32,

            dao: T,
        }
        impl<T> AddHourlyEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            pub fn new(
                id: EmployeeId,
                name: &str,
                address: &str,
                hourly_rate: f32,
                dao: T,
            ) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    address: address.to_string(),
                    hourly_rate,
                    dao,
                }
            }
        }

        impl<T> HaveEmployeeDao for AddHourlyEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.dao
            }
        }
        impl<T> AddEmployee for AddHourlyEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            fn get_id(&self) -> EmployeeId {
                self.id
            }
            fn get_name(&self) -> &str {
                &self.name
            }
            fn get_address(&self) -> &str {
                &self.address
            }
            fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
                Rc::new(RefCell::new(HourlyClassification::new(self.hourly_rate)))
            }
            fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
                Rc::new(RefCell::new(WeeklySchedule))
            }
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
                Rc::new(RefCell::new(HoldMethod))
            }
            fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>> {
                Rc::new(RefCell::new(NoAffiliation))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for AddHourlyEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("AddHourlyEmployeeTx::execute called");
                AddEmployee::execute(self)
                    .map(|_| Response::EmployeeId(self.id))
                    .map_err(Into::into)
            }
        }
    }
    pub use add_hourly_emp_tx::*;

    mod add_salaried_emp_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        use super::super::AddEmployee;
        use dao::{EmployeeDao, HaveEmployeeDao};
        use payroll_domain::{
            Affiliation, EmployeeId, NoAffiliation, PaymentClassification, PaymentMethod,
            PaymentSchedule,
        };
        use payroll_impl::{HoldMethod, MonthlySchedule, SalariedClassification};
        use tx_app::{Response, Transaction};

        // ユースケース: AddSalariedEmployee トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct AddSalariedEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            id: EmployeeId,
            name: String,
            address: String,
            salary: f32,

            dao: T,
        }
        impl<T> AddSalariedEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            pub fn new(id: EmployeeId, name: &str, address: &str, salary: f32, dao: T) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    address: address.to_string(),
                    salary,
                    dao,
                }
            }
        }

        impl<T> HaveEmployeeDao for AddSalariedEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.dao
            }
        }
        impl<T> AddEmployee for AddSalariedEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            fn get_id(&self) -> EmployeeId {
                self.id
            }
            fn get_name(&self) -> &str {
                &self.name
            }
            fn get_address(&self) -> &str {
                &self.address
            }
            fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
                Rc::new(RefCell::new(SalariedClassification::new(self.salary)))
            }
            fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
                Rc::new(RefCell::new(MonthlySchedule))
            }
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
                Rc::new(RefCell::new(HoldMethod))
            }
            fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>> {
                Rc::new(RefCell::new(NoAffiliation))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for AddSalariedEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("AddSalariedEmployeeTx::execute called");
                AddEmployee::execute(self)
                    .map(|_| Response::EmployeeId(self.id))
                    .map_err(Into::into)
            }
        }
    }
    pub use add_salaried_emp_tx::*;

    mod add_commissioned_emp_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        use super::super::AddEmployee;
        use dao::{EmployeeDao, HaveEmployeeDao};
        use payroll_domain::{
            Affiliation, EmployeeId, NoAffiliation, PaymentClassification, PaymentMethod,
            PaymentSchedule,
        };
        use payroll_impl::{BiweeklySchedule, CommissionedClassification, HoldMethod};
        use tx_app::{Response, Transaction};

        // ユースケース: AddCommissionedEmployee トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct AddCommissionedEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            id: EmployeeId,
            name: String,
            address: String,
            salary: f32,
            commission_rate: f32,

            dao: T,
        }
        impl<T> AddCommissionedEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            pub fn new(
                id: EmployeeId,
                name: &str,
                address: &str,
                salary: f32,
                commission_rate: f32,
                dao: T,
            ) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    address: address.to_string(),
                    salary,
                    commission_rate,
                    dao,
                }
            }
        }

        impl<T> HaveEmployeeDao for AddCommissionedEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.dao
            }
        }
        impl<T> AddEmployee for AddCommissionedEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            fn get_id(&self) -> EmployeeId {
                self.id
            }
            fn get_name(&self) -> &str {
                &self.name
            }
            fn get_address(&self) -> &str {
                &self.address
            }
            fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
                Rc::new(RefCell::new(CommissionedClassification::new(
                    self.salary,
                    self.commission_rate,
                )))
            }
            fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
                Rc::new(RefCell::new(BiweeklySchedule))
            }
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
                Rc::new(RefCell::new(HoldMethod))
            }
            fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>> {
                Rc::new(RefCell::new(NoAffiliation))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for AddCommissionedEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("AddCommissionedEmployeeTx::execute called");
                AddEmployee::execute(self)
                    .map(|_| Response::EmployeeId(self.id))
                    .map_err(Into::into)
            }
        }
    }
    pub use add_commissioned_emp_tx::*;

    mod del_emp_tx {
        use anyhow;
        use log::trace;

        use super::super::DeleteEmployee;
        use dao::{EmployeeDao, HaveEmployeeDao};
        use payroll_domain::EmployeeId;
        use tx_app::{Response, Transaction};

        // ユースケース: DeleteEmployee トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct DeleteEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            id: EmployeeId,
            dao: T,
        }
        impl<T> DeleteEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            pub fn new(id: EmployeeId, dao: T) -> Self {
                Self { id, dao }
            }
        }

        impl<T> HaveEmployeeDao for DeleteEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.dao
            }
        }
        impl<T> DeleteEmployee for DeleteEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            fn get_id(&self) -> EmployeeId {
                self.id
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for DeleteEmployeeTx<T>
        where
            T: EmployeeDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("DeleteEmployeeTx::execute called");
                DeleteEmployee::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use del_emp_tx::*;

    mod add_timecard {
        use anyhow;
        use chrono::NaiveDate;
        use log::trace;

        use super::super::AddTimeCard;
        use dao::{EmployeeDao, HaveEmployeeDao};
        use payroll_domain::EmployeeId;
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
        impl<T> AddTimeCard for AddTimeCardTx<T>
        where
            T: EmployeeDao,
        {
            fn get_id(&self) -> EmployeeId {
                self.id
            }
            fn get_date(&self) -> NaiveDate {
                self.date
            }
            fn get_hours(&self) -> f32 {
                self.hours
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for AddTimeCardTx<T>
        where
            T: EmployeeDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("AddTimeCardTx::execute called");
                AddTimeCard::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use add_timecard::*;

    mod add_sales_receipt {
        use anyhow;
        use chrono::NaiveDate;
        use log::trace;

        use super::super::AddSalesReceipt;
        use dao::{EmployeeDao, HaveEmployeeDao};
        use payroll_domain::EmployeeId;
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
        impl<T> AddSalesReceipt for AddSalesReceiptTx<T>
        where
            T: EmployeeDao,
        {
            fn get_id(&self) -> EmployeeId {
                self.id
            }
            fn get_date(&self) -> NaiveDate {
                self.date
            }
            fn get_amount(&self) -> f32 {
                self.amount
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for AddSalesReceiptTx<T>
        where
            T: EmployeeDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("AddSalesReceiptTx::execute called");
                AddSalesReceipt::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use add_sales_receipt::*;

    mod add_service_charge_tx {
        use anyhow;
        use chrono::NaiveDate;
        use log::trace;

        use super::super::AddServiceCharge;
        use dao::{EmployeeDao, HaveEmployeeDao};
        use payroll_domain::MemberId;
        use tx_app::{Response, Transaction};

        // ユースケース: AddServiceCharge トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct AddServiceChargeTx<T>
        where
            T: EmployeeDao,
        {
            member_id: MemberId,
            date: NaiveDate,
            amount: f32,

            dao: T,
        }
        impl<T> AddServiceChargeTx<T>
        where
            T: EmployeeDao,
        {
            pub fn new(member_id: MemberId, date: NaiveDate, amount: f32, dao: T) -> Self {
                Self {
                    member_id,
                    date,
                    amount,
                    dao,
                }
            }
        }

        impl<T> HaveEmployeeDao for AddServiceChargeTx<T>
        where
            T: EmployeeDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.dao
            }
        }
        impl<T> AddServiceCharge for AddServiceChargeTx<T>
        where
            T: EmployeeDao,
        {
            fn get_member_id(&self) -> MemberId {
                self.member_id
            }
            fn get_date(&self) -> NaiveDate {
                self.date
            }
            fn get_amount(&self) -> f32 {
                self.amount
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for AddServiceChargeTx<T>
        where
            T: EmployeeDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("AddServiceChargeTx::execute called");
                AddServiceCharge::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use add_service_charge_tx::*;

    mod chg_name_tx {
        use anyhow;
        use log::trace;

        use super::super::ChangeEmployeeName;
        use dao::{EmployeeDao, HaveEmployeeDao};
        use payroll_domain::EmployeeId;
        use tx_app::{Response, Transaction};

        // ユースケース: ChangeEmployeeName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChangeEmployeeNameTx<T>
        where
            T: EmployeeDao,
        {
            id: EmployeeId,
            new_name: String,
            dao: T,
        }
        impl<T> ChangeEmployeeNameTx<T>
        where
            T: EmployeeDao,
        {
            pub fn new(id: EmployeeId, new_name: &str, dao: T) -> Self {
                Self {
                    id,
                    new_name: new_name.to_string(),
                    dao,
                }
            }
        }

        impl<T> HaveEmployeeDao for ChangeEmployeeNameTx<T>
        where
            T: EmployeeDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.dao
            }
        }
        impl<T> ChangeEmployeeName for ChangeEmployeeNameTx<T>
        where
            T: EmployeeDao,
        {
            fn get_id(&self) -> EmployeeId {
                self.id
            }
            fn get_new_name(&self) -> &str {
                &self.new_name
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChangeEmployeeNameTx<T>
        where
            T: EmployeeDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChangeEmployeeNameTx::execute called");
                ChangeEmployeeName::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_name_tx::*;

    mod chg_address_tx {
        use anyhow;
        use log::trace;

        use super::super::ChangeEmployeeAddress;
        use dao::{EmployeeDao, HaveEmployeeDao};
        use payroll_domain::EmployeeId;
        use tx_app::{Response, Transaction};

        // ユースケース: ChangeEmployeeAddress トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChangeEmployeeAddressTx<T>
        where
            T: EmployeeDao,
        {
            id: EmployeeId,
            new_address: String,
            dao: T,
        }
        impl<T> ChangeEmployeeAddressTx<T>
        where
            T: EmployeeDao,
        {
            pub fn new(id: EmployeeId, new_address: &str, dao: T) -> Self {
                Self {
                    id,
                    new_address: new_address.to_string(),
                    dao,
                }
            }
        }

        impl<T> HaveEmployeeDao for ChangeEmployeeAddressTx<T>
        where
            T: EmployeeDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.dao
            }
        }
        impl<T> ChangeEmployeeAddress for ChangeEmployeeAddressTx<T>
        where
            T: EmployeeDao,
        {
            fn get_id(&self) -> EmployeeId {
                self.id
            }
            fn get_new_address(&self) -> &str {
                &self.new_address
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChangeEmployeeAddressTx<T>
        where
            T: EmployeeDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChangeEmployeeAddressTx::execute called");
                ChangeEmployeeAddress::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_address_tx::*;

    mod chg_hourly_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        use super::super::ChangeClassification;
        use dao::{EmployeeDao, HaveEmployeeDao};
        use payroll_domain::{EmployeeId, PaymentClassification};
        use payroll_impl::{HourlyClassification, WeeklySchedule};
        use tx_app::{Response, Transaction};

        // ユースケース: ChangeHourly トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChangeHourlyTx<T>
        where
            T: EmployeeDao,
        {
            id: EmployeeId,
            hourly_rate: f32,

            dao: T,
        }
        impl<T> ChangeHourlyTx<T>
        where
            T: EmployeeDao,
        {
            pub fn new(id: EmployeeId, hourly_rate: f32, dao: T) -> Self {
                Self {
                    id,
                    hourly_rate,
                    dao,
                }
            }
        }

        impl<T> HaveEmployeeDao for ChangeHourlyTx<T>
        where
            T: EmployeeDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.dao
            }
        }
        impl<T> ChangeClassification for ChangeHourlyTx<T>
        where
            T: EmployeeDao,
        {
            fn get_id(&self) -> EmployeeId {
                self.id
            }
            fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
                Rc::new(RefCell::new(HourlyClassification::new(self.hourly_rate)))
            }
            fn get_schedule(&self) -> Rc<RefCell<dyn payroll_domain::PaymentSchedule>> {
                Rc::new(RefCell::new(WeeklySchedule))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChangeHourlyTx<T>
        where
            T: EmployeeDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChangeHourlyTx::execute called");
                ChangeClassification::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_hourly_tx::*;

    mod chg_salary_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        use super::super::ChangeClassification;
        use dao::{EmployeeDao, HaveEmployeeDao};
        use payroll_domain::{EmployeeId, PaymentClassification};
        use payroll_impl::{MonthlySchedule, SalariedClassification};
        use tx_app::{Response, Transaction};

        // ユースケース: ChangeSalaried トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChangeSalariedTx<T>
        where
            T: EmployeeDao,
        {
            id: EmployeeId,
            salary: f32,

            dao: T,
        }
        impl<T> ChangeSalariedTx<T>
        where
            T: EmployeeDao,
        {
            pub fn new(id: EmployeeId, salary: f32, dao: T) -> Self {
                Self { id, salary, dao }
            }
        }

        impl<T> HaveEmployeeDao for ChangeSalariedTx<T>
        where
            T: EmployeeDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.dao
            }
        }
        impl<T> ChangeClassification for ChangeSalariedTx<T>
        where
            T: EmployeeDao,
        {
            fn get_id(&self) -> EmployeeId {
                self.id
            }
            fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
                Rc::new(RefCell::new(SalariedClassification::new(self.salary)))
            }
            fn get_schedule(&self) -> Rc<RefCell<dyn payroll_domain::PaymentSchedule>> {
                Rc::new(RefCell::new(MonthlySchedule))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChangeSalariedTx<T>
        where
            T: EmployeeDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChangeSalariedTx::execute called");
                ChangeClassification::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_salary_tx::*;

    mod chg_commissioned_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        use super::super::ChangeClassification;
        use dao::{EmployeeDao, HaveEmployeeDao};
        use payroll_domain::{EmployeeId, PaymentClassification};
        use payroll_impl::{BiweeklySchedule, CommissionedClassification};
        use tx_app::{Response, Transaction};

        // ユースケース: ChangeCommissioned トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChangeCommissionedTx<T>
        where
            T: EmployeeDao,
        {
            id: EmployeeId,
            salary: f32,
            commission_rate: f32,

            dao: T,
        }
        impl<T> ChangeCommissionedTx<T>
        where
            T: EmployeeDao,
        {
            pub fn new(id: EmployeeId, salary: f32, commission_rate: f32, dao: T) -> Self {
                Self {
                    id,
                    salary,
                    commission_rate,
                    dao,
                }
            }
        }

        impl<T> HaveEmployeeDao for ChangeCommissionedTx<T>
        where
            T: EmployeeDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.dao
            }
        }
        impl<T> ChangeClassification for ChangeCommissionedTx<T>
        where
            T: EmployeeDao,
        {
            fn get_id(&self) -> EmployeeId {
                self.id
            }
            fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
                Rc::new(RefCell::new(CommissionedClassification::new(
                    self.salary,
                    self.commission_rate,
                )))
            }
            fn get_schedule(&self) -> Rc<RefCell<dyn payroll_domain::PaymentSchedule>> {
                Rc::new(RefCell::new(BiweeklySchedule))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChangeCommissionedTx<T>
        where
            T: EmployeeDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChangeCommissionedTx::execute called");
                ChangeClassification::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_commissioned_tx::*;

    mod chg_hold_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        use super::super::ChgMethod;
        use dao::{EmployeeDao, HaveEmployeeDao};
        use payroll_domain::{EmployeeId, PaymentMethod};
        use payroll_impl::HoldMethod;
        use tx_app::{Response, Transaction};

        // ユースケース: ChangeHold トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChangeHoldTx<T>
        where
            T: EmployeeDao,
        {
            id: EmployeeId,

            dao: T,
        }
        impl<T> ChangeHoldTx<T>
        where
            T: EmployeeDao,
        {
            pub fn new(id: EmployeeId, dao: T) -> Self {
                Self { id, dao }
            }
        }

        impl<T> HaveEmployeeDao for ChangeHoldTx<T>
        where
            T: EmployeeDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.dao
            }
        }
        impl<T> ChgMethod for ChangeHoldTx<T>
        where
            T: EmployeeDao,
        {
            fn get_id(&self) -> EmployeeId {
                self.id
            }
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
                Rc::new(RefCell::new(HoldMethod))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChangeHoldTx<T>
        where
            T: EmployeeDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChangeHoldTx::execute called");
                ChgMethod::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_hold_tx::*;

    mod chg_direct_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        use super::super::ChgMethod;
        use dao::{EmployeeDao, HaveEmployeeDao};
        use payroll_domain::{EmployeeId, PaymentMethod};
        use payroll_impl::DirectMethod;
        use tx_app::{Response, Transaction};

        // ユースケース: ChangeDirect トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChangeDirectTx<T>
        where
            T: EmployeeDao,
        {
            id: EmployeeId,
            bank: String,
            account: String,

            dao: T,
        }
        impl<T> ChangeDirectTx<T>
        where
            T: EmployeeDao,
        {
            pub fn new(id: EmployeeId, bank: &str, account: &str, dao: T) -> Self {
                Self {
                    id,
                    bank: bank.to_string(),
                    account: account.to_string(),
                    dao,
                }
            }
        }

        impl<T> HaveEmployeeDao for ChangeDirectTx<T>
        where
            T: EmployeeDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.dao
            }
        }
        impl<T> ChgMethod for ChangeDirectTx<T>
        where
            T: EmployeeDao,
        {
            fn get_id(&self) -> EmployeeId {
                self.id
            }
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
                Rc::new(RefCell::new(DirectMethod::new(&self.bank, &self.account)))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChangeDirectTx<T>
        where
            T: EmployeeDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChangeDirectTx::execute called");
                ChgMethod::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_direct_tx::*;

    mod chg_mail_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        use super::super::ChgMethod;
        use dao::{EmployeeDao, HaveEmployeeDao};
        use payroll_domain::{EmployeeId, PaymentMethod};
        use payroll_impl::MailMethod;
        use tx_app::{Response, Transaction};

        // ユースケース: ChangeMail トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChangeMailTx<T>
        where
            T: EmployeeDao,
        {
            id: EmployeeId,
            address: String,

            dao: T,
        }
        impl<T> ChangeMailTx<T>
        where
            T: EmployeeDao,
        {
            pub fn new(id: EmployeeId, address: &str, dao: T) -> Self {
                Self {
                    id,
                    address: address.to_string(),
                    dao,
                }
            }
        }

        impl<T> HaveEmployeeDao for ChangeMailTx<T>
        where
            T: EmployeeDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.dao
            }
        }
        impl<T> ChgMethod for ChangeMailTx<T>
        where
            T: EmployeeDao,
        {
            fn get_id(&self) -> EmployeeId {
                self.id
            }
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
                Rc::new(RefCell::new(MailMethod::new(&self.address)))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChangeMailTx<T>
        where
            T: EmployeeDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChangeMailTx::execute called");
                ChgMethod::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_mail_tx::*;

    mod chg_member_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        use super::super::ChangeMember;
        use dao::{EmployeeDao, HaveEmployeeDao};
        use payroll_domain::{EmployeeId, MemberId};
        use payroll_impl::UnionAffiliation;
        use tx_app::{Response, Transaction};

        // ユースケース: ChangeMember トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChangeMemberTx<T>
        where
            T: EmployeeDao,
        {
            member_id: MemberId,
            emp_id: EmployeeId,
            dues: f32,

            dao: T,
        }
        impl<T> ChangeMemberTx<T>
        where
            T: EmployeeDao,
        {
            pub fn new(member_id: MemberId, emp_id: EmployeeId, dues: f32, dao: T) -> Self {
                Self {
                    member_id,
                    emp_id,
                    dues,
                    dao,
                }
            }
        }

        impl<T> HaveEmployeeDao for ChangeMemberTx<T>
        where
            T: EmployeeDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.dao
            }
        }
        impl<T> ChangeMember for ChangeMemberTx<T>
        where
            T: EmployeeDao,
        {
            fn get_member_id(&self) -> MemberId {
                self.member_id
            }
            fn get_emp_id(&self) -> EmployeeId {
                self.emp_id
            }
            fn get_dues(&self) -> f32 {
                self.dues
            }
            fn get_affiliation(&self) -> Rc<RefCell<dyn payroll_domain::Affiliation>> {
                Rc::new(RefCell::new(UnionAffiliation::new(
                    self.get_member_id(),
                    self.get_dues(),
                )))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChangeMemberTx<T>
        where
            T: EmployeeDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChangeMemberTx::execute called");
                ChangeMember::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_member_tx::*;

    mod chg_no_member_tx {
        use anyhow;
        use log::trace;

        use super::super::ChangeNoMember;
        use dao::{EmployeeDao, HaveEmployeeDao};
        use payroll_domain::EmployeeId;
        use tx_app::{Response, Transaction};

        // ユースケース: ChangeNoMember トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChangeNoMemberTx<T>
        where
            T: EmployeeDao,
        {
            emp_id: EmployeeId,

            dao: T,
        }
        impl<T> ChangeNoMemberTx<T>
        where
            T: EmployeeDao,
        {
            pub fn new(emp_id: EmployeeId, dao: T) -> Self {
                Self { emp_id, dao }
            }
        }

        impl<T> HaveEmployeeDao for ChangeNoMemberTx<T>
        where
            T: EmployeeDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.dao
            }
        }
        impl<T> ChangeNoMember for ChangeNoMemberTx<T>
        where
            T: EmployeeDao,
        {
            fn get_emp_id(&self) -> EmployeeId {
                self.emp_id
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChangeNoMemberTx<T>
        where
            T: EmployeeDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChangeNoMemberTx::execute called");
                ChangeNoMember::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_no_member_tx::*;

    mod payday_tx {
        use anyhow;
        use chrono::NaiveDate;
        use log::trace;

        use super::super::Payday;
        use dao::{EmployeeDao, HaveEmployeeDao};
        use tx_app::{Response, Transaction};

        // ユースケース: Payday トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct PaydayTx<T>
        where
            T: EmployeeDao,
        {
            pay_date: NaiveDate,

            dao: T,
        }
        impl<T> PaydayTx<T>
        where
            T: EmployeeDao,
        {
            pub fn new(pay_date: NaiveDate, dao: T) -> Self {
                Self { pay_date, dao }
            }
        }

        impl<T> HaveEmployeeDao for PaydayTx<T>
        where
            T: EmployeeDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.dao
            }
        }
        impl<T> Payday for PaydayTx<T>
        where
            T: EmployeeDao,
        {
            fn get_pay_date(&self) -> NaiveDate {
                self.pay_date
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for PaydayTx<T>
        where
            T: EmployeeDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("PaydayTx::execute called");
                Payday::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use payday_tx::*;
}
pub use tx_impl::*;

mod tx_factory_impl {
    use chrono::NaiveDate;
    use log::trace;

    use dao::EmployeeDao;
    use payroll_domain::{EmployeeId, MemberId};
    use tx_app::Transaction;
    use tx_factory::TxFactory;

    use crate::{
        AddCommissionedEmployeeTx, AddHourlyEmployeeTx, AddSalariedEmployeeTx, AddSalesReceiptTx,
        AddServiceChargeTx, AddTimeCardTx, ChangeCommissionedTx, ChangeDirectTx,
        ChangeEmployeeAddressTx, ChangeEmployeeNameTx, ChangeHoldTx, ChangeHourlyTx, ChangeMailTx,
        ChangeMemberTx, ChangeNoMemberTx, ChangeSalariedTx, DeleteEmployeeTx, PaydayTx,
    };

    pub struct TxFactoryImpl<T>
    where
        T: EmployeeDao + Clone,
    {
        dao: T,
    }
    impl<T> TxFactoryImpl<T>
    where
        T: EmployeeDao + Clone,
    {
        pub fn new(dao: T) -> Self {
            Self { dao }
        }
    }
    impl<T> TxFactory for TxFactoryImpl<T>
    where
        T: EmployeeDao + Clone + 'static,
    {
        fn mk_add_hourly_employee_tx(
            &self,
            id: EmployeeId,
            name: &str,
            address: &str,
            hourly_rate: f32,
        ) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_add_hourly_employee_tx called");
            Box::new(AddHourlyEmployeeTx::new(
                id,
                name,
                address,
                hourly_rate,
                self.dao.clone(),
            ))
        }
        fn mk_add_salaried_employee_tx(
            &self,
            id: EmployeeId,
            name: &str,
            address: &str,
            salary: f32,
        ) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_add_salaried_employee_tx called");
            Box::new(AddSalariedEmployeeTx::new(
                id,
                name,
                address,
                salary,
                self.dao.clone(),
            ))
        }
        fn mk_add_commissioned_employee_tx(
            &self,
            id: EmployeeId,
            name: &str,
            address: &str,
            salary: f32,
            commission_rate: f32,
        ) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_add_commissioned_employee_tx called");
            Box::new(AddCommissionedEmployeeTx::new(
                id,
                name,
                address,
                salary,
                commission_rate,
                self.dao.clone(),
            ))
        }
        fn mk_delete_employee_tx(&self, id: EmployeeId) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_delete_employee_tx called");
            Box::new(DeleteEmployeeTx::new(id, self.dao.clone()))
        }
        fn mk_add_timecard_tx(
            &self,
            id: EmployeeId,
            date: NaiveDate,
            hours: f32,
        ) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_add_timecard_tx called");
            Box::new(AddTimeCardTx::new(id, date, hours, self.dao.clone()))
        }
        fn mk_add_sales_receipt_tx(
            &self,
            id: EmployeeId,
            date: NaiveDate,
            amount: f32,
        ) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_add_sales_receipt_tx called");
            Box::new(AddSalesReceiptTx::new(id, date, amount, self.dao.clone()))
        }
        fn mk_add_service_charge_tx(
            &self,
            id: MemberId,
            date: NaiveDate,
            amount: f32,
        ) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_add_service_charge_tx called");
            Box::new(AddServiceChargeTx::new(id, date, amount, self.dao.clone()))
        }
        fn mk_change_employee_name_tx(
            &self,
            id: EmployeeId,
            new_name: &str,
        ) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_change_name_tx called");
            Box::new(ChangeEmployeeNameTx::new(id, new_name, self.dao.clone()))
        }
        fn mk_change_employee_address_tx(
            &self,
            id: EmployeeId,
            new_address: &str,
        ) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_change_address_tx called");
            Box::new(ChangeEmployeeAddressTx::new(
                id,
                new_address,
                self.dao.clone(),
            ))
        }
        fn mk_change_employee_hourly_tx(
            &self,
            id: EmployeeId,
            hourly_rate: f32,
        ) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_change_employee_hourly_tx called");
            Box::new(ChangeHourlyTx::new(id, hourly_rate, self.dao.clone()))
        }
        fn mk_change_employee_salaried_tx(
            &self,
            id: EmployeeId,
            salary: f32,
        ) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_change_employee_salaried_tx called");
            Box::new(ChangeSalariedTx::new(id, salary, self.dao.clone()))
        }
        fn mk_change_employee_commissioned_tx(
            &self,
            id: EmployeeId,
            salary: f32,
            commission_rate: f32,
        ) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_change_employee_commissioned_tx called");
            Box::new(ChangeCommissionedTx::new(
                id,
                salary,
                commission_rate,
                self.dao.clone(),
            ))
        }
        fn mk_change_employee_hold_tx(&self, id: EmployeeId) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_change_employee_hold_tx called");
            Box::new(ChangeHoldTx::new(id, self.dao.clone()))
        }
        fn mk_change_employee_direct_tx(
            &self,
            id: EmployeeId,
            bank: &str,
            account: &str,
        ) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_change_employee_direct_tx called");
            Box::new(ChangeDirectTx::new(id, bank, account, self.dao.clone()))
        }
        fn mk_change_employee_mail_tx(
            &self,
            id: EmployeeId,
            address: &str,
        ) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_change_employee_mail_tx called");
            Box::new(ChangeMailTx::new(id, address, self.dao.clone()))
        }
        fn mk_change_employee_member_tx(
            &self,
            emp_id: EmployeeId,
            member_id: MemberId,
            dues: f32,
        ) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_change_employee_member_tx called");
            Box::new(ChangeMemberTx::new(
                member_id,
                emp_id,
                dues,
                self.dao.clone(),
            ))
        }
        fn mk_change_employee_no_member_tx(&self, id: EmployeeId) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_change_employee_no_member_tx called");
            Box::new(ChangeNoMemberTx::new(id, self.dao.clone()))
        }
        fn mk_payday_tx(&self, date: NaiveDate) -> Box<dyn Transaction> {
            trace!("TxFactoryImpl::mk_payday_tx called");
            Box::new(PaydayTx::new(date, self.dao.clone()))
        }
    }
}
pub use tx_factory_impl::*;
