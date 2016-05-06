#![crate_name = "sam4l"]
#![crate_type = "rlib"]
#![feature(asm,core_intrinsics,concat_idents,const_fn)]
#![no_std]

#[macro_use]
extern crate common;
extern crate hil;
extern crate process;

#[macro_use]
mod helpers;

pub mod chip;
pub mod ast;
pub mod bpm;
pub mod dma;
pub mod i2c;
pub mod spi;
pub mod nvic;
pub mod pm;
pub mod gpio;
pub mod usart;
pub mod scif;
pub mod adc;

unsafe extern "C" fn unhandled_interrupt() {
    let mut interrupt_number: u32;
    panic!("NO INTERRUPT");

    // IPSR[8:0] holds the currently active interrupt
    asm!(
        "mrs    r0, ipsr                    "
        : "={r0}"(interrupt_number)
        :
        : "r0"
        :
        );

    interrupt_number = interrupt_number & 0x1ff;

    panic!("Unhandled Interrupt. ISR {} is active.", interrupt_number);
}

extern {
    // _estack is not really a function, but it makes the types work
    // You should never actually invoke it!!
    fn _estack();

    // Defined in src/main/main.rs
    fn main();

    // Defined in src/arch/cortex-m4/ctx_switch.S
    fn SVC_Handler();
    fn systick_handler();

    fn hard_fault_handler();
    fn generic_isr();

    static mut _szero : u32;
    static mut _ezero : u32;
    static mut _etext : u32;
    static mut _srelocate : u32;
    static mut _erelocate : u32;
}

#[link_section=".vectors"]
pub static BASE_VECTORS: [unsafe extern fn(); 16] = [
    _estack, reset_handler,
    /* NMI */           unhandled_interrupt,
    /* Hard Fault */    hard_fault_handler,
    /* MemManage */     unhandled_interrupt,
    /* BusFault */      unhandled_interrupt,
    /* UsageFault*/     unhandled_interrupt,
    unhandled_interrupt, unhandled_interrupt, unhandled_interrupt,
    unhandled_interrupt,
    /* SVC */           SVC_Handler,
    /* DebugMon */      unhandled_interrupt,
    unhandled_interrupt,
    /* PendSV */        unhandled_interrupt,
    /* SysTick */       systick_handler
];

#[link_section=".vectors"]
pub static IRQS: [unsafe extern fn(); 80] = [generic_isr; 80];

