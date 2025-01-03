mod payroll_domain {
    mod types {
        pub type EmployeeId = u32;
        pub type MemberId = u32;
    }
    pub use types::*;

    mod bo {
        use chrono::NaiveDate;
        use std::{cell::RefCell, fmt::Debug, ops::RangeInclusive, rc::Rc};

        use super::{
            interface::{Affiliation, PaymentClassification, PaymentMethod, PaymentSchedule},
            types::EmployeeId,
        };

        #[derive(Debug, Clone)]
        pub struct Employee {
            id: EmployeeId,
            name: String,
            address: String,

            classification: Rc<RefCell<dyn PaymentClassification>>,
            schedule: Rc<RefCell<dyn PaymentSchedule>>,
            method: Rc<RefCell<dyn PaymentMethod>>,
            affiliation: Rc<RefCell<dyn Affiliation>>,
        }
        impl Employee {
            pub fn new(
                id: EmployeeId,
                name: &str,
                address: &str,
                classification: Rc<RefCell<dyn PaymentClassification>>,
                schedule: Rc<RefCell<dyn PaymentSchedule>>,
                method: Rc<RefCell<dyn PaymentMethod>>,
                affiliation: Rc<RefCell<dyn Affiliation>>,
            ) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    address: address.to_string(),
                    classification,
                    schedule,
                    method,
                    affiliation,
                }
            }
            pub fn emp_id(&self) -> EmployeeId {
                self.id
            }
            pub fn set_name(&mut self, name: &str) {
                self.name = name.to_string();
            }
            pub fn set_address(&mut self, address: &str) {
                self.address = address.to_string();
            }
            pub fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
                self.classification.clone()
            }
            pub fn set_classification(
                &mut self,
                classification: Rc<RefCell<dyn PaymentClassification>>,
            ) {
                self.classification = classification;
            }
            pub fn set_schedule(&mut self, schedule: Rc<RefCell<dyn PaymentSchedule>>) {
                self.schedule = schedule;
            }
            pub fn set_method(&mut self, method: Rc<RefCell<dyn PaymentMethod>>) {
                self.method = method;
            }
            pub fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>> {
                self.affiliation.clone()
            }
            pub fn set_affiliation(&mut self, affiliation: Rc<RefCell<dyn Affiliation>>) {
                self.affiliation = affiliation;
            }
            pub fn is_pay_date(&self, date: NaiveDate) -> bool {
                self.schedule.borrow().is_pay_date(date)
            }
            pub fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
                self.schedule.borrow().get_pay_period(pay_date)
            }
            pub fn payday(&self, pc: &mut Paycheck) {
                let gross_pay = self.classification.borrow().calculate_pay(pc);
                let deductions = self.affiliation.borrow().calculate_deductions(pc);
                let net_pay = gross_pay - deductions;
                pc.set_gross_pay(gross_pay);
                pc.set_deductions(deductions);
                pc.set_net_pay(net_pay);
                self.method.borrow().pay(pc);
            }
        }

        #[derive(Debug, Clone)]
        pub struct Paycheck {
            period: RangeInclusive<NaiveDate>,

            gross_pay: f32,
            deductions: f32,
            net_pay: f32,
        }
        impl Paycheck {
            pub fn new(period: RangeInclusive<NaiveDate>) -> Self {
                Self {
                    period,
                    gross_pay: 0.0,
                    deductions: 0.0,
                    net_pay: 0.0,
                }
            }
            pub fn set_gross_pay(&mut self, gross_pay: f32) {
                self.gross_pay = gross_pay;
            }
            pub fn set_deductions(&mut self, deductions: f32) {
                self.deductions = deductions;
            }
            pub fn set_net_pay(&mut self, net_pay: f32) {
                self.net_pay = net_pay;
            }
            pub fn get_pay_period(&self) -> RangeInclusive<NaiveDate> {
                self.period.clone()
            }
        }
    }
    pub use bo::*;

    mod interface {
        mod payment_classification {
            use dyn_clone::DynClone;
            use std::{any::Any, fmt::Debug};

            use super::super::bo::Paycheck;

            pub trait PaymentClassification: Debug + DynClone {
                fn as_any(&self) -> &dyn Any;
                fn as_any_mut(&mut self) -> &mut dyn Any;
                fn calculate_pay(&self, pc: &Paycheck) -> f32;
            }
            dyn_clone::clone_trait_object!(PaymentClassification);
        }
        pub use payment_classification::*;

        mod payment_schedule {
            use chrono::NaiveDate;
            use dyn_clone::DynClone;
            use std::{fmt::Debug, ops::RangeInclusive};

            pub trait PaymentSchedule: Debug + DynClone {
                fn is_pay_date(&self, date: NaiveDate) -> bool;
                fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate>;
            }
            dyn_clone::clone_trait_object!(PaymentSchedule);
        }
        pub use payment_schedule::*;

        mod payment_method {
            use dyn_clone::DynClone;
            use std::fmt::Debug;

            use crate::payroll_domain::Paycheck;

            pub trait PaymentMethod: Debug + DynClone {
                // TODO: return type
                fn pay(&self, pc: &Paycheck);
            }
            dyn_clone::clone_trait_object!(PaymentMethod);
        }
        pub use payment_method::*;

        mod affiliation {
            use dyn_clone::DynClone;
            use std::{any::Any, fmt::Debug};

            use crate::payroll_domain::Paycheck;

            pub trait Affiliation: Debug + DynClone {
                fn as_any(&self) -> &dyn Any;
                fn as_any_mut(&mut self) -> &mut dyn Any;
                fn calculate_deductions(&self, pc: &Paycheck) -> f32;
            }
            dyn_clone::clone_trait_object!(Affiliation);

            #[derive(Debug, Clone)]
            pub struct NoAffiliation;
            impl Affiliation for NoAffiliation {
                fn as_any(&self) -> &dyn Any {
                    self
                }
                fn as_any_mut(&mut self) -> &mut dyn Any {
                    self
                }
                fn calculate_deductions(&self, _pc: &Paycheck) -> f32 {
                    0.0
                }
            }
        }
        pub use affiliation::*;
    }
    pub use interface::*;
}

mod payroll_impl {
    mod classification {
        use chrono::NaiveDate;
        use std::any::Any;

