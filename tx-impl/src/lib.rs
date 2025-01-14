// ユースケースのトランザクションのインターフェース
mod interface {
    use thiserror::Error;

    // dao にのみ依存
    use dao::DaoError;

    #[derive(Debug, Clone, Error)]
    pub enum UsecaseError {
        #[error("add employee failed: {0}")]
        AddEmpFailed(DaoError),
        #[error("employee retrieval failed: {0}")]
        EmpRetrievalFailed(DaoError),
        #[error("delete employee failed: {0}")]
        DelEmpFailed(DaoError),
        #[error("add timecard failed: {0}")]
        AddTimeCardFailed(DaoError),
        #[error("add sales receipt failed: {0}")]
        AddSalesReceiptFailed(DaoError),
        #[error("change employee name failed: {0}")]
        ChgEmpNameFailed(DaoError),
        #[error("change employee address failed: {0}")]
        ChgEmpAddressFailed(DaoError),
        #[error("change employee classification failed: {0}")]
        ChgPaymentClassificationFailed(DaoError),
        #[error("change employee method failed: {0}")]
        ChgPaymentMethodFailed(DaoError),
        #[error("change member failed: {0}")]
        ChgMemberFailed(DaoError),
        #[error("change unaffiliated failed: {0}")]
        ChgUnaffiliatedFailed(DaoError),
        #[error("add service charge failed: {0}")]
        AddServiceChargeFailed(DaoError),
        #[error("payday failed: {0}")]
        PaydayFailed(DaoError),
        #[error("unexpected error: {0}")]
        UnexpectedError(String),
    }

    mod add_emp {
        use log::{debug, trace};
        use std::{cell::RefCell, rc::Rc};
        use tx_rs::Tx;

        // dao にのみ依存 (domain は当然 ok)
        use super::UsecaseError;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{
            Affiliation, Emp, EmpId, PaymentClassification, PaymentMethod, PaymentSchedule,
        };

        // ユースケース: AddEmp トランザクション(抽象レベルのビジネスロジック)
        pub trait AddEmp: HaveEmpDao {
            fn get_id(&self) -> EmpId;
            fn get_name(&self) -> &str;
            fn get_address(&self) -> &str;
            fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>>;
            fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>>;
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>>;
            fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>>;

