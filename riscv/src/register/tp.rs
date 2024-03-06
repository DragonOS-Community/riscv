/// Write the value into the tp register
#[inline(always)]
pub fn write(bits: usize) {
    unsafe {
        core::arch::asm!("mv tp, {0}", in(reg) bits);
    }
}

/// Read the value of the tp register
#[inline(always)]
pub fn read() -> usize {
    let r: usize;
    unsafe {
        core::arch::asm!("mv {0}, tp", out(reg) r);
    }
    r
}