        use crate::payroll_domain::{Paycheck, PaymentClassification};

        #[derive(Debug, Clone)]
        pub struct SalariedClassification {
            salary: f32,
        }
        impl SalariedClassification {
            pub fn new(salary: f32) -> Self {
                Self { salary }
            }
        }

        impl PaymentClassification for SalariedClassification {
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
            fn calculate_pay(&self, _pc: &Paycheck) -> f32 {
                self.salary
            }
        }

        #[derive(Debug, Clone)]
        struct TimeCard {
            date: NaiveDate,
            hours: f32,
        }
        impl TimeCard {
            fn new(date: NaiveDate, hours: f32) -> Self {
                Self { date, hours }
            }
            fn get_date(&self) -> NaiveDate {
                self.date
            }
            fn get_hours(&self) -> f32 {
                self.hours
            }
        }

        #[derive(Debug, Clone)]
        pub struct HourlyClassification {
            hourly_rate: f32,
            timecards: Vec<TimeCard>,
        }
        impl HourlyClassification {
            pub fn new(hourly_rate: f32) -> Self {
                Self {
                    hourly_rate,
                    timecards: vec![],
                }
            }
            pub fn add_timecard(&mut self, date: NaiveDate, hours: f32) {
                let tc = TimeCard::new(date, hours);
                self.timecards.push(tc);
            }
            fn calculate_pay_for_timecard(&self, tc: &TimeCard) -> f32 {
                let hours = tc.get_hours();
                let overtime = (hours - 8.0).max(0.0);
                let straight_time = hours - overtime;
                straight_time * self.hourly_rate + overtime * self.hourly_rate * 1.5
            }
        }
        impl PaymentClassification for HourlyClassification {
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
            fn calculate_pay(&self, pc: &Paycheck) -> f32 {
                let pay_period = pc.get_pay_period();
                let mut total_pay = 0.0;
                for tc in &self.timecards {
                    if pay_period.contains(&tc.get_date()) {
                        total_pay += self.calculate_pay_for_timecard(tc);
                    }
                }
                total_pay
            }
        }

        #[derive(Debug, Clone)]
        struct SalesReceipt {
            date: NaiveDate,
            amount: f32,
        }
        impl SalesReceipt {
            fn new(date: NaiveDate, amount: f32) -> Self {
                Self { date, amount }
            }
            fn get_date(&self) -> NaiveDate {
                self.date
            }
            fn get_amount(&self) -> f32 {
                self.amount
            }
        }

        #[derive(Debug, Clone)]
        pub struct CommissionedClassification {
            salary: f32,
            commission_rate: f32,
            sales_receipts: Vec<SalesReceipt>,
        }
        impl CommissionedClassification {
            pub fn new(salary: f32, commission_rate: f32) -> Self {
                Self {
                    salary,
                    commission_rate,
                    sales_receipts: vec![],
                }
            }
            pub fn add_sales_receipt(&mut self, date: NaiveDate, amount: f32) {
                let sr = SalesReceipt::new(date, amount);
                self.sales_receipts.push(sr);
            }
            fn calculate_pay_for_sales_receipt(&self, sr: &SalesReceipt) -> f32 {
                self.commission_rate * sr.get_amount()
            }
        }
        impl PaymentClassification for CommissionedClassification {
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
            fn calculate_pay(&self, pc: &Paycheck) -> f32 {
                let mut total_pay = self.salary;
                let pay_period = pc.get_pay_period();
                for sr in self.sales_receipts.iter() {
                    if pay_period.contains(&sr.get_date()) {
                        total_pay += self.calculate_pay_for_sales_receipt(sr);
                    }
                }
                total_pay
            }
        }
    }
    pub use classification::*;

    mod schedule {
        use chrono::{Datelike, Days, NaiveDate, Weekday};
        use std::{fmt::Debug, ops::RangeInclusive};

        use crate::payroll_domain::PaymentSchedule;

        #[derive(Debug, Clone)]
        pub struct MonthlySchedule;
        impl MonthlySchedule {
            pub fn is_last_day_of_month(&self, date: NaiveDate) -> bool {
                date.month() != date.checked_add_days(Days::new(1)).unwrap().month()
            }
        }
        impl PaymentSchedule for MonthlySchedule {
            fn is_pay_date(&self, date: NaiveDate) -> bool {
                self.is_last_day_of_month(date)
            }
            fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
                pay_date.with_day(1).unwrap()..=pay_date
            }
        }