            fn execute<'a>(&self) -> Result<(), UsecaseError> {
                trace!("AddEmp::execute called");
                self.dao()
                    .run_tx(|mut ctx| {
                        trace!("AddEmp::run_tx called");
                        let emp = Emp::new(
                            self.get_id(),
                            self.get_name(),
                            self.get_address(),
                            self.get_classification(),
                            self.get_schedule(),
                            self.get_method(),
                            self.get_affiliation(),
                        );
                        debug!("AddEmp::execute: emp={:?}", emp);
                        self.dao().insert(emp).run(&mut ctx)
                    })
                    .map(|_| ())
                    .map_err(UsecaseError::AddEmpFailed)
            }
        }
    }
    pub use add_emp::*;

    mod del_emp {
        use log::{debug, trace};
        use tx_rs::Tx;

        // dao にのみ依存 (domain は当然 ok)
        use super::UsecaseError;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::EmpId;

        // ユースケース: AddEmp トランザクション(抽象レベルのビジネスロジック)
        pub trait DelEmp: HaveEmpDao {
            fn get_id(&self) -> EmpId;

            fn execute<'a>(&self) -> Result<(), UsecaseError> {
                trace!("DelEmp::execute called");
                self.dao()
                    .run_tx(|mut ctx| {
                        trace!("DelEmp::run_tx called");
                        let emp_id = self.get_id();
                        debug!("DelEmp::execute: emp_id={}", emp_id);
                        self.dao().remove(emp_id).run(&mut ctx)
                    })
                    .map(|_| ())
                    .map_err(UsecaseError::DelEmpFailed)
            }
        }
    }
    pub use del_emp::*;

    mod add_timecard {
        use chrono::NaiveDate;
        use log::trace;
        use payroll_impl::HourlyClassification;
        use tx_rs::Tx;

        // dao にのみ依存 (domain は当然 ok)
        use super::UsecaseError;
        use dao::{DaoError, EmpDao, HaveEmpDao};
        use payroll_domain::EmpId;

        // ユースケース: AddTimeCard トランザクション(抽象レベルのビジネスロジック)
        pub trait AddTimeCard: HaveEmpDao {
            fn get_id(&self) -> EmpId;
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
                                "classification is not HourlyClassification".to_string(),
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
        use payroll_impl::CommissionedClassification;
        use tx_rs::Tx;

        // dao にのみ依存 (domain は当然 ok)
        use super::UsecaseError;
        use dao::{DaoError, EmpDao, HaveEmpDao};
        use payroll_domain::EmpId;

        // ユースケース: AddSalesReceipt トランザクション(抽象レベルのビジネスロジック)
        pub trait AddSalesReceipt: HaveEmpDao {
            fn get_id(&self) -> EmpId;
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

    mod chg_name {
        use log::{debug, trace};
        use tx_rs::Tx;

        // dao にのみ依存 (domain は当然 ok)
        use super::UsecaseError;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::EmpId;

        // ユースケース: ChgEmpName トランザクション(抽象レベルのビジネスロジック)
        pub trait ChgEmpName: HaveEmpDao {
            fn get_id(&self) -> EmpId;
            fn get_new_name(&self) -> &str;
            fn execute<'a>(&self) -> Result<(), UsecaseError> {
                trace!("ChgEmpName::execute called");
                self.dao()
                    .run_tx(|mut ctx| {
                        trace!("ChgEmpName::run_tx called");
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
                    .map_err(UsecaseError::ChgEmpNameFailed)
            }
        }
    }
    pub use chg_name::*;

    mod chg_address {
        use log::{debug, trace};
        use tx_rs::Tx;

        // dao にのみ依存 (domain は当然 ok)
        use super::UsecaseError;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::EmpId;

        // ユースケース: ChgEmpAddress トランザクション(抽象レベルのビジネスロジック)
        pub trait ChgEmpAddress: HaveEmpDao {
            fn get_id(&self) -> EmpId;
            fn get_new_address(&self) -> &str;
            fn execute<'a>(&self) -> Result<(), UsecaseError> {
                trace!("ChgEmpAddress::execute called");
                self.dao()
                    .run_tx(|mut ctx| {
                        trace!("ChgEmpAddress::run_tx called");
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
                    .map_err(UsecaseError::ChgEmpAddressFailed)
            }
        }
    }
    pub use chg_address::*;

    mod chg_classification {
        use log::{debug, trace};
        use std::{cell::RefCell, rc::Rc};
        use tx_rs::Tx;

        // dao にのみ依存 (domain は当然 ok)
        use super::UsecaseError;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{EmpId, PaymentClassification, PaymentSchedule};

        // ユースケース: ChgClassification トランザクション(抽象レベルのビジネスロジック)
        pub trait ChgClassification: HaveEmpDao {
            fn get_id(&self) -> EmpId;
            fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>>;
            fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>>;
            fn execute<'a>(&self) -> Result<(), UsecaseError> {
                trace!("ChgClassification::execute called");
                self.dao()
                    .run_tx(|mut ctx| {
                        trace!("ChgClassification::run_tx called");
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
                    .map_err(UsecaseError::ChgPaymentClassificationFailed)
            }
        }
    }
    pub use chg_classification::*;

    mod chg_method {
        use log::{debug, trace};
        use std::{cell::RefCell, rc::Rc};
        use tx_rs::Tx;

        // dao にのみ依存 (domain は当然 ok)
        use super::UsecaseError;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{EmpId, PaymentMethod};

        // ユースケース: ChgMethod トランザクション(抽象レベルのビジネスロジック)
        pub trait ChgMethod: HaveEmpDao {
            fn get_id(&self) -> EmpId;
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
                    .map_err(UsecaseError::ChgPaymentMethodFailed)
            }
        }
    }
    pub use chg_method::*;

    mod chg_member {
        use log::{debug, trace};
        use std::{cell::RefCell, rc::Rc};
        use tx_rs::Tx;

        // dao にのみ依存 (domain は当然 ok)
        use super::UsecaseError;
        use dao::{DaoError, EmpDao, HaveEmpDao};
        use payroll_domain::{Affiliation, EmpId, MemberId};

        // ユースケース: ChgMember トランザクション(抽象レベルのビジネスロジック)
        pub trait ChgMember: HaveEmpDao {
            fn get_member_id(&self) -> MemberId;
            fn get_emp_id(&self) -> EmpId;
            fn get_dues(&self) -> f32;
            fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>>;

            fn record_membership<'a>(&self, ctx: &mut Self::Ctx<'a>) -> Result<(), DaoError> {
                trace!("record_membership called");
                self.dao()
                    .add_union_member(self.get_member_id(), self.get_emp_id())
                    .run(ctx)
            }

            fn execute<'a>(&self) -> Result<(), UsecaseError> {
                trace!("ChgMember::execute called");
                self.dao()
                    .run_tx(|mut ctx| {
                        trace!("ChgMember::run_tx called");
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
                    .map_err(UsecaseError::ChgMemberFailed)
            }
        }
    }
    pub use chg_member::*;

    mod chg_unaffiliated {
        use log::{debug, trace};
        use std::{cell::RefCell, rc::Rc};
        use tx_rs::Tx;

        // dao にのみ依存 (domain は当然 ok)
        use super::UsecaseError;
        use dao::{DaoError, EmpDao, HaveEmpDao};
        use payroll_domain::{EmpId, NoAffiliation};
        use payroll_impl::UnionAffiliation;

        // ユースケース: ChgMember トランザクション(抽象レベルのビジネスロジック)
        pub trait ChgUnaffiliated: HaveEmpDao {
            fn get_emp_id(&self) -> EmpId;

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
                trace!("ChgUnaffiliated::execute called");
                self.dao()
                    .run_tx(|mut ctx| {
                        trace!("ChgUnaffiliated::run_tx called");
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
                    .map_err(UsecaseError::ChgUnaffiliatedFailed)
            }
        }
    }
    pub use chg_unaffiliated::*;

    mod add_service_charge {
        use chrono::NaiveDate;
        use log::trace;
        use tx_rs::Tx;

        // dao にのみ依存 (domain は当然 ok)
        use super::UsecaseError;
        use dao::{DaoError, EmpDao, HaveEmpDao};
        use payroll_domain::MemberId;
        use payroll_impl::UnionAffiliation;

        // ユースケース: AddTimeCard トランザクション(抽象レベルのビジネスロジック)
        pub trait AddServiceCharge: HaveEmpDao {
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

    mod payday {
        use chrono::NaiveDate;
        use log::trace;
        use tx_rs::Tx;

        // dao にのみ依存 (domain は当然 ok)
        use super::UsecaseError;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::Paycheck;

        // ユースケース: AddTimeCard トランザクション(抽象レベルのビジネスロジック)
        pub trait Payday: HaveEmpDao {
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

// ユースケースのトランザクションの実装
mod tx_impl {
    mod add_salaried_emp_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        // dao と tx_app のインターフェースにのみ依存 (domain は当然 ok)
        use super::super::AddEmp;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{
            Affiliation, EmpId, NoAffiliation, PaymentClassification, PaymentMethod,
            PaymentSchedule,
        };
        use payroll_impl::{HoldMethod, MonthlySchedule, SalariedClassification};
        use tx_app::{Response, Transaction};

        // ユースケース: AddSalariedEmp トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct AddSalariedEmpTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            name: String,
            address: String,
            salary: f32,

            db: T,
        }
        impl<T> AddSalariedEmpTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, name: &str, address: &str, salary: f32, dao: T) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    address: address.to_string(),
                    salary,

                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for AddSalariedEmpTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> AddEmp for AddSalariedEmpTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
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
        impl<T> Transaction for AddSalariedEmpTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("AddSalariedEmpTx::execute called");
                AddEmp::execute(self)
                    .map(|_| Response::EmpId(self.id))
                    .map_err(Into::into)
            }
        }
    }
    pub use add_salaried_emp_tx::*;

    mod add_hourly_emp_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        // dao と tx_app のインターフェースにのみ依存 (domain は当然 ok)
        use super::super::AddEmp;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{
            Affiliation, EmpId, NoAffiliation, PaymentClassification, PaymentMethod,
            PaymentSchedule,
        };
        use payroll_impl::{HoldMethod, HourlyClassification, WeeklySchedule};
        use tx_app::{Response, Transaction};

        // ユースケース: AddSalariedEmp トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct AddHourlyEmpTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            name: String,
            address: String,
            hourly_rate: f32,

            db: T,
        }
        impl<T> AddHourlyEmpTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, name: &str, address: &str, hourly_rate: f32, dao: T) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    address: address.to_string(),
                    hourly_rate,

                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for AddHourlyEmpTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> AddEmp for AddHourlyEmpTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
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
        impl<T> Transaction for AddHourlyEmpTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("AddHourlyEmpTx::execute called");
                AddEmp::execute(self)
                    .map(|_| Response::EmpId(self.id))
                    .map_err(Into::into)
            }
        }
    }
    pub use add_hourly_emp_tx::*;

    mod add_commissioned_emp_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        // dao と tx_app のインターフェースにのみ依存 (domain は当然 ok)
        use super::super::AddEmp;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{
            Affiliation, EmpId, NoAffiliation, PaymentClassification, PaymentMethod,
            PaymentSchedule,
        };
        use payroll_impl::{BiweeklySchedule, CommissionedClassification, HoldMethod};
        use tx_app::{Response, Transaction};

        // ユースケース: AddSalariedEmp トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct AddCommissionedEmpTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            name: String,
            address: String,
            salary: f32,
            commission_rate: f32,

            db: T,
        }
        impl<T> AddCommissionedEmpTx<T>
        where
            T: EmpDao,
        {
            pub fn new(
                id: EmpId,
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

                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for AddCommissionedEmpTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> AddEmp for AddCommissionedEmpTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
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
        impl<T> Transaction for AddCommissionedEmpTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("AddCommissionedEmpTx::execute called");
                AddEmp::execute(self)
                    .map(|_| Response::EmpId(self.id))
                    .map_err(Into::into)
            }
        }
    }
    pub use add_commissioned_emp_tx::*;

    mod del_emp_tx {
        use anyhow;
        use log::trace;

        // dao と tx_app のインターフェースにのみ依存 (domain は当然 ok)
        use super::super::DelEmp;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::EmpId;
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct DelEmpTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            db: T,
        }
        impl<T> DelEmpTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, dao: T) -> Self {
                Self { id, db: dao }
            }
        }

        impl<T> HaveEmpDao for DelEmpTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> DelEmp for DelEmpTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
                self.id
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for DelEmpTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("DelEmpTx::execute called");
                DelEmp::execute(self)
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

        // dao と tx_app のインターフェースにのみ依存 (domain は当然 ok)
        use super::super::AddTimeCard;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::EmpId;
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct AddTimeCardTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            date: NaiveDate,
            hours: f32,

            db: T,
        }
        impl<T> AddTimeCardTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, date: NaiveDate, hours: f32, dao: T) -> Self {
                Self {
                    id,
                    date,
                    hours,
                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for AddTimeCardTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> AddTimeCard for AddTimeCardTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
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
            T: EmpDao,
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

        // dao と tx_app のインターフェースにのみ依存 (domain は当然 ok)
        use super::super::AddSalesReceipt;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::EmpId;
        use tx_app::{Response, Transaction};

        // ユースケース: ChgSalesReceipt トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct AddSalesReceiptTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            date: NaiveDate,
            amount: f32,

            db: T,
        }
        impl<T> AddSalesReceiptTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, date: NaiveDate, amount: f32, dao: T) -> Self {
                Self {
                    id,
                    date,
                    amount,
                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for AddSalesReceiptTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> AddSalesReceipt for AddSalesReceiptTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
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
            T: EmpDao,
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

    mod chg_name_tx {
        use anyhow;
        use log::trace;

        // dao と tx_app のインターフェースにのみ依存 (domain は当然 ok)
        use super::super::ChgEmpName;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::EmpId;
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChgEmpNameTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            new_name: String,
            db: T,
        }
        impl<T> ChgEmpNameTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, new_name: &str, dao: T) -> Self {
                Self {
                    id,
                    new_name: new_name.to_string(),
                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for ChgEmpNameTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> ChgEmpName for ChgEmpNameTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
                self.id
            }
            fn get_new_name(&self) -> &str {
                &self.new_name
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChgEmpNameTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChgEmpNameTx::execute called");
                ChgEmpName::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_name_tx::*;

    mod chg_address_tx {
        use anyhow;
        use log::trace;

        // dao と tx_app のインターフェースにのみ依存 (domain は当然 ok)
        use super::super::ChgEmpAddress;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::EmpId;
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChgEmpAddressTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            new_address: String,
            db: T,
        }
        impl<T> ChgEmpAddressTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, new_address: &str, dao: T) -> Self {
                Self {
                    id,
                    new_address: new_address.to_string(),
                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for ChgEmpAddressTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> ChgEmpAddress for ChgEmpAddressTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
                self.id
            }
            fn get_new_address(&self) -> &str {
                &self.new_address
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChgEmpAddressTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChgEmpAddressTx::execute called");
                ChgEmpAddress::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_address_tx::*;

    mod chg_salary_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        use super::super::ChgClassification;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{EmpId, PaymentClassification};
        use payroll_impl::{MonthlySchedule, SalariedClassification};
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChgSalariedTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            salary: f32,

            db: T,
        }
        impl<T> ChgSalariedTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, salary: f32, dao: T) -> Self {
                Self {
                    id,
                    salary,
                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for ChgSalariedTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> ChgClassification for ChgSalariedTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
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
        impl<T> Transaction for ChgSalariedTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChgSalariedTx::execute called");
                ChgClassification::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_salary_tx::*;

    mod chg_hourly_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        use super::super::ChgClassification;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{EmpId, PaymentClassification};
        use payroll_impl::{HourlyClassification, WeeklySchedule};
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChgHourlyTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            hourly_rate: f32,

            db: T,
        }
        impl<T> ChgHourlyTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, hourly_rate: f32, dao: T) -> Self {
                Self {
                    id,
                    hourly_rate,
                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for ChgHourlyTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> ChgClassification for ChgHourlyTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
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
        impl<T> Transaction for ChgHourlyTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChgHourlyTx::execute called");
                ChgClassification::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_hourly_tx::*;

    mod chg_commissioned_tx {
        use anyhow;
        use log::trace;
        use std::{cell::RefCell, rc::Rc};

        use super::super::ChgClassification;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{EmpId, PaymentClassification};
        use payroll_impl::{BiweeklySchedule, CommissionedClassification};
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChgCommissionedTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            salary: f32,
            commission_rate: f32,

            db: T,
        }
        impl<T> ChgCommissionedTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, salary: f32, commission_rate: f32, dao: T) -> Self {
                Self {
                    id,
                    salary,
                    commission_rate,

                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for ChgCommissionedTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> ChgClassification for ChgCommissionedTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
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
        impl<T> Transaction for ChgCommissionedTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChgCommissionedTx::execute called");
                ChgClassification::execute(self)
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
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{EmpId, PaymentMethod};
        use payroll_impl::HoldMethod;
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChgHoldTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,

            db: T,
        }
        impl<T> ChgHoldTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, dao: T) -> Self {
                Self { id, db: dao }
            }
        }

        impl<T> HaveEmpDao for ChgHoldTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> ChgMethod for ChgHoldTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
                self.id
            }
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
                Rc::new(RefCell::new(HoldMethod))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChgHoldTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChgHoldTx::execute called");
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
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{EmpId, PaymentMethod};
        use payroll_impl::DirectMethod;
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChgDirectTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            bank: String,
            account: String,

            db: T,
        }
        impl<T> ChgDirectTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, bank: &str, account: &str, dao: T) -> Self {
                Self {
                    id,
                    bank: bank.to_string(),
                    account: account.to_string(),
                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for ChgDirectTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> ChgMethod for ChgDirectTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
                self.id
            }
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
                Rc::new(RefCell::new(DirectMethod::new(&self.bank, &self.account)))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChgDirectTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChgDirectTx::execute called");
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
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{EmpId, PaymentMethod};
        use payroll_impl::MailMethod;
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChgMailTx<T>
        where
            T: EmpDao,
        {
            id: EmpId,
            address: String,

            db: T,
        }
        impl<T> ChgMailTx<T>
        where
            T: EmpDao,
        {
            pub fn new(id: EmpId, address: &str, dao: T) -> Self {
                Self {
                    id,
                    address: address.to_string(),

                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for ChgMailTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> ChgMethod for ChgMailTx<T>
        where
            T: EmpDao,
        {
            fn get_id(&self) -> EmpId {
                self.id
            }
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
                Rc::new(RefCell::new(MailMethod::new(&self.address)))
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChgMailTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChgMailTx::execute called");
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

        use super::super::ChgMember;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::{EmpId, MemberId};
        use payroll_impl::UnionAffiliation;
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChgMemberTx<T>
        where
            T: EmpDao,
        {
            member_id: MemberId,
            emp_id: EmpId,
            dues: f32,

            db: T,
        }
        impl<T> ChgMemberTx<T>
        where
            T: EmpDao,
        {
            pub fn new(member_id: MemberId, emp_id: EmpId, dues: f32, dao: T) -> Self {
                Self {
                    member_id,
                    emp_id,
                    dues,
                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for ChgMemberTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> ChgMember for ChgMemberTx<T>
        where
            T: EmpDao,
        {
            fn get_member_id(&self) -> MemberId {
                self.member_id
            }
            fn get_emp_id(&self) -> EmpId {
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
        impl<T> Transaction for ChgMemberTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChgMemberTx::execute called");
                ChgMember::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_member_tx::*;

    mod chg_unaffiliated_tx {
        use anyhow;
        use log::trace;

        use super::super::ChgUnaffiliated;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::EmpId;
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct ChgUnaffiliatedTx<T>
        where
            T: EmpDao,
        {
            emp_id: EmpId,

            db: T,
        }
        impl<T> ChgUnaffiliatedTx<T>
        where
            T: EmpDao,
        {
            pub fn new(emp_id: EmpId, dao: T) -> Self {
                Self { emp_id, db: dao }
            }
        }

        impl<T> HaveEmpDao for ChgUnaffiliatedTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> ChgUnaffiliated for ChgUnaffiliatedTx<T>
        where
            T: EmpDao,
        {
            fn get_emp_id(&self) -> EmpId {
                self.emp_id
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for ChgUnaffiliatedTx<T>
        where
            T: EmpDao,
        {
            fn execute(&self) -> Result<Response, anyhow::Error> {
                trace!("ChgUnaffiliatedTx::execute called");
                ChgUnaffiliated::execute(self)
                    .map(|_| Response::Void)
                    .map_err(Into::into)
            }
        }
    }
    pub use chg_unaffiliated_tx::*;

    mod add_service_charge_tx {
        use anyhow;
        use chrono::NaiveDate;
        use log::trace;

        use super::super::AddServiceCharge;
        use dao::{EmpDao, HaveEmpDao};
        use payroll_domain::MemberId;
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct AddServiceChargeTx<T>
        where
            T: EmpDao,
        {
            member_id: MemberId,
            date: NaiveDate,
            amount: f32,

            db: T,
        }
        impl<T> AddServiceChargeTx<T>
        where
            T: EmpDao,
        {
            pub fn new(member_id: MemberId, date: NaiveDate, amount: f32, dao: T) -> Self {
                Self {
                    member_id,
                    date,
                    amount,
                    db: dao,
                }
            }
        }

        impl<T> HaveEmpDao for AddServiceChargeTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> AddServiceCharge for AddServiceChargeTx<T>
        where
            T: EmpDao,
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
            T: EmpDao,
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

    mod payday_tx {
        use anyhow;
        use chrono::NaiveDate;
        use log::trace;

        use super::super::Payday;
        use dao::{EmpDao, HaveEmpDao};
        use tx_app::{Response, Transaction};

        // ユースケース: ChgEmpName トランザクションの実装 (struct)
        #[derive(Debug)]
        pub struct PaydayTx<T>
        where
            T: EmpDao,
        {
            pay_date: NaiveDate,

            db: T,
        }
        impl<T> PaydayTx<T>
        where
            T: EmpDao,
        {
            pub fn new(pay_date: NaiveDate, dao: T) -> Self {
                Self { pay_date, db: dao }
            }
        }

        impl<T> HaveEmpDao for PaydayTx<T>
        where
            T: EmpDao,
        {
            type Ctx<'a> = T::Ctx<'a>;

            fn dao<'a>(&self) -> &impl EmpDao<Ctx<'a> = Self::Ctx<'a>> {
                &self.db
            }
        }
        impl<T> Payday for PaydayTx<T>
        where
            T: EmpDao,
        {
            fn get_pay_date(&self) -> NaiveDate {
                self.pay_date
            }
        }
        // 共通インターフェースの実装
        impl<T> Transaction for PaydayTx<T>
        where
            T: EmpDao,
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
