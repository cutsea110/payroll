use dao;
use payroll_db;
use payroll_domain;
use payroll_impl;
use service;
use usecase;

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