        #[derive(Debug, Clone)]
        pub struct WeeklySchedule;
        impl PaymentSchedule for WeeklySchedule {
            fn is_pay_date(&self, date: NaiveDate) -> bool {
                date.weekday() == Weekday::Fri
            }
            fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
                pay_date.checked_sub_days(Days::new(6)).unwrap()..=pay_date
            }
        }

        #[derive(Debug, Clone)]
        pub struct BiweeklySchedule;
        impl PaymentSchedule for BiweeklySchedule {
            fn is_pay_date(&self, date: NaiveDate) -> bool {
                date.weekday() == Weekday::Fri && date.iso_week().week() % 2 == 0
            }
            fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
                pay_date.checked_sub_days(Days::new(13)).unwrap()..=pay_date
            }
        }
    }
    pub use schedule::*;

    mod method {
        use crate::payroll_domain::{Paycheck, PaymentMethod};

        #[derive(Debug, Clone)]
        pub struct HoldMethod;
        impl PaymentMethod for HoldMethod {
            fn pay(&self, pc: &Paycheck) {
                println!("HoldMethod: {:#?}", pc);
            }
        }

        #[derive(Debug, Clone)]
        pub struct DirectMethod {
            bank: String,
            account: String,
        }
        impl DirectMethod {
            pub fn new(bank: &str, account: &str) -> Self {
                Self {
                    bank: bank.to_string(),
                    account: account.to_string(),
                }
            }
        }
        impl PaymentMethod for DirectMethod {
            fn pay(&self, pc: &Paycheck) {
                println!("DirectMethod to {}{}: {:#?}", self.bank, self.account, pc);
            }
        }

        #[derive(Debug, Clone)]
        pub struct MailMethod {
            address: String,
        }
        impl MailMethod {
            pub fn new(address: &str) -> Self {
                Self {
                    address: address.to_string(),
                }
            }
        }
        impl PaymentMethod for MailMethod {
            fn pay(&self, pc: &Paycheck) {
                println!("MailMethod to {}: {:#?}", self.address, pc);
            }
        }
    }
    pub use method::*;

    mod affiliation {
        use chrono::{Datelike, NaiveDate, Weekday};
        use std::any::Any;

        use crate::payroll_domain::{Affiliation, MemberId, Paycheck};

        #[derive(Debug, Clone)]
        struct ServiceCharge {
            date: NaiveDate,
            amount: f32,
        }
        impl ServiceCharge {
            fn new(date: NaiveDate, amount: f32) -> Self {
                Self { date, amount }
            }
            fn get_amount(&self) -> f32 {
                self.amount
            }
        }

        #[derive(Debug, Clone)]
        pub struct UnionAffiliation {
            member_id: MemberId,
            dues: f32,
            service_charges: Vec<ServiceCharge>,
        }
        impl UnionAffiliation {
            pub fn new(member_id: MemberId, dues: f32) -> Self {
                Self {
                    member_id,
                    dues,
                    service_charges: vec![],
                }
            }
            pub fn get_member_id(&self) -> MemberId {
                self.member_id
            }
            pub fn add_service_charge(&mut self, date: NaiveDate, amount: f32) {
                let sc = ServiceCharge::new(date, amount);
                self.service_charges.push(sc);
            }
        }
        impl Affiliation for UnionAffiliation {
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
            fn calculate_deductions(&self, pc: &Paycheck) -> f32 {
                let mut total_deductions = 0.0;
                let pay_period = pc.get_pay_period();
                for d in pc.get_pay_period().start().iter_days() {
                    if d > *pay_period.end() {
                        break;
                    }
                    if d.weekday() == Weekday::Fri {
                        total_deductions += self.dues;
                    }
                }
                for sc in self.service_charges.iter() {
                    if pay_period.contains(&sc.date) {
                        total_deductions += sc.get_amount();
                    }
                }
                total_deductions
            }
        }
    }
    pub use affiliation::*;
}

mod dao {
    use thiserror::Error;

    use crate::payroll_domain::{Employee, EmployeeId, MemberId, Paycheck};

    #[derive(Debug, Clone, Eq, PartialEq, Error)]
    pub enum DaoError {
        #[error("EmployeeId({0}) already exists")]
        AlreadyExists(EmployeeId),
        #[error("EmployeeId({0}) not found")]
        NotFound(EmployeeId),
        #[error("MemberId({0}) is already a union member of EmployeeId({1})")]
        AlreadyUnionMember(MemberId, EmployeeId),
        #[error("MemberId({0}) is not a union member")]
        NotYetUnionMember(MemberId),
    }

    pub trait EmployeeDao<Ctx> {
        fn insert(&self, emp: Employee) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = DaoError>;
        fn remove(&self, emp_id: EmployeeId) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
        fn fetch(&self, emp_id: EmployeeId)
            -> impl tx_rs::Tx<Ctx, Item = Employee, Err = DaoError>;
        fn fetch_all(
            &self,
        ) -> impl tx_rs::Tx<Ctx, Item = Vec<(EmployeeId, Employee)>, Err = DaoError>;
        fn update(&self, emp: Employee) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
        fn add_union_member(
            &self,
            member_id: MemberId,
            emp_id: EmployeeId,
        ) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
        fn remove_union_member(
            &self,
            member_id: MemberId,
        ) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
        fn find_union_member(
            &self,
            member_id: MemberId,
        ) -> impl tx_rs::Tx<Ctx, Item = EmployeeId, Err = DaoError>;
        fn record_paycheck(
            &self,
            emp_id: EmployeeId,
            paycheck: Paycheck,
        ) -> impl tx_rs::Tx<Ctx, Item = (), Err = DaoError>;
    }

    pub trait HaveEmployeeDao<Ctx> {
        fn dao(&self) -> &impl EmployeeDao<Ctx>;
    }
}

mod usecase {
    use chrono::NaiveDate;
    use std::{cell::RefCell, rc::Rc};
    use thiserror::Error;
    use tx_rs::Tx;

    use crate::{
        dao::{DaoError, EmployeeDao, HaveEmployeeDao},
        payroll_domain::{
            Affiliation, Employee, EmployeeId, MemberId, NoAffiliation, Paycheck,
            PaymentClassification, PaymentMethod, PaymentSchedule,
        },
        payroll_impl::{
            CommissionedClassification, HoldMethod, HourlyClassification, UnionAffiliation,
        },
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
}

mod service {
    use thiserror::Error;
    use tx_rs::Tx;

    use crate::{
        payroll_domain::EmployeeId,
        usecase::{
            AddEmployee, AddSalesReceipt, AddServiceCharge, AddTimeCard, AddUnionAffiliation,
            ChgClassification, ChgEmployeeAddress, ChgEmployeeName, ChgMethod, DelEmployee,
            DelUnionAffiliation, Payday, UsecaseError,
        },
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
}

mod payroll_db {
    use std::{cell::RefCell, cell::RefMut, collections::HashMap, fmt::Debug, rc::Rc};

    use crate::{
        dao::{DaoError, EmployeeDao},
        payroll_domain::{Employee, EmployeeId, MemberId, Paycheck},
    };

    #[derive(Debug, Clone)]
    pub struct PayrollDatabase {
        employees: HashMap<EmployeeId, Employee>,
        union_members: HashMap<MemberId, EmployeeId>,
        paychecks: Rc<RefCell<HashMap<EmployeeId, Vec<Paycheck>>>>,
    }
    impl PayrollDatabase {
        pub fn new() -> Self {
            Self {
                employees: HashMap::new(),
                union_members: HashMap::new(),
                paychecks: Rc::new(RefCell::new(HashMap::new())),
            }
        }
    }
    pub type PayrollDbCtx<'a> = RefMut<'a, PayrollDatabase>;

