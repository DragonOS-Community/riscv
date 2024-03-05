/// Write the value into the tp register
#[inline(always)]
pub unsafe fn write(bits: usize) {
    core::arch::asm!("mv tp, {0}", in(reg) bits);
}

/// Read the value of the tp register
#[inline(always)]
pub unsafe fn read() -> usize {
    let r: usize;
    core::arch::asm!("mv {0}, tp", out(reg) r);
    r
}
