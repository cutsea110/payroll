use payroll_domain::{Paycheck, PaymentMethod};

#[derive(Debug, Clone)]
pub struct HoldMethod;
impl PaymentMethod for HoldMethod {
    fn pay(&self, pc: &Paycheck) {
        println!("HoldMethod.pay: {:?}", pc);
    }
}
