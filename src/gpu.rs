use super::mmu::MMU;

pub struct GPU<'a> {
    mmu: &'a mut MMU
}

pub fn new<'a>(mmu: &'a mut MMU) -> GPU {
    GPU {
        mmu
    }
}

impl<'a> GPU<'a> {
    pub fn step(cycles: u64) {

    }
}
