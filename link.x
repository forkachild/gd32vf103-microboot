INCLUDE memory-cb.x

/* The entry point is the reset handler */
ENTRY(Reset);
EXTERN(Default_Int_Handler);

PROVIDE(SFT_Int_Handler = Default_Int_Handler);
PROVIDE(TMR_Int_Handler = Default_Int_Handler);
PROVIDE(BWEI_Int_Handler = Default_Int_Handler);
PROVIDE(PMOVI_Int_Handler = Default_Int_Handler);
PROVIDE(WWDGT_Int_Handler = Default_Int_Handler);
PROVIDE(LVD_Int_Handler = Default_Int_Handler);
PROVIDE(Tamper_Int_Handler = Default_Int_Handler);
PROVIDE(RTC_Global_Int_Handler = Default_Int_Handler);
PROVIDE(FMC_Global_Int_Handler = Default_Int_Handler);
PROVIDE(RCU_Global_Int_Handler = Default_Int_Handler);
PROVIDE(EXTI_Line0_Int_Handler = Default_Int_Handler);
PROVIDE(EXTI_Line1_Int_Handler = Default_Int_Handler);
PROVIDE(EXTI_Line2_Int_Handler = Default_Int_Handler);
PROVIDE(EXTI_Line3_Int_Handler = Default_Int_Handler);
PROVIDE(EXTI_Line4_Int_Handler = Default_Int_Handler);
PROVIDE(DMA0_Chan0_Int_Handler = Default_Int_Handler);
PROVIDE(DMA0_Chan1_Int_Handler = Default_Int_Handler);
PROVIDE(DMA0_Chan2_Int_Handler = Default_Int_Handler);
PROVIDE(DMA0_Chan3_Int_Handler = Default_Int_Handler);
PROVIDE(DMA0_Chan4_Int_Handler = Default_Int_Handler);
PROVIDE(DMA0_Chan5_Int_Handler = Default_Int_Handler);
PROVIDE(DMA0_Chan6_Int_Handler = Default_Int_Handler);
PROVIDE(ADC0to1_Global_Int_Handler = Default_Int_Handler);
PROVIDE(CAN0_Tx_Int_Handler = Default_Int_Handler);
PROVIDE(CAN0_Rx0_Int_Handler = Default_Int_Handler);
PROVIDE(CAN0_Rx1_Int_Handler = Default_Int_Handler);
PROVIDE(CAN0_EWMC_Int_Handler = Default_Int_Handler);
PROVIDE(EXTI_Line9to5_Int_Handler = Default_Int_Handler);
PROVIDE(Timer0_Break_Int_Handler = Default_Int_Handler);
PROVIDE(Timer0_Update_Int_Handler = Default_Int_Handler);
PROVIDE(Timer0_Trig_Chan_Com_Int_Handler = Default_Int_Handler);
PROVIDE(Timer0_Chan_Cap_Com_Int_Handler = Default_Int_Handler);
PROVIDE(Timer1_Global_Int_Handler = Default_Int_Handler);
PROVIDE(Timer2_Global_Int_Handler = Default_Int_Handler);
PROVIDE(Timer3_Global_Int_Handler = Default_Int_Handler);
PROVIDE(I2C0_Event_Int_Handler = Default_Int_Handler);
PROVIDE(I2C0_Error_Int_Handler = Default_Int_Handler);
PROVIDE(I2C1_Event_Int_Handler = Default_Int_Handler);
PROVIDE(I2C1_Error_Int_Handler = Default_Int_Handler);
PROVIDE(SPI0_Global_Int_Handler = Default_Int_Handler);
PROVIDE(SPI1_Global_Int_Handler = Default_Int_Handler);
PROVIDE(USART0_Global_Int_Handler = Default_Int_Handler);
PROVIDE(USART1_Global_Int_Handler = Default_Int_Handler);
PROVIDE(USART2_Global_Int_Handler = Default_Int_Handler);
PROVIDE(EXTI_Line15to10_Int_Handler = Default_Int_Handler);
PROVIDE(RTC_Alarm_Int_Handler = Default_Int_Handler);
PROVIDE(USBFS_Wakeup_Int_Handler = Default_Int_Handler);
PROVIDE(Timer4_Global_Int_Handler = Default_Int_Handler);
PROVIDE(SPI2_Global_Int_Handler = Default_Int_Handler);
PROVIDE(UART3_Global_Int_Handler = Default_Int_Handler);
PROVIDE(UART4_Global_Int_Handler = Default_Int_Handler);
PROVIDE(Timer5_Global_Int_Handler = Default_Int_Handler);
PROVIDE(Timer6_Global_Int_Handler = Default_Int_Handler);
PROVIDE(DMA1_Chan0_Int_Handler = Default_Int_Handler);
PROVIDE(DMA1_Chan1_Int_Handler = Default_Int_Handler);
PROVIDE(DMA1_Chan2_Int_Handler = Default_Int_Handler);
PROVIDE(DMA1_Chan3_Int_Handler = Default_Int_Handler);
PROVIDE(DMA1_Chan4_Int_Handler = Default_Int_Handler);
PROVIDE(Can1_Tx_Int_Handler = Default_Int_Handler);
PROVIDE(Can1_Rx0_Int_Handler = Default_Int_Handler);
PROVIDE(Can1_Rx1_Int_Handler = Default_Int_Handler);
PROVIDE(Can1_EWMC_Int_Handler = Default_Int_Handler);
PROVIDE(USBFS_Global_Int_Handler = Default_Int_Handler);

SECTIONS
{
  PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));

  .vector_table ORIGIN(FLASH) :
  {
    LONG(_stack_start);
    KEEP(*(.vector_table.reset));
    KEEP(*(.vector_table.interrupts));
  } >FLASH

  PROVIDE(_stext = ADDR(.vector_table) + SIZEOF(.vector_table));

  .text _stext :
  {
    PROVIDE(__stext = .);
    *(.text .text.*);
    . = ALIGN(4);
    PROVIDE(__etext = .);
  } >FLASH

  .rodata : ALIGN(4)
  {
    . = ALIGN(4);
    PROVIDE(__srodata = .);
    *(.rodata .rodata.*);
    . = ALIGN(4);
    $(COMMON);
    . = ALIGN(4);
    PROVIDE(__erodata = .);
  } > FLASH

  .data : ALIGN(4)
  {
    . = ALIGN(4);
    PROVIDE(__sdata = .);
    *(.data .data.*);
    . = ALIGN(4);
    PROVIDE(__edata = .);
  } > RAM AT>FLASH

  PROVIDE(__sidata = LOADADDR(.data));

  .bss (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    PROVIDE(__sbss = .);
    *(.bss .bss.*);
    . = ALIGN(4);
    PROVIDE(__ebss = .);
  } > RAM AT>FLASH

  /DISCARD/ :
  {
    *(.eh_frame);
    *(.debug*);
  }

  PROVIDE(__sheap = __ebss);
}
