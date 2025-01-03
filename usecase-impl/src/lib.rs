mod add_salaried_emp {
    use std::{cell::RefCell, fmt::Debug, rc::Rc};

    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_db::{PayrollDbCtx, PayrollDbDao};
    use payroll_domain::{EmployeeId, PaymentClassification, PaymentSchedule};
    use payroll_impl::{MonthlySchedule, SalariedClassification};
    use usecase::AddEmployee;

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

    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_db::{PayrollDbCtx, PayrollDbDao};
    use payroll_domain::{EmployeeId, PaymentClassification, PaymentSchedule};
    use payroll_impl::{HourlyClassification, WeeklySchedule};
    use usecase::AddEmployee;

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

    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_db::{PayrollDbCtx, PayrollDbDao};
    use payroll_domain::{EmployeeId, PaymentClassification, PaymentSchedule};
    use payroll_impl::{BiweeklySchedule, CommissionedClassification};
    use usecase::AddEmployee;

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

    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_db::{PayrollDbCtx, PayrollDbDao};
    use payroll_domain::EmployeeId;
    use usecase::ChgEmployeeName;

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

    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_db::{PayrollDbCtx, PayrollDbDao};
    use payroll_domain::EmployeeId;
    use usecase::ChgEmployeeAddress;

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

    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_db::{PayrollDbCtx, PayrollDbDao};
    use payroll_domain::EmployeeId;
    use usecase::DelEmployee;

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

    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_db::{PayrollDbCtx, PayrollDbDao};
    use payroll_domain::{EmployeeId, PaymentClassification, PaymentSchedule};
    use payroll_impl::{MonthlySchedule, SalariedClassification};
    use usecase::ChgClassification;

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

    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_db::{PayrollDbCtx, PayrollDbDao};
    use payroll_domain::{EmployeeId, PaymentClassification, PaymentSchedule};
    use payroll_impl::{HourlyClassification, WeeklySchedule};
    use usecase::ChgClassification;

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

    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_db::{PayrollDbCtx, PayrollDbDao};
    use payroll_domain::{EmployeeId, PaymentClassification, PaymentSchedule};
    use payroll_impl::{BiweeklySchedule, CommissionedClassification};
    use usecase::ChgClassification;

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

    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_db::{PayrollDbCtx, PayrollDbDao};
    use payroll_domain::{EmployeeId, PaymentMethod};
    use payroll_impl::HoldMethod;
    use usecase::ChgMethod;

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

    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_db::{PayrollDbCtx, PayrollDbDao};
    use payroll_domain::{EmployeeId, PaymentMethod};
    use payroll_impl::DirectMethod;
    use usecase::ChgMethod;

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

    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_db::{PayrollDbCtx, PayrollDbDao};
    use payroll_domain::{EmployeeId, PaymentMethod};
    use payroll_impl::MailMethod;
    use usecase::ChgMethod;

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

    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_db::{PayrollDbCtx, PayrollDbDao};
    use payroll_domain::{Affiliation, EmployeeId, MemberId};
    use payroll_impl::UnionAffiliation;
    use usecase::AddUnionAffiliation;

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

    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_db::{PayrollDbCtx, PayrollDbDao};
    use payroll_domain::{Affiliation, EmployeeId, NoAffiliation};
    use usecase::DelUnionAffiliation;

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

    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_db::{PayrollDbCtx, PayrollDbDao};
    use payroll_domain::EmployeeId;
    use usecase::AddTimeCard;

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

    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_db::{PayrollDbCtx, PayrollDbDao};
    use payroll_domain::EmployeeId;
    use usecase::AddSalesReceipt;

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

    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_db::{PayrollDbCtx, PayrollDbDao};
    use payroll_domain::{EmployeeId, MemberId};
    use usecase::AddServiceCharge;

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

    use dao::{EmployeeDao, HaveEmployeeDao};
    use payroll_db::{PayrollDbCtx, PayrollDbDao};
    use usecase::Payday;

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
