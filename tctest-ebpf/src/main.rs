#![no_std]
#![no_main]

use aya_bpf::{
    macros::classifier,
    programs::SkBuffContext,
};
use aya_log_ebpf::info;

#[classifier(name="tctest1")]
pub fn tctest1(ctx: SkBuffContext) -> i32 {
    match unsafe { try_tctest1(ctx) } {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

unsafe fn try_tctest1(ctx: SkBuffContext) -> Result<i32, i32> {
    info!(&ctx, "tctest1 received a packet");
    Ok(0)
}

#[classifier(name="tctest2")]
pub fn tctest2(ctx: SkBuffContext) -> i32 {
    match unsafe { try_tctest2(ctx) } {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

unsafe fn try_tctest2(ctx: SkBuffContext) -> Result<i32, i32> {
    info!(&ctx, "tctest2 received a packet");
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
