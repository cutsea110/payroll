use chrono::NaiveDate;
use log::trace;

use crate::{
    AddCommissionedEmployeeTx, AddHourlyEmployeeTx, AddSalariedEmployeeTx, AddSalesReceiptTx,
    AddServiceChargeTx, AddTimeCardTx, ChangeCommissionedTx, ChangeDirectTx,
    ChangeEmployeeAddressTx, ChangeEmployeeNameTx, ChangeHoldTx, ChangeHourlyTx, ChangeMailTx,
    ChangeMemberTx, ChangeNoMemberTx, ChangeSalariedTx, DeleteEmployeeTx, PaydayTx,
};
use dao::EmployeeDao;
use payroll_domain::{EmployeeId, MemberId};
use payroll_factory::{
    BiweeklyScheduleFactory, CommissionedClassificationFactory, DirectMethodFactory,
    HoldMethodFactory, HourlyClassificationFactory, MailMethodFactory, MonthlyScheduleFactory,
    NoAffiliationFactory, SalariedClassificationFactory, UnionAffiliationFactory,
    WeeklyScheduleFactory,
};
use tx_app::Transaction;
use tx_factory::{
    AddCommissionedEmployeeTxFactory, AddHourlyEmployeeTxFactory, AddSalariedEmployeeTxFactory,
    AddSalesReceiptTxFactory, AddServiceChargeTxFactory, AddTimecardTxFactory,
    ChangeEmployeeAddressTxFactory, ChangeEmployeeCommissionedTxFactory,
    ChangeEmployeeDirectTxFactory, ChangeEmployeeHoldTxFactory, ChangeEmployeeHourlyTxFactory,
    ChangeEmployeeMailTxFactory, ChangeEmployeeMemberTxFactory, ChangeEmployeeNameTxFactory,
    ChangeEmployeeNoMemberTxFactory, ChangeEmployeeSalariedTxFactory, DeleteEmployeeTxFactory,
    PaydayTxFactory,
};