#[no_mangle]
pub static INTERRUPT_TABLE: [Option<unsafe extern fn()>; 80] = [
    // Perhipheral vectors are defined by Atmel in the SAM4L datasheet section
    // 4.7.
    /* HFLASHC */       Option::Some(unhandled_interrupt),
    /* PDCA0 */         Option::Some(dma::pdca0_handler),
    /* PDCA1 */         Option::Some(dma::pdca1_handler),
    /* PDCA2 */         Option::Some(dma::pdca2_handler),
    /* PDCA3 */         Option::Some(dma::pdca3_handler),
    /* PDCA4 */         Option::Some(dma::pdca4_handler),
    /* PDCA5 */         Option::Some(dma::pdca5_handler),
    /* PDCA6 */         Option::Some(dma::pdca6_handler),
    /* PDCA7 */         Option::Some(dma::pdca7_handler),
    /* PDCA8 */         Option::Some(dma::pdca8_handler),
    /* PDCA9 */         Option::Some(dma::pdca9_handler),
    /* PDCA10 */        Option::Some(dma::pdca10_handler),
    /* PDCA11 */        Option::Some(dma::pdca11_handler),
    /* PDCA12 */        Option::Some(dma::pdca12_handler),
    /* PDCA13 */        Option::Some(dma::pdca13_handler),
    /* PDCA14 */        Option::Some(dma::pdca14_handler),
    /* PDCA15 */        Option::Some(dma::pdca15_handler),
    /* CRCCU */         Option::Some(unhandled_interrupt),
    /* USBC */          Option::Some(unhandled_interrupt),
    /* PEVC_TR */       Option::Some(unhandled_interrupt),
    /* PEVC_OV */       Option::Some(unhandled_interrupt),
    /* AESA */          Option::Some(unhandled_interrupt),
    /* PM */            Option::Some(unhandled_interrupt),
    /* SCIF */          Option::Some(unhandled_interrupt),
    /* FREQM */         Option::Some(unhandled_interrupt),
    /* GPIO0 */         Option::Some(gpio::gpio0_handler),
    /* GPIO1 */         Option::Some(gpio::gpio1_handler),
    /* GPIO2 */         Option::Some(gpio::gpio2_handler),
    /* GPIO3 */         Option::Some(gpio::gpio3_handler),
    /* GPIO4 */         Option::Some(gpio::gpio4_handler),
    /* GPIO5 */         Option::Some(gpio::gpio5_handler),
    /* GPIO6 */         Option::Some(gpio::gpio6_handler),
    /* GPIO7 */         Option::Some(gpio::gpio7_handler),
    /* GPIO8 */         Option::Some(gpio::gpio8_handler),
    /* GPIO9 */         Option::Some(gpio::gpio9_handler),
    /* GPIO10 */        Option::Some(gpio::gpio10_handler),
    /* GPIO11 */        Option::Some(gpio::gpio11_handler),
    /* BPM */           Option::Some(unhandled_interrupt),
    /* BSCIF */         Option::Some(unhandled_interrupt),
    /* AST_ALARM */     Option::Some(ast::ast_alarm_handler),
    /* AST_PER */       Option::Some(unhandled_interrupt),
    /* AST_OVF */       Option::Some(unhandled_interrupt),
    /* AST_READY */     Option::Some(unhandled_interrupt),
    /* AST_CLKREADY */  Option::Some(unhandled_interrupt),
    /* WDT */           Option::Some(unhandled_interrupt),
    /* EIC1 */          Option::Some(unhandled_interrupt),
    /* EIC2 */          Option::Some(unhandled_interrupt),
    /* EIC3 */          Option::Some(unhandled_interrupt),
    /* EIC4 */          Option::Some(unhandled_interrupt),
    /* EIC5 */          Option::Some(unhandled_interrupt),
    /* EIC6 */          Option::Some(unhandled_interrupt),
    /* EIC7 */          Option::Some(unhandled_interrupt),
    /* EIC8 */          Option::Some(unhandled_interrupt),
    /* IISC */          Option::Some(unhandled_interrupt),
    /* SPI */           Option::Some(unhandled_interrupt),
    /* TC00 */          Option::Some(unhandled_interrupt),
    /* TC01 */          Option::Some(unhandled_interrupt),
    /* TC02 */          Option::Some(unhandled_interrupt),
    /* TC10 */          Option::Some(unhandled_interrupt),
    /* TC11 */          Option::Some(unhandled_interrupt),
    /* TC12 */          Option::Some(unhandled_interrupt),
    /* TWIM0 */         Option::Some(i2c::twim0_handler),
    /* TWIS0 */         Option::Some(unhandled_interrupt),
    /* TWIM1 */         Option::Some(i2c::twim1_handler),
    /* TWIS1 */         Option::Some(unhandled_interrupt),
    /* USART0 */        Option::Some(usart::usart0_handler),
    /* USART1 */        Option::Some(usart::usart1_handler),
    /* USART2 */        Option::Some(usart::usart2_handler),
    /* USART3 */        Option::Some(usart::usart3_handler),
    /* ADCIFE */        Option::Some(adc::adcife_handler),
    /* DACC */          Option::Some(unhandled_interrupt),
    /* ACIFC */         Option::Some(unhandled_interrupt),
    /* ABDACB */        Option::Some(unhandled_interrupt),
    /* TRNG */          Option::Some(unhandled_interrupt),
    /* PARC */          Option::Some(unhandled_interrupt),
    /* CATB */          Option::Some(unhandled_interrupt),
    None,
    /* TWIM2 */         Option::Some(i2c::twim2_handler),
    /* TWIM3 */         Option::Some(i2c::twim3_handler),
    /* LCDCA */         Option::Some(unhandled_interrupt),
];

unsafe extern "C" fn reset_handler() {

    // Relocate data segment.
    // Assumes data starts right after text segment as specified by the linker
    // file.
    let mut pdest  = &mut _srelocate as *mut u32;
    let pend  = &mut _erelocate as *mut u32;
    let mut psrc = &_etext as *const u32;

    if psrc != pdest {
        while (pdest as *const u32) < pend {
            *pdest = *psrc;
            pdest = pdest.offset(1);
            psrc = psrc.offset(1);
        }
    }

    // Clear the zero segment (BSS)
    let pzero = &_ezero as *const u32;
    pdest = &mut _szero as *mut u32;

    while (pdest as *const u32) < pzero {
        *pdest = 0;
        pdest = pdest.offset(1);
    }

    main();
}