    #[derive(Debug, Clone)]
    pub struct PayrollDbDao;
    impl<'a> EmployeeDao<PayrollDbCtx<'a>> for PayrollDbDao {
        fn insert(
            &self,
            emp: Employee,
        ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = EmployeeId, Err = DaoError> {
            tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
                let emp_id = emp.emp_id();
                if tx.employees.contains_key(&emp_id) {
                    Err(DaoError::AlreadyExists(emp_id))
                } else {
                    tx.employees.insert(emp_id, emp);
                    Ok(emp_id)
                }
            })
        }
        fn remove(
            &self,
            emp_id: EmployeeId,
        ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = (), Err = DaoError> {
            tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
                if tx.employees.contains_key(&emp_id) {
                    tx.employees.remove(&emp_id);
                    Ok(())
                } else {
                    Err(DaoError::NotFound(emp_id))
                }
            })
        }
        fn fetch(
            &self,
            emp_id: EmployeeId,
        ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = Employee, Err = DaoError> {
            tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
                tx.employees
                    .get(&emp_id)
                    .cloned()
                    .ok_or(DaoError::NotFound(emp_id))
            })
        }
        fn fetch_all(
            &self,
        ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = Vec<(EmployeeId, Employee)>, Err = DaoError>
        {
            tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
                Ok(tx.employees.iter().map(|(k, v)| (*k, v.clone())).collect())
            })
        }
        fn update(
            &self,
            emp: Employee,
        ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = (), Err = DaoError> {
            tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
                let emp_id = emp.emp_id();
                if tx.employees.contains_key(&emp_id) {
                    tx.employees.insert(emp_id, emp);
                    Ok(())
                } else {
                    Err(DaoError::NotFound(emp_id))
                }
            })
        }
        fn add_union_member(
            &self,
            member_id: MemberId,
            emp_id: EmployeeId,
        ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = (), Err = DaoError> {
            tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
                if tx.union_members.contains_key(&member_id) {
                    return Err(DaoError::AlreadyUnionMember(member_id, emp_id));
                }
                tx.union_members.insert(member_id, emp_id);
                Ok(())
            })
        }
        fn remove_union_member(
            &self,
            member_id: MemberId,
        ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = (), Err = DaoError> {
            tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
                if tx.union_members.remove(&member_id).is_none() {
                    return Err(DaoError::NotYetUnionMember(member_id));
                }
                Ok(())
            })
        }
        fn find_union_member(
            &self,
            member_id: MemberId,
        ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = EmployeeId, Err = DaoError> {
            tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
                tx.union_members
                    .get(&member_id)
                    .cloned()
                    .ok_or(DaoError::NotYetUnionMember(member_id))
            })
        }
        fn record_paycheck(
            &self,
            emp_id: EmployeeId,
            pc: Paycheck,
        ) -> impl tx_rs::Tx<PayrollDbCtx<'a>, Item = (), Err = DaoError> {
            tx_rs::with_tx(move |tx: &mut PayrollDbCtx<'a>| {
                tx.paychecks
                    .borrow_mut()
                    .entry(emp_id)
                    .or_insert(vec![])
                    .push(pc);
                Ok(())
            })
        }
    }
}

mod usecase_impl {
    mod add_salaried_emp {
        use std::{cell::RefCell, fmt::Debug, rc::Rc};

        use crate::{
            dao::{EmployeeDao, HaveEmployeeDao},
            payroll_db::{PayrollDbCtx, PayrollDbDao},
            payroll_domain::{EmployeeId, PaymentClassification, PaymentSchedule},
            payroll_impl::{MonthlySchedule, SalariedClassification},
            usecase::AddEmployee,
        };

        #[derive(Debug, Clone)]
        pub struct AddSalariedEmployeeImpl {
            id: EmployeeId,
            name: String,
            address: String,
            salary: f32,