pub struct TxFactoryImpl<T, F>
where
    T: EmployeeDao,
{
    dao: T,
    payroll_factory: F,
}
impl<T, F> TxFactoryImpl<T, F>
where
    T: EmployeeDao,
{
    pub fn new(dao: T, payroll_factory: F) -> Self {
        Self {
            dao,
            payroll_factory,
        }
    }
}
impl<T, F> AddSalariedEmployeeTxFactory for TxFactoryImpl<T, F>
where
    T: EmployeeDao + Clone + 'static,
    F: SalariedClassificationFactory
        + MonthlyScheduleFactory
        + HoldMethodFactory
        + NoAffiliationFactory
        + Clone
        + 'static,
{
    fn mk_tx(
        &self,
        id: EmployeeId,
        name: &str,
        address: &str,
        salary: f32,
    ) -> Box<dyn Transaction> {
        trace!("mk_tx called for AddSalariedEmployeeTx");
        Box::new(AddSalariedEmployeeTx::new(
            id,
            name,
            address,
            salary,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
}
impl<T, F> AddHourlyEmployeeTxFactory for TxFactoryImpl<T, F>
where
    T: EmployeeDao + Clone + 'static,
    F: HourlyClassificationFactory
        + WeeklyScheduleFactory
        + HoldMethodFactory
        + NoAffiliationFactory
        + Clone
        + 'static,
{
    fn mk_tx(
        &self,
        id: EmployeeId,
        name: &str,
        address: &str,
        hourly_rate: f32,
    ) -> Box<dyn Transaction> {
        trace!("mk_tx called for AddHourlyEmployeeTx");
        Box::new(AddHourlyEmployeeTx::new(
            id,
            name,
            address,
            hourly_rate,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
}
impl<T, F> AddCommissionedEmployeeTxFactory for TxFactoryImpl<T, F>
where
    T: EmployeeDao + Clone + 'static,
    F: CommissionedClassificationFactory
        + BiweeklyScheduleFactory
        + HoldMethodFactory
        + NoAffiliationFactory
        + Clone
        + 'static,
{
    fn mk_tx(
        &self,
        id: EmployeeId,
        name: &str,
        address: &str,
        salary: f32,
        commission_rate: f32,
    ) -> Box<dyn Transaction> {
        trace!("mk_tx called for AddCommissionedEmployeeTx");
        Box::new(AddCommissionedEmployeeTx::new(
            id,
            name,
            address,
            salary,
            commission_rate,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
}
impl<T, F> DeleteEmployeeTxFactory for TxFactoryImpl<T, F>
where
    T: EmployeeDao + Clone + 'static,
    F: Clone,
{
    fn mk_tx(&self, id: EmployeeId) -> Box<dyn Transaction> {
        trace!("mk_tx called for DeleteEmployeeTx");
        Box::new(DeleteEmployeeTx::new(id, self.dao.clone()))
    }
}
impl<T, F> AddTimecardTxFactory for TxFactoryImpl<T, F>
where
    T: EmployeeDao + Clone + 'static,
    F: Clone,
{
    fn mk_tx(&self, id: EmployeeId, date: NaiveDate, hours: f32) -> Box<dyn Transaction> {
        trace!("mk_tx called for AddTimeCardTx");
        Box::new(AddTimeCardTx::new(id, date, hours, self.dao.clone()))
    }
}
impl<T, F> AddSalesReceiptTxFactory for TxFactoryImpl<T, F>
where
    T: EmployeeDao + Clone + 'static,
    F: Clone,
{
    fn mk_tx(&self, id: EmployeeId, date: NaiveDate, amount: f32) -> Box<dyn Transaction> {
        trace!("mk_tx called for AddSalesReceiptTx");
        Box::new(AddSalesReceiptTx::new(id, date, amount, self.dao.clone()))
    }
}
impl<T, F> AddServiceChargeTxFactory for TxFactoryImpl<T, F>
where
    T: EmployeeDao + Clone + 'static,
    F: Clone,
{
    fn mk_tx(&self, member_id: MemberId, date: NaiveDate, amount: f32) -> Box<dyn Transaction> {
        trace!("mk_tx called for AddServiceChargeTx");
        Box::new(AddServiceChargeTx::new(
            member_id,
            date,
            amount,
            self.dao.clone(),
        ))
    }
}
impl<T, F> ChangeEmployeeNameTxFactory for TxFactoryImpl<T, F>
where
    T: EmployeeDao + Clone + 'static,
    F: Clone,
{
    fn mk_tx(&self, id: EmployeeId, new_name: &str) -> Box<dyn Transaction> {
        trace!("mk_tx called for ChangeEmployeeNameTx");
        Box::new(ChangeEmployeeNameTx::new(id, new_name, self.dao.clone()))
    }
}
impl<T, F> ChangeEmployeeAddressTxFactory for TxFactoryImpl<T, F>
where
    T: EmployeeDao + Clone + 'static,
    F: Clone,
{
    fn mk_tx(&self, id: EmployeeId, new_address: &str) -> Box<dyn Transaction> {
        trace!("mk_tx called for ChangeEmployeeAddressTx");
        Box::new(ChangeEmployeeAddressTx::new(
            id,
            new_address,
            self.dao.clone(),
        ))
    }
}
impl<T, F> ChangeEmployeeSalariedTxFactory for TxFactoryImpl<T, F>
where
    T: EmployeeDao + Clone + 'static,
    F: SalariedClassificationFactory + MonthlyScheduleFactory + Clone + 'static,
{
    fn mk_tx(&self, id: EmployeeId, salary: f32) -> Box<dyn Transaction> {
        trace!("mk_tx called for ChangeEmployeeSalariedTx");
        Box::new(ChangeSalariedTx::new(
            id,
            salary,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
}
impl<T, F> ChangeEmployeeHourlyTxFactory for TxFactoryImpl<T, F>
where
    T: EmployeeDao + Clone + 'static,
    F: HourlyClassificationFactory + WeeklyScheduleFactory + Clone + 'static,
{
    fn mk_tx(&self, id: EmployeeId, hourly_rate: f32) -> Box<dyn Transaction> {
        trace!("mk_tx called for ChangeEmployeeHourlyTx");
        Box::new(ChangeHourlyTx::new(
            id,
            hourly_rate,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
}
impl<T, F> ChangeEmployeeCommissionedTxFactory for TxFactoryImpl<T, F>
where
    T: EmployeeDao + Clone + 'static,
    F: CommissionedClassificationFactory + BiweeklyScheduleFactory + Clone + 'static,
{
    fn mk_tx(&self, id: EmployeeId, salary: f32, commission_rate: f32) -> Box<dyn Transaction> {
        trace!("mk_tx called for ChangeEmployeeCommissionedTx");
        Box::new(ChangeCommissionedTx::new(
            id,
            salary,
            commission_rate,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
}
impl<T, F> ChangeEmployeeHoldTxFactory for TxFactoryImpl<T, F>
where
    T: EmployeeDao + Clone + 'static,
    F: HoldMethodFactory + Clone + 'static,
{
    fn mk_tx(&self, id: EmployeeId) -> Box<dyn Transaction> {
        trace!("mk_tx called for ChangeEmployeeHoldTx");
        Box::new(ChangeHoldTx::new(
            id,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
}
impl<T, F> ChangeEmployeeDirectTxFactory for TxFactoryImpl<T, F>
where
    T: EmployeeDao + Clone + 'static,
    F: DirectMethodFactory + Clone + 'static,
{
    fn mk_tx(&self, id: EmployeeId, bank: &str, account: &str) -> Box<dyn Transaction> {
        trace!("mk_tx called for ChangeEmployeeDirectTx");
        Box::new(ChangeDirectTx::new(
            id,
            bank,
            account,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
}
impl<T, F> ChangeEmployeeMailTxFactory for TxFactoryImpl<T, F>
where
    T: EmployeeDao + Clone + 'static,
    F: MailMethodFactory + Clone + 'static,
{
    fn mk_tx(&self, id: EmployeeId, address: &str) -> Box<dyn Transaction> {
        trace!("mk_tx called for ChangeEmployeeMailTx");
        Box::new(ChangeMailTx::new(
            id,
            address,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
}
impl<T, F> ChangeEmployeeMemberTxFactory for TxFactoryImpl<T, F>
where
    T: EmployeeDao + Clone + 'static,
    F: UnionAffiliationFactory + Clone + 'static,
{
    fn mk_tx(&self, emp_id: EmployeeId, member_id: MemberId, dues: f32) -> Box<dyn Transaction> {
        trace!("mk_change_employee_member_tx called");
        Box::new(ChangeMemberTx::new(
            member_id,
            emp_id,
            dues,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
}
impl<T, F> ChangeEmployeeNoMemberTxFactory for TxFactoryImpl<T, F>
where
    T: EmployeeDao + Clone + 'static,
    F: NoAffiliationFactory + Clone + 'static,
{
    fn mk_tx(&self, id: EmployeeId) -> Box<dyn Transaction> {
        trace!("mk_change_employee_no_member_tx called");
        Box::new(ChangeNoMemberTx::new(
            id,
            self.dao.clone(),
            self.payroll_factory.clone(),
        ))
    }
}
impl<T, F> PaydayTxFactory for TxFactoryImpl<T, F>
where
    T: EmployeeDao + Clone + 'static,
{
    fn mk_tx(&self, date: NaiveDate) -> Box<dyn Transaction> {
        trace!("mk_payday_tx called");
        Box::new(PaydayTx::new(date, self.dao.clone()))
    }
}
