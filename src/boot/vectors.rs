use crate::boot::entry::Reset;

union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

macro_rules! vector_handler {
    ($fp:ident) => { Vector { handler: $fp } };
    () => { Vector { reserved: 0 } };
}

#[link_section = ".vector_table.reset"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
static VECTOR_TABLE: [Vector; 81] = [
    vector_handler!(),
    vector_handler!(SFT_Int_Handler),
    vector_handler!(),
    vector_handler!(),
    vector_handler!(),
    vector_handler!(TMR_Int_Handler),
    vector_handler!(),
    vector_handler!(),
    vector_handler!(),
    vector_handler!(),
    vector_handler!(),
    vector_handler!(BWEI_Int_Handler),
    vector_handler!(PMOVI_Int_Handler),
    vector_handler!(WWDGT_Int_Handler),
    vector_handler!(LVD_Int_Handler),
    vector_handler!(Tamper_Int_Handler),
    vector_handler!(RTC_Global_Int_Handler),
    vector_handler!(FMC_Global_Int_Handler),
    vector_handler!(RCU_Global_Int_Handler),
    vector_handler!(EXTI_Line0_Int_Handler),
    vector_handler!(EXTI_Line1_Int_Handler),
    vector_handler!(EXTI_Line2_Int_Handler),
    vector_handler!(EXTI_Line3_Int_Handler),
    vector_handler!(EXTI_Line4_Int_Handler),
    vector_handler!(DMA0_Chan0_Int_Handler),
    vector_handler!(DMA0_Chan1_Int_Handler),
    vector_handler!(DMA0_Chan2_Int_Handler),
    vector_handler!(DMA0_Chan3_Int_Handler),
    vector_handler!(DMA0_Chan4_Int_Handler),
    vector_handler!(DMA0_Chan5_Int_Handler),
    vector_handler!(DMA0_Chan6_Int_Handler),
    vector_handler!(ADC0to1_Global_Int_Handler),
    vector_handler!(CAN0_Tx_Int_Handler),
    vector_handler!(CAN0_Rx0_Int_Handler),
    vector_handler!(CAN0_Rx1_Int_Handler),
    vector_handler!(CAN0_EWMC_Int_Handler),
    vector_handler!(EXTI_Line9to5_Int_Handler),
    vector_handler!(Timer0_Break_Int_Handler),
    vector_handler!(Timer0_Update_Int_Handler),
    vector_handler!(Timer0_Trig_Chan_Com_Int_Handler),
    vector_handler!(Timer0_Chan_Cap_Com_Int_Handler),
    vector_handler!(Timer1_Global_Int_Handler),
    vector_handler!(Timer2_Global_Int_Handler),
    vector_handler!(Timer3_Global_Int_Handler),
    vector_handler!(I2C0_Event_Int_Handler),
    vector_handler!(I2C0_Error_Int_Handler),
    vector_handler!(I2C1_Event_Int_Handler),
    vector_handler!(I2C1_Error_Int_Handler),
    vector_handler!(SPI0_Global_Int_Handler),
    vector_handler!(SPI1_Global_Int_Handler),
    vector_handler!(USART0_Global_Int_Handler),
    vector_handler!(USART1_Global_Int_Handler),
    vector_handler!(USART2_Global_Int_Handler),
    vector_handler!(EXTI_Line15to10_Int_Handler),
    vector_handler!(RTC_Alarm_Int_Handler),
    vector_handler!(USBFS_Wakeup_Int_Handler),
    vector_handler!(),
    vector_handler!(),
    vector_handler!(),
    vector_handler!(),
    vector_handler!(),
    vector_handler!(),
    vector_handler!(),
    vector_handler!(Timer4_Global_Int_Handler),
    vector_handler!(SPI2_Global_Int_Handler),
    vector_handler!(UART3_Global_Int_Handler),
    vector_handler!(UART4_Global_Int_Handler),
    vector_handler!(Timer5_Global_Int_Handler),
    vector_handler!(Timer6_Global_Int_Handler),
    vector_handler!(DMA1_Chan0_Int_Handler),
    vector_handler!(DMA1_Chan1_Int_Handler),
    vector_handler!(DMA1_Chan2_Int_Handler),
    vector_handler!(DMA1_Chan3_Int_Handler),
    vector_handler!(DMA1_Chan4_Int_Handler),
    vector_handler!(),
    vector_handler!(),
    vector_handler!(Can1_Tx_Int_Handler),
    vector_handler!(Can1_Rx0_Int_Handler),
    vector_handler!(Can1_Rx1_Int_Handler),
    vector_handler!(Can1_EWMC_Int_Handler),
    vector_handler!(USBFS_Global_Int_Handler),
];

