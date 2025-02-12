use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmployeeId(u32);
impl fmt::Display for EmployeeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EmployeeId({})", self.0)
    }
}
impl From<u32> for EmployeeId {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemberId(u32);
impl fmt::Display for MemberId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MemberId({})", self.0)
    }
}
impl From<u32> for MemberId {
    fn from(id: u32) -> Self {
        Self(id)
    }
}
