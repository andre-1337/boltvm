/// The registers of the virtual machine. There are 65535 registers in total.
#[derive(Debug, Copy, Clone)]
pub struct Register(pub u16);

impl Register {
    pub fn as_index(&self) -> usize {
        self.0 as usize
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       write!(f, "r{}", self.0)
    }
}