#[no_mangle]
pub unsafe extern "C" fn Default_Int_Handler() {}

extern "C" {
    pub fn SFT_Int_Handler();
    pub fn TMR_Int_Handler();
    pub fn BWEI_Int_Handler();
    pub fn PMOVI_Int_Handler();
    pub fn WWDGT_Int_Handler();
    pub fn LVD_Int_Handler();
    pub fn Tamper_Int_Handler();
    pub fn RTC_Global_Int_Handler();
    pub fn FMC_Global_Int_Handler();
    pub fn RCU_Global_Int_Handler();
    pub fn EXTI_Line0_Int_Handler();
    pub fn EXTI_Line1_Int_Handler();
    pub fn EXTI_Line2_Int_Handler();
    pub fn EXTI_Line3_Int_Handler();
    pub fn EXTI_Line4_Int_Handler();
    pub fn DMA0_Chan0_Int_Handler();
    pub fn DMA0_Chan1_Int_Handler();
    pub fn DMA0_Chan2_Int_Handler();
    pub fn DMA0_Chan3_Int_Handler();
    pub fn DMA0_Chan4_Int_Handler();
    pub fn DMA0_Chan5_Int_Handler();
    pub fn DMA0_Chan6_Int_Handler();
    pub fn ADC0to1_Global_Int_Handler();
    pub fn CAN0_Tx_Int_Handler();
    pub fn CAN0_Rx0_Int_Handler();
    pub fn CAN0_Rx1_Int_Handler();
    pub fn CAN0_EWMC_Int_Handler();
    pub fn EXTI_Line9to5_Int_Handler();
    pub fn Timer0_Break_Int_Handler();
    pub fn Timer0_Update_Int_Handler();
    pub fn Timer0_Trig_Chan_Com_Int_Handler();
    pub fn Timer0_Chan_Cap_Com_Int_Handler();
    pub fn Timer1_Global_Int_Handler();
    pub fn Timer2_Global_Int_Handler();
    pub fn Timer3_Global_Int_Handler();
    pub fn I2C0_Event_Int_Handler();
    pub fn I2C0_Error_Int_Handler();
    pub fn I2C1_Event_Int_Handler();
    pub fn I2C1_Error_Int_Handler();
    pub fn SPI0_Global_Int_Handler();
    pub fn SPI1_Global_Int_Handler();
    pub fn USART0_Global_Int_Handler();
    pub fn USART1_Global_Int_Handler();
    pub fn USART2_Global_Int_Handler();
    pub fn EXTI_Line15to10_Int_Handler();
    pub fn RTC_Alarm_Int_Handler();
    pub fn USBFS_Wakeup_Int_Handler();
    pub fn Timer4_Global_Int_Handler();
    pub fn SPI2_Global_Int_Handler();
    pub fn UART3_Global_Int_Handler();
    pub fn UART4_Global_Int_Handler();
    pub fn Timer5_Global_Int_Handler();
    pub fn Timer6_Global_Int_Handler();
    pub fn DMA1_Chan0_Int_Handler();
    pub fn DMA1_Chan1_Int_Handler();
    pub fn DMA1_Chan2_Int_Handler();
    pub fn DMA1_Chan3_Int_Handler();
    pub fn DMA1_Chan4_Int_Handler();
    pub fn Can1_Tx_Int_Handler();
    pub fn Can1_Rx0_Int_Handler();
    pub fn Can1_Rx1_Int_Handler();
    pub fn Can1_EWMC_Int_Handler();
    pub fn USBFS_Global_Int_Handler();
}