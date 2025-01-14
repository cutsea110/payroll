use log::trace;

pub struct TxApp<S, F>
where
    S: TxSource,
    F: TxFactory,
{
    tx_source: S,
    tx_factory: F,
}
impl<S, F> TxApp<S, F>
where
    S: TxSource,
    F: TxFactory,
{
    pub fn new(tx_source: S, tx_factory: F) -> Self {
        Self {
            tx_source,
            tx_factory,
        }
    }
    pub fn run(&self) -> Result<(), anyhow::Error> {
        trace!("TxApp::run called");
        while let Some(tx_src) = self.tx_source.get_tx_source() {
            trace!("get tx_source={:?}", tx_src);
            let tx = self.tx_factory.convert(tx_src);
            tx.execute()?;
        }
        Ok(())
    }
}

mod tx {
    use anyhow;
    // なににも依存しない (domain は当然 ok)
    use payroll_domain::EmployeeId;

    // トランザクションのインターフェース
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Response {
        Void,
        EmployeeId(EmployeeId),
    }
    pub trait Transaction {
        fn execute(&self) -> Result<Response, anyhow::Error>;
    }
}
pub use tx::*;

mod tx_source {
    use chrono::NaiveDate;

    // なににも依存しない (domain は当然 ok)
    use payroll_domain::{EmployeeId, MemberId};

    #[derive(Debug, Clone, PartialEq)]
    pub enum Tx {
        AddSalariedEmployee(EmployeeId, String, String, f32),
        AddHourlyEmployee(EmployeeId, String, String, f32),
        AddCommissionedEmployee(EmployeeId, String, String, f32, f32),
        DeleteEmployee(EmployeeId),
        AddTimeCard(EmployeeId, NaiveDate, f32),
        AddSalesReceipt(EmployeeId, NaiveDate, f32),
        ChangeEmployeeName(EmployeeId, String),
        ChangeEmployeeAddress(EmployeeId, String),
        ChangeEmployeeSalaried(EmployeeId, f32),
        ChangeEmployeeHourly(EmployeeId, f32),
        ChangeEmployeeCommissioned(EmployeeId, f32, f32),
        ChangeMethodHold(EmployeeId),
        ChangeMethodDirect(EmployeeId, String, String),
        ChangeMethodMail(EmployeeId, String),
        AddUnionMember(EmployeeId, MemberId, f32),
        DeleteUnionMember(EmployeeId),
        AddServiceCharge(MemberId, NaiveDate, f32),
        Payday(NaiveDate),
    }
    pub trait TxSource {
        fn get_tx_source(&self) -> Option<Tx>;
    }
}
pub use tx_source::*;

mod tx_factory {
    use chrono::NaiveDate;
    use log::trace;

    // なににも依存しない (domain は当然 ok)
    use super::{Transaction, Tx};
    use payroll_domain::{EmployeeId, MemberId};

    pub trait TxFactory {
        fn convert(&self, src: Tx) -> Box<dyn Transaction> {
            trace!("TxFactory::convert called");
            match src {
                Tx::AddSalariedEmployee(id, name, address, salary) => {
                    trace!("convert Tx::AddSalariedEmployee by mk_add_salaried_employee_tx called");
                    self.mk_add_salaried_employee_tx(id, &name, &address, salary)
                }
                Tx::AddHourlyEmployee(id, name, address, hourly_rate) => {
                    trace!("convert Tx::AddHourlyEmployee by mk_add_hourly_employee_tx called");
                    self.mk_add_hourly_employee_tx(id, &name, &address, hourly_rate)
                }
                Tx::AddCommissionedEmployee(id, name, address, salary, commission_rate) => {
                    trace!(
                        "convert Tx::AddCommissionedEmployee by mk_add_commissioned_employee_tx called"
                    );
                    self.mk_add_commissioned_employee_tx(
                        id,
                        &name,
                        &address,
                        salary,
                        commission_rate,
                    )
                }
                Tx::DeleteEmployee(id) => {
                    trace!("convert Tx::DeleteEmployee by mk_delete_employee_tx called");
                    self.mk_delete_employee_tx(id)
                }
                Tx::ChangeEmployeeName(id, new_name) => {
                    trace!("convert Tx::ChangeEmployeeName by mk_change_employee_name_tx called");
                    self.mk_change_employee_name_tx(id, &new_name)
                }
                Tx::ChangeEmployeeAddress(id, new_address) => {
                    trace!(
                        "convert Tx::ChangeEmployeeAddress by mk_change_employee_address_tx called"
                    );
                    self.mk_change_employee_address_tx(id, &new_address)
                }
                Tx::AddTimeCard(id, date, hours) => {
                    trace!("convert Tx::AddTimeCard by mk_add_timecard_tx called");
                    self.mk_add_timecard_tx(id, date, hours)
                }
                Tx::AddSalesReceipt(id, date, amount) => {
                    trace!("convert Tx::AddSalesReceipt by mk_add_sales_receipt_tx called");
                    self.mk_add_sales_receipt_tx(id, date, amount)
                }
                Tx::ChangeEmployeeSalaried(id, salary) => {
                    trace!("convert Tx::ChangeEmployeeSalaried by mk_change_employee_salaried_tx called");
                    self.mk_change_employee_salaried_tx(id, salary)
                }
                Tx::ChangeEmployeeHourly(id, hourly_rate) => {
                    trace!(
                        "convert Tx::ChangeEmployeeHourly by mk_change_employee_hourly_tx called"
                    );
                    self.mk_change_employee_hourly_tx(id, hourly_rate)
                }
                Tx::ChangeEmployeeCommissioned(id, salary, commission_rate) => {
                    trace!(
                        "convert Tx::ChangeEmployeeCommissioned by mk_change_employee_commissioned_tx called"
                    );
                    self.mk_change_employee_commissioned_tx(id, salary, commission_rate)
                }
                Tx::ChangeMethodHold(id) => {
                    trace!("convert Tx::ChangeMethodHold by mk_change_method_hold_tx called");
                    self.mk_change_method_hold_tx(id)
                }
                Tx::ChangeMethodDirect(id, bank, account) => {
                    trace!("convert Tx::ChangeMethodDirect by mk_change_method_direct_tx called");
                    self.mk_change_method_direct_tx(id, &bank, &account)
                }
                Tx::ChangeMethodMail(id, address) => {
                    trace!("convert Tx::ChangeMethodMail by mk_change_method_mail_tx called");
                    self.mk_change_method_mail_tx(id, &address)
                }
                Tx::AddUnionMember(id, member_id, dues) => {
                    trace!("convert Tx::AddUnionMember by mk_add_union_member_tx called");
                    self.mk_add_union_member_tx(id, member_id, dues)
                }
                Tx::DeleteUnionMember(id) => {
                    trace!("convert Tx::DeleteUnionMember by mk_delete_union_member_tx called");
                    self.mk_delete_union_member_tx(id)
                }
                Tx::AddServiceCharge(member_id, date, amount) => {
                    trace!("convert Tx::AddServiceCharge by mk_add_service_charge_tx called");
                    self.mk_add_service_charge_tx(member_id, date, amount)
                }
                Tx::Payday(date) => {
                    trace!("convert Tx::Payday by mk_payday_tx called");
                    self.mk_payday_tx(date)
                }
            }
        }