            dao: PayrollDbDao,
        }
        impl AddSalariedEmployeeImpl {
            pub fn new(id: EmployeeId, name: &str, address: &str, salary: f32) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    address: address.to_string(),
                    salary,

                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for AddSalariedEmployeeImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> AddEmployee<PayrollDbCtx<'a>> for AddSalariedEmployeeImpl {
            fn get_emp_id(&self) -> EmployeeId {
                self.id
            }
            fn get_name(&self) -> &str {
                self.name.as_str()
            }
            fn get_address(&self) -> &str {
                self.address.as_str()
            }
            fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
                Rc::new(RefCell::new(SalariedClassification::new(self.salary)))
            }
            fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
                Rc::new(RefCell::new(MonthlySchedule))
            }
        }
    }
    pub use add_salaried_emp::*;

    mod add_hourly_emp {
        use std::{cell::RefCell, fmt::Debug, rc::Rc};

        use crate::{
            dao::{EmployeeDao, HaveEmployeeDao},
            payroll_db::{PayrollDbCtx, PayrollDbDao},
            payroll_domain::{EmployeeId, PaymentClassification, PaymentSchedule},
            payroll_impl::{HourlyClassification, WeeklySchedule},
            usecase::AddEmployee,
        };

        #[derive(Debug, Clone)]
        pub struct AddHourlyEmployeeImpl {
            id: EmployeeId,
            name: String,
            address: String,
            hourly_rate: f32,

            dao: PayrollDbDao,
        }
        impl AddHourlyEmployeeImpl {
            pub fn new(id: EmployeeId, name: &str, address: &str, hourly_rate: f32) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    address: address.to_string(),
                    hourly_rate,

                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for AddHourlyEmployeeImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> AddEmployee<PayrollDbCtx<'a>> for AddHourlyEmployeeImpl {
            fn get_emp_id(&self) -> EmployeeId {
                self.id
            }
            fn get_name(&self) -> &str {
                self.name.as_str()
            }
            fn get_address(&self) -> &str {
                self.address.as_str()
            }
            fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
                Rc::new(RefCell::new(HourlyClassification::new(self.hourly_rate)))
            }
            fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
                Rc::new(RefCell::new(WeeklySchedule))
            }
        }
    }
    pub use add_hourly_emp::*;

    mod add_commissioned_emp {
        use std::{cell::RefCell, fmt::Debug, rc::Rc};

        use crate::{
            dao::{EmployeeDao, HaveEmployeeDao},
            payroll_db::{PayrollDbCtx, PayrollDbDao},
            payroll_domain::{EmployeeId, PaymentClassification, PaymentSchedule},
            payroll_impl::{BiweeklySchedule, CommissionedClassification},
            usecase::AddEmployee,
        };

        #[derive(Debug, Clone)]
        pub struct AddCommissionedEmployeeImpl {
            id: EmployeeId,
            name: String,
            address: String,
            salary: f32,
            commission_rate: f32,

            dao: PayrollDbDao,
        }
        impl AddCommissionedEmployeeImpl {
            pub fn new(
                id: EmployeeId,
                name: &str,
                address: &str,
                salary: f32,
                commission_rate: f32,
            ) -> Self {
                Self {
                    id,
                    name: name.to_string(),
                    address: address.to_string(),
                    salary,
                    commission_rate,

                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for AddCommissionedEmployeeImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> AddEmployee<PayrollDbCtx<'a>> for AddCommissionedEmployeeImpl {
            fn get_emp_id(&self) -> EmployeeId {
                self.id
            }
            fn get_name(&self) -> &str {
                self.name.as_str()
            }
            fn get_address(&self) -> &str {
                self.address.as_str()
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
        }
    }
    pub use add_commissioned_emp::*;

    mod chg_emp_name {
        use std::fmt::Debug;

        use crate::{
            dao::{EmployeeDao, HaveEmployeeDao},
            payroll_db::{PayrollDbCtx, PayrollDbDao},
            payroll_domain::EmployeeId,
            usecase::ChgEmployeeName,
        };

        #[derive(Debug, Clone)]
        pub struct ChgEmployeeNameImpl {
            id: EmployeeId,
            new_name: String,

            dao: PayrollDbDao,
        }
        impl ChgEmployeeNameImpl {
            pub fn new(id: EmployeeId, new_name: &str) -> Self {
                Self {
                    id,
                    new_name: new_name.to_string(),

                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for ChgEmployeeNameImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> ChgEmployeeName<PayrollDbCtx<'a>> for ChgEmployeeNameImpl {
            fn get_emp_id(&self) -> EmployeeId {
                self.id
            }
            fn get_name(&self) -> &str {
                self.new_name.as_str()
            }
        }
    }
    pub use chg_emp_name::*;

    mod chg_emp_address {
        use std::fmt::Debug;

        use crate::{
            dao::{EmployeeDao, HaveEmployeeDao},
            payroll_db::{PayrollDbCtx, PayrollDbDao},
            payroll_domain::EmployeeId,
            usecase::ChgEmployeeAddress,
        };

        #[derive(Debug, Clone)]
        pub struct ChgEmployeeAddressImpl {
            id: EmployeeId,
            new_address: String,

            dao: PayrollDbDao,
        }
        impl ChgEmployeeAddressImpl {
            pub fn new(id: EmployeeId, new_address: &str) -> Self {
                Self {
                    id,
                    new_address: new_address.to_string(),

                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for ChgEmployeeAddressImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> ChgEmployeeAddress<PayrollDbCtx<'a>> for ChgEmployeeAddressImpl {
            fn get_emp_id(&self) -> EmployeeId {
                self.id
            }
            fn get_address(&self) -> &str {
                self.new_address.as_str()
            }
        }
    }
    pub use chg_emp_address::*;

    mod del_emp {
        use std::fmt::Debug;

        use crate::{
            dao::{EmployeeDao, HaveEmployeeDao},
            payroll_db::{PayrollDbCtx, PayrollDbDao},
            payroll_domain::EmployeeId,
            usecase::DelEmployee,
        };

        #[derive(Debug, Clone)]
        pub struct DelEmployeeImpl {
            id: EmployeeId,

            dao: PayrollDbDao,
        }
        impl DelEmployeeImpl {
            pub fn new(id: EmployeeId) -> Self {
                Self {
                    id,

                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for DelEmployeeImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> DelEmployee<PayrollDbCtx<'a>> for DelEmployeeImpl {
            fn get_emp_id(&self) -> EmployeeId {
                self.id
            }
        }
    }
    pub use del_emp::*;

    mod chg_salaried_emp {
        use std::{cell::RefCell, fmt::Debug, rc::Rc};

        use crate::{
            dao::{EmployeeDao, HaveEmployeeDao},
            payroll_db::{PayrollDbCtx, PayrollDbDao},
            payroll_domain::{EmployeeId, PaymentClassification, PaymentSchedule},
            payroll_impl::{MonthlySchedule, SalariedClassification},
            usecase::ChgClassification,
        };

        #[derive(Debug, Clone)]
        pub struct ChgSalariedEmployeeImpl {
            id: EmployeeId,
            salary: f32,

            dao: PayrollDbDao,
        }
        impl ChgSalariedEmployeeImpl {
            pub fn new(id: EmployeeId, salary: f32) -> Self {
                Self {
                    id,
                    salary,

                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for ChgSalariedEmployeeImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> ChgClassification<PayrollDbCtx<'a>> for ChgSalariedEmployeeImpl {
            fn get_emp_id(&self) -> EmployeeId {
                self.id
            }
            fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
                Rc::new(RefCell::new(SalariedClassification::new(self.salary)))
            }
            fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
                Rc::new(RefCell::new(MonthlySchedule))
            }
        }
    }
    pub use chg_salaried_emp::*;

    mod chg_hourly_emp {
        use std::{cell::RefCell, fmt::Debug, rc::Rc};

        use crate::{
            dao::{EmployeeDao, HaveEmployeeDao},
            payroll_db::{PayrollDbCtx, PayrollDbDao},
            payroll_domain::{EmployeeId, PaymentClassification, PaymentSchedule},
            payroll_impl::{HourlyClassification, WeeklySchedule},
            usecase::ChgClassification,
        };

        #[derive(Debug, Clone)]
        pub struct ChgHourlyEmployeeImpl {
            id: EmployeeId,
            hourly_rate: f32,

            dao: PayrollDbDao,
        }
        impl ChgHourlyEmployeeImpl {
            pub fn new(id: EmployeeId, hourly_rate: f32) -> Self {
                Self {
                    id,
                    hourly_rate,

                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for ChgHourlyEmployeeImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> ChgClassification<PayrollDbCtx<'a>> for ChgHourlyEmployeeImpl {
            fn get_emp_id(&self) -> EmployeeId {
                self.id
            }
            fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
                Rc::new(RefCell::new(HourlyClassification::new(self.hourly_rate)))
            }
            fn get_schedule(&self) -> Rc<RefCell<dyn PaymentSchedule>> {
                Rc::new(RefCell::new(WeeklySchedule))
            }
        }
    }
    pub use chg_hourly_emp::*;

    mod chg_commissioned_emp {
        use std::{cell::RefCell, fmt::Debug, rc::Rc};

        use crate::{
            dao::{EmployeeDao, HaveEmployeeDao},
            payroll_db::{PayrollDbCtx, PayrollDbDao},
            payroll_domain::{EmployeeId, PaymentClassification, PaymentSchedule},
            payroll_impl::{BiweeklySchedule, CommissionedClassification},
            usecase::ChgClassification,
        };

        #[derive(Debug, Clone)]
        pub struct ChgCommissionedEmployeeImpl {
            id: EmployeeId,
            salary: f32,
            commission_rate: f32,

            dao: PayrollDbDao,
        }
        impl ChgCommissionedEmployeeImpl {
            pub fn new(id: EmployeeId, salary: f32, commission_rate: f32) -> Self {
                Self {
                    id,
                    salary,
                    commission_rate,

                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for ChgCommissionedEmployeeImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> ChgClassification<PayrollDbCtx<'a>> for ChgCommissionedEmployeeImpl {
            fn get_emp_id(&self) -> EmployeeId {
                self.id
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
        }
    }
    pub use chg_commissioned_emp::*;

    mod chg_hold_method {
        use std::{cell::RefCell, fmt::Debug, rc::Rc};

        use crate::{
            dao::{EmployeeDao, HaveEmployeeDao},
            payroll_db::{PayrollDbCtx, PayrollDbDao},
            payroll_domain::{EmployeeId, PaymentMethod},
            payroll_impl::HoldMethod,
            usecase::ChgMethod,
        };

        #[derive(Debug, Clone)]
        pub struct ChgHoldMethodImpl {
            id: EmployeeId,

            dao: PayrollDbDao,
        }
        impl ChgHoldMethodImpl {
            pub fn new(id: EmployeeId) -> Self {
                Self {
                    id,

                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for ChgHoldMethodImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> ChgMethod<PayrollDbCtx<'a>> for ChgHoldMethodImpl {
            fn get_emp_id(&self) -> EmployeeId {
                self.id
            }
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
                Rc::new(RefCell::new(HoldMethod))
            }
        }
    }
    pub use chg_hold_method::*;

    mod chg_direct_method {
        use std::{cell::RefCell, fmt::Debug, rc::Rc};

        use crate::{
            dao::{EmployeeDao, HaveEmployeeDao},
            payroll_db::{PayrollDbCtx, PayrollDbDao},
            payroll_domain::{EmployeeId, PaymentMethod},
            payroll_impl::DirectMethod,
            usecase::ChgMethod,
        };

        #[derive(Debug, Clone)]
        pub struct ChgDirectMethodImpl {
            id: EmployeeId,
            bank: String,
            account: String,

            dao: PayrollDbDao,
        }
        impl ChgDirectMethodImpl {
            pub fn new(id: EmployeeId, bank: &str, account: &str) -> Self {
                Self {
                    id,
                    bank: bank.to_string(),
                    account: account.to_string(),

                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for ChgDirectMethodImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> ChgMethod<PayrollDbCtx<'a>> for ChgDirectMethodImpl {
            fn get_emp_id(&self) -> EmployeeId {
                self.id
            }
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
                Rc::new(RefCell::new(DirectMethod::new(&self.bank, &self.account)))
            }
        }
    }
    pub use chg_direct_method::*;

    mod chg_mail_method {
        use std::{cell::RefCell, fmt::Debug, rc::Rc};

        use crate::{
            dao::{EmployeeDao, HaveEmployeeDao},
            payroll_db::{PayrollDbCtx, PayrollDbDao},
            payroll_domain::{EmployeeId, PaymentMethod},
            payroll_impl::MailMethod,
            usecase::ChgMethod,
        };

        #[derive(Debug, Clone)]
        pub struct ChgMailMethodImpl {
            id: EmployeeId,
            address: String,

            dao: PayrollDbDao,
        }
        impl ChgMailMethodImpl {
            pub fn new(id: EmployeeId, address: &str) -> Self {
                Self {
                    id,
                    address: address.to_string(),

                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for ChgMailMethodImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> ChgMethod<PayrollDbCtx<'a>> for ChgMailMethodImpl {
            fn get_emp_id(&self) -> EmployeeId {
                self.id
            }
            fn get_method(&self) -> Rc<RefCell<dyn PaymentMethod>> {
                Rc::new(RefCell::new(MailMethod::new(&self.address)))
            }
        }
    }
    pub use chg_mail_method::*;

    mod add_union_member {
        use std::{cell::RefCell, fmt::Debug, rc::Rc};

        use crate::{
            dao::{EmployeeDao, HaveEmployeeDao},
            payroll_db::{PayrollDbCtx, PayrollDbDao},
            payroll_domain::{Affiliation, EmployeeId, MemberId},
            payroll_impl::UnionAffiliation,
            usecase::AddUnionAffiliation,
        };

        #[derive(Debug, Clone)]
        pub struct AddUnionMemberImpl {
            member_id: MemberId,
            emp_id: EmployeeId,
            dues: f32,

            dao: PayrollDbDao,
        }
        impl AddUnionMemberImpl {
            pub fn new(member_id: MemberId, emp_id: EmployeeId, dues: f32) -> Self {
                Self {
                    member_id,
                    emp_id,
                    dues,

                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for AddUnionMemberImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> AddUnionAffiliation<PayrollDbCtx<'a>> for AddUnionMemberImpl {
            fn get_member_id(&self) -> MemberId {
                self.member_id
            }
            fn get_emp_id(&self) -> EmployeeId {
                self.emp_id
            }
            fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>> {
                Rc::new(RefCell::new(UnionAffiliation::new(
                    self.member_id,
                    self.dues,
                )))
            }
        }
    }
    pub use add_union_member::*;

    mod del_union_member {
        use std::{cell::RefCell, fmt::Debug, rc::Rc};

        use crate::{
            dao::{EmployeeDao, HaveEmployeeDao},
            payroll_db::{PayrollDbCtx, PayrollDbDao},
            payroll_domain::{Affiliation, EmployeeId, NoAffiliation},
            usecase::DelUnionAffiliation,
        };

        #[derive(Debug, Clone)]
        pub struct DelUnionMemberImpl {
            emp_id: EmployeeId,

            dao: PayrollDbDao,
        }
        impl DelUnionMemberImpl {
            pub fn new(emp_id: EmployeeId) -> Self {
                Self {
                    emp_id,

                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for DelUnionMemberImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> DelUnionAffiliation<PayrollDbCtx<'a>> for DelUnionMemberImpl {
            fn get_emp_id(&self) -> EmployeeId {
                self.emp_id
            }
            fn get_affiliation(&self) -> Rc<RefCell<dyn Affiliation>> {
                Rc::new(RefCell::new(NoAffiliation))
            }
        }
    }
    pub use del_union_member::*;

    mod timecard {
        use chrono::NaiveDate;

        use crate::{
            dao::{EmployeeDao, HaveEmployeeDao},
            payroll_db::{PayrollDbCtx, PayrollDbDao},
            payroll_domain::EmployeeId,
            usecase::AddTimeCard,
        };

        #[derive(Debug, Clone)]
        pub struct AddTimecardImpl {
            emp_id: EmployeeId,
            date: NaiveDate,
            hours: f32,

            dao: PayrollDbDao,
        }
        impl AddTimecardImpl {
            pub fn new(emp_id: EmployeeId, date: NaiveDate, hours: f32) -> Self {
                Self {
                    emp_id,
                    date,
                    hours,
                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for AddTimecardImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> AddTimeCard<PayrollDbCtx<'a>> for AddTimecardImpl {
            fn get_emp_id(&self) -> EmployeeId {
                self.emp_id
            }
            fn get_date(&self) -> NaiveDate {
                self.date
            }
            fn get_hours(&self) -> f32 {
                self.hours
            }
        }
    }
    pub use timecard::*;

    mod sales_receipt {
        use chrono::NaiveDate;

        use crate::{
            dao::{EmployeeDao, HaveEmployeeDao},
            payroll_db::{PayrollDbCtx, PayrollDbDao},
            payroll_domain::EmployeeId,
            usecase::AddSalesReceipt,
        };

        #[derive(Debug, Clone)]
        pub struct AddSalesReceiptImpl {
            emp_id: EmployeeId,
            date: NaiveDate,
            amount: f32,

            dao: PayrollDbDao,
        }
        impl AddSalesReceiptImpl {
            pub fn new(emp_id: EmployeeId, date: NaiveDate, amount: f32) -> Self {
                Self {
                    emp_id,
                    date,
                    amount,
                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for AddSalesReceiptImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> AddSalesReceipt<PayrollDbCtx<'a>> for AddSalesReceiptImpl {
            fn get_emp_id(&self) -> EmployeeId {
                self.emp_id
            }
            fn get_date(&self) -> NaiveDate {
                self.date
            }
            fn get_amount(&self) -> f32 {
                self.amount
            }
        }
    }
    pub use sales_receipt::*;

    mod service_charge {
        use chrono::NaiveDate;

        use crate::{
            dao::{EmployeeDao, HaveEmployeeDao},
            payroll_db::{PayrollDbCtx, PayrollDbDao},
            payroll_domain::{EmployeeId, MemberId},
            usecase::AddServiceCharge,
        };

        #[derive(Debug, Clone)]
        pub struct AddServiceChargeImpl {
            member_id: MemberId,
            date: NaiveDate,
            amount: f32,

            dao: PayrollDbDao,
        }
        impl AddServiceChargeImpl {
            pub fn new(member_id: MemberId, date: NaiveDate, amount: f32) -> Self {
                Self {
                    member_id,
                    date,
                    amount,
                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for AddServiceChargeImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> AddServiceCharge<PayrollDbCtx<'a>> for AddServiceChargeImpl {
            fn get_member_id(&self) -> EmployeeId {
                self.member_id
            }
            fn get_date(&self) -> NaiveDate {
                self.date
            }
            fn get_amount(&self) -> f32 {
                self.amount
            }
        }
    }
    pub use service_charge::*;

    mod payday {
        use chrono::NaiveDate;

        use crate::{
            dao::{EmployeeDao, HaveEmployeeDao},
            payroll_db::{PayrollDbCtx, PayrollDbDao},
            usecase::Payday,
        };

        #[derive(Debug, Clone)]
        pub struct PaydayImpl {
            pay_date: NaiveDate,

            dao: PayrollDbDao,
        }
        impl PaydayImpl {
            pub fn new(pay_date: NaiveDate) -> Self {
                Self {
                    pay_date,
                    dao: PayrollDbDao,
                }
            }
        }
        impl<'a> HaveEmployeeDao<PayrollDbCtx<'a>> for PaydayImpl {
            fn dao(&self) -> &impl EmployeeDao<PayrollDbCtx<'a>> {
                &self.dao
            }
        }
        impl<'a> Payday<PayrollDbCtx<'a>> for PaydayImpl {
            fn get_pay_date(&self) -> NaiveDate {
                self.pay_date
            }
        }
    }
    pub use payday::*;
}

mod service_impl {
    mod add_salaried_emp {
        use std::{cell::RefCell, fmt::Debug, rc::Rc};

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            payroll_domain::EmployeeId,
            service::{AddEmployeeTransaction, ServiceError, Transaction},
            usecase::UsecaseError,
            usecase_impl::AddSalariedEmployeeImpl,
        };

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

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            payroll_domain::EmployeeId,
            service::{AddEmployeeTransaction, ServiceError, Transaction},
            usecase::UsecaseError,
            usecase_impl::AddHourlyEmployeeImpl,
        };

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
                    usecase: RefCell::new(AddHourlyEmployeeImpl::new(
                        id,
                        name,
                        address,
                        hourly_rate,
                    )),
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

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            payroll_domain::EmployeeId,
            service::{AddEmployeeTransaction, ServiceError, Transaction},
            usecase::UsecaseError,
            usecase_impl::AddCommissionedEmployeeImpl,
        };

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

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            payroll_domain::EmployeeId,
            service::{ChgEmployeeNameTransaction, ServiceError, Transaction},
            usecase::UsecaseError,
            usecase_impl::ChgEmployeeNameImpl,
        };

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

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            payroll_domain::EmployeeId,
            service::{ChgEmployeeAddressTransaction, ServiceError, Transaction},
            usecase::UsecaseError,
            usecase_impl::ChgEmployeeAddressImpl,
        };

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

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            payroll_domain::EmployeeId,
            service::{DelEmployeeTransaction, ServiceError, Transaction},
            usecase::UsecaseError,
            usecase_impl::DelEmployeeImpl,
        };

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

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            payroll_domain::EmployeeId,
            service::{ChgClassificationTransaction, ServiceError, Transaction},
            usecase::UsecaseError,
            usecase_impl::ChgSalariedEmployeeImpl,
        };

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

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            payroll_domain::EmployeeId,
            service::{ChgClassificationTransaction, ServiceError, Transaction},
            usecase::UsecaseError,
            usecase_impl::ChgHourlyEmployeeImpl,
        };

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

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            payroll_domain::EmployeeId,
            service::{ChgClassificationTransaction, ServiceError, Transaction},
            usecase::UsecaseError,
            usecase_impl::ChgCommissionedEmployeeImpl,
        };

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

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            payroll_domain::EmployeeId,
            service::{ChgMethodTransaction, ServiceError, Transaction},
            usecase::UsecaseError,
            usecase_impl::ChgHoldMethodImpl,
        };

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

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            payroll_domain::EmployeeId,
            service::{ChgMethodTransaction, ServiceError, Transaction},
            usecase::UsecaseError,
            usecase_impl::ChgDirectMethodImpl,
        };

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

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            payroll_domain::EmployeeId,
            service::{ChgMethodTransaction, ServiceError, Transaction},
            usecase::UsecaseError,
            usecase_impl::ChgMailMethodImpl,
        };

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

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            payroll_domain::EmployeeId,
            service::{AddUnionAffiliationTransaction, ServiceError, Transaction},
            usecase::UsecaseError,
            usecase_impl::AddUnionMemberImpl,
        };

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

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            payroll_domain::EmployeeId,
            service::{DelUnionAffiliationTransaction, ServiceError, Transaction},
            usecase::UsecaseError,
            usecase_impl::DelUnionMemberImpl,
        };

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

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            payroll_domain::EmployeeId,
            service::{AddTimeCardTransaction, ServiceError, Transaction},
            usecase::UsecaseError,
            usecase_impl::AddTimecardImpl,
        };

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

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            payroll_domain::EmployeeId,
            service::{AddSalesReceiptTransaction, ServiceError, Transaction},
            usecase::UsecaseError,
            usecase_impl::AddSalesReceiptImpl,
        };

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

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            payroll_domain::MemberId,
            service::{AddServiceChargeTransaction, ServiceError, Transaction},
            usecase::UsecaseError,
            usecase_impl::AddServiceChargeImpl,
        };

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

        use crate::{
            payroll_db::{PayrollDatabase, PayrollDbCtx},
            service::{PaydayTransaction, ServiceError, Transaction},
            usecase::UsecaseError,
            usecase_impl::PaydayImpl,
        };

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
}

mod payroll_util {
    use chrono::NaiveDate;

    pub fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }
}

use std::{cell::RefCell, rc::Rc};

use crate::payroll_db::PayrollDatabase;
use crate::payroll_util::date;
use crate::service::Transaction;
use crate::service_impl::*;

fn main() {
    env_logger::init();

    let db = Rc::new(RefCell::new(PayrollDatabase::new()));

    let tx: &mut dyn Transaction<T = _> =
        &mut AddSalariedEmployeeTx::new(1, "Bob", "Home", 1000.0, db.clone());
    println!("{:#?}", db);
    tx.execute().expect("register employee Bob");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> = &mut ChgEmployeeNameTx::new(1, "Alice", db.clone());
    tx.execute().expect("change employee name");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut ChgEmployeeAddressTx::new(1, "123 Main St.", db.clone());
    tx.execute().expect("change employee address");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> = &mut PaydayTx::new(date(2025, 1, 31), db.clone());
    tx.execute().expect("payday");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> = &mut ChgHourlyClassificationTx::new(1, 10.0, db.clone());
    tx.execute().expect("change employee to hourly");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut AddTimecardTx::new(1, date(2025, 1, 1), 8.0, db.clone());
    tx.execute().expect("add timecard");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> = &mut PaydayTx::new(date(2025, 1, 3), db.clone());
    tx.execute().expect("payday");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut ChgCommissionedClassificationTx::new(1, 510.0, 0.05, db.clone());
    tx.execute().expect("change employee to commissioned");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut AddSalesReceiptTx::new(1, date(2025, 1, 1), 35980.0, db.clone());
    tx.execute().expect("add sales receipt");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> = &mut PaydayTx::new(date(2025, 1, 10), db.clone());
    tx.execute().expect("payday");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut ChgSalariedClassificationTx::new(1, 1020.0, db.clone());
    tx.execute().expect("change employee to salaried");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut ChgDirectMethodTx::new(1, "mufg", "3-14159265", db.clone());
    tx.execute().expect("change employee to direct method");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut ChgMailMethodTx::new(1, "alice@gmail.com", db.clone());
    tx.execute().expect("change employee to mail method");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> = &mut ChgHoldMethodTx::new(1, db.clone());
    tx.execute().expect("change employee to hold method");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> = &mut AddUnionMemberTx::new(7463, 1, 100.0, db.clone());
    tx.execute().expect("add union member");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut AddServiceChargeTx::new(7463, date(2025, 1, 1), 300.5, db.clone());
    tx.execute().expect("add service charge");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> = &mut DelUnionMemberTx::new(1, db.clone());
    tx.execute().expect("delete union member");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> = &mut DelEmployeeTx::new(1, db.clone());
    tx.execute().expect("delete employee");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut AddHourlyEmployeeTx::new(2, "Charlie", "Home", 10.0, db.clone());
    tx.execute().expect("register employee Charlie");
    println!("{:#?}", db);

    let tx: &mut dyn Transaction<T = _> =
        &mut AddCommissionedEmployeeTx::new(3, "David", "Home", 500.0, 0.5, db.clone());
    tx.execute().expect("register employee David");
    println!("{:#?}", db);
}
