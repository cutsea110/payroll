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
        pub fn new(id: EmployeeId, name: &str, address: &str, hourly_rate: f32, dao: T) -> Self {
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