        fn mk_add_salaried_employee_tx(
            &self,
            id: EmployeeId,
            name: &str,
            address: &str,
            salary: f32,
        ) -> Box<dyn Transaction>;
        fn mk_add_hourly_employee_tx(
            &self,
            id: EmployeeId,
            name: &str,
            address: &str,
            hourly_rate: f32,
        ) -> Box<dyn Transaction>;
        fn mk_add_commissioned_employee_tx(
            &self,
            id: EmployeeId,
            name: &str,
            address: &str,
            salary: f32,
            commission_rate: f32,
        ) -> Box<dyn Transaction>;
        fn mk_delete_employee_tx(&self, id: EmployeeId) -> Box<dyn Transaction>;
        fn mk_add_timecard_tx(
            &self,
            id: EmployeeId,
            date: NaiveDate,
            hours: f32,
        ) -> Box<dyn Transaction>;
        fn mk_add_sales_receipt_tx(
            &self,
            id: EmployeeId,
            date: NaiveDate,
            amount: f32,
        ) -> Box<dyn Transaction>;
        fn mk_change_employee_name_tx(
            &self,
            id: EmployeeId,
            new_name: &str,
        ) -> Box<dyn Transaction>;
        fn mk_change_employee_address_tx(
            &self,
            id: EmployeeId,
            new_address: &str,
        ) -> Box<dyn Transaction>;
        fn mk_change_employee_salaried_tx(
            &self,
            id: EmployeeId,
            salary: f32,
        ) -> Box<dyn Transaction>;
        fn mk_change_employee_hourly_tx(
            &self,
            id: EmployeeId,
            hourly_rate: f32,
        ) -> Box<dyn Transaction>;
        fn mk_change_employee_commissioned_tx(
            &self,
            id: EmployeeId,
            salary: f32,
            commission_rate: f32,
        ) -> Box<dyn Transaction>;
        fn mk_change_method_hold_tx(&self, id: EmployeeId) -> Box<dyn Transaction>;
        fn mk_change_method_direct_tx(
            &self,
            id: EmployeeId,
            bank: &str,
            account: &str,
        ) -> Box<dyn Transaction>;
        fn mk_change_method_mail_tx(&self, id: EmployeeId, address: &str) -> Box<dyn Transaction>;
        fn mk_add_union_member_tx(
            &self,
            id: EmployeeId,
            member_id: MemberId,
            dues: f32,
        ) -> Box<dyn Transaction>;
        fn mk_delete_union_member_tx(&self, id: EmployeeId) -> Box<dyn Transaction>;
        fn mk_add_service_charge_tx(
            &self,
            member_id: MemberId,
            date: NaiveDate,
            amount: f32,
        ) -> Box<dyn Transaction>;
        fn mk_payday_tx(&self, date: NaiveDate) -> Box<dyn Transaction>;
    }
}
pub use tx_factory::*;
