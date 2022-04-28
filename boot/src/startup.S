.equ REG_BYTES,      4
.equ MSTATUS_MIE,   (1 << 3)
.equ MSTATUS_MPP,   (0b11 << 11)

.equ mstatus,       0x300
.equ mie,           0x304
.equ mtvec,         0x305
.equ mtvt,          0x307
.equ msubm,         0x7c4
.equ mtvt2,         0x7ec
.equ jalmnxti,      0x7ed
.equ pushmcause,    0x7ee
.equ pushmepc,      0x7ef
.equ pushmsubm,     0x7eb

.macro SAVE_REGS
    sw ra, 0*REG_BYTES(sp)
    sw tp, 1*REG_BYTES(sp)
    sw t0, 2*REG_BYTES(sp)
    sw t1, 3*REG_BYTES(sp)
    sw t2, 4*REG_BYTES(sp)
    sw t3, 5*REG_BYTES(sp)
    sw t4, 6*REG_BYTES(sp)
    sw t5, 7*REG_BYTES(sp)
    sw t6, 8*REG_BYTES(sp)
    sw a0, 9*REG_BYTES(sp)
    sw a1, 10*REG_BYTES(sp)
    sw a2, 11*REG_BYTES(sp)
    sw a3, 12*REG_BYTES(sp)
    sw a4, 13*REG_BYTES(sp)
    sw a5, 14*REG_BYTES(sp)
    sw a6, 15*REG_BYTES(sp)
    sw a7, 16*REG_BYTES(sp)
.endm

.macro SAVE_CSR
    csrw pushmcause, 17
    csrw pushmepc, 18
    csrw pushmsubm, 19
.endm

.macro RESTORE_REGS
    lw ra, 0*REG_BYTES(sp)
    lw tp, 1*REG_BYTES(sp)
    lw t0, 2*REG_BYTES(sp)
    lw t1, 3*REG_BYTES(sp)
    lw t2, 4*REG_BYTES(sp)
    lw t3, 5*REG_BYTES(sp)
    lw t4, 6*REG_BYTES(sp)
    lw t5, 7*REG_BYTES(sp)
    lw t6, 8*REG_BYTES(sp)
    lw a0, 9*REG_BYTES(sp)
    lw a1, 10*REG_BYTES(sp)
    lw a2, 11*REG_BYTES(sp)
    lw a3, 12*REG_BYTES(sp)
    lw a4, 13*REG_BYTES(sp)
    lw a5, 14*REG_BYTES(sp)
    lw a6, 15*REG_BYTES(sp)
    lw a7, 16*REG_BYTES(sp)
.endm

.macro RESTORE_CSR
    lw t0, 17*REG_BYTES(sp)
    csrw mcause, t0
    lw t0, 18*REG_BYTES(sp)
    csrw mepc, t0
    lw t0, 19*REG_BYTES(sp)
    csrw msubm, t0
.endm

.macro SAVE_CONTEXT
    addi sp, sp, -20*REG_BYTES
    SAVE_REGS
    SAVE_CSR
.endm

.macro RESTORE_CONTEXT
    RESTORE_CSR
    RESTORE_REGS
    addi sp, sp, 20*REG_BYTES
.endm

# Contains the Reset Vector and the rest of the Interrupt Vector Table
.section .vectors, "ax"
.global vector_table
vector_table:
    j _start
    .word     0
    .word     0
    .word     ECLIC_SFT_IRQHandler
    .word     0
    .word     0
    .word  	  0
    .word  	  ECLIC_TMR_IRQHandler
    .word  	  0
    .word  	  0
    .word  	  0
    .word  	  0
    .word  	  0
    .word  	  0
    .word  	  0
    .word 	  0
    .word  	  0
    .word  	  ECLIC_BWEI_IRQHandler
    .word  	  ECLIC_PMOVI_IRQHandler
    .word  	  WWDGT_IRQHandler
	.word     LVD_IRQHandler
	.word     TAMPER_IRQHandler
	.word     RTC_IRQHandler
	.word     FMC_IRQHandler
	.word     RCU_IRQHandler
	.word     EXTI0_IRQHandler
	.word     EXTI1_IRQHandler
	.word     EXTI2_IRQHandler
	.word     EXTI3_IRQHandler
	.word     EXTI4_IRQHandler
	.word     DMA0_Channel0_IRQHandler
	.word     DMA0_Channel1_IRQHandler
	.word     DMA0_Channel2_IRQHandler
	.word     DMA0_Channel3_IRQHandler
	.word     DMA0_Channel4_IRQHandler
	.word     DMA0_Channel5_IRQHandler
	.word     DMA0_Channel6_IRQHandler
	.word     ADC0_1_IRQHandler
	.word     CAN0_TX_IRQHandler
	.word     CAN0_RX0_IRQHandler
	.word     CAN0_RX1_IRQHandler
	.word     CAN0_EWMC_IRQHandler
	.word     EXTI5_9_IRQHandler
	.word     TIMER0_BRK_IRQHandler
	.word     TIMER0_UP_IRQHandler
	.word     TIMER0_TRG_CMT_IRQHandler
	.word     TIMER0_Channel_IRQHandler
	.word     TIMER1_IRQHandler
	.word     TIMER2_IRQHandler
	.word     TIMER3_IRQHandler
	.word     I2C0_EV_IRQHandler
	.word     I2C0_ER_IRQHandler
	.word     I2C1_EV_IRQHandler
	.word     I2C1_ER_IRQHandler
	.word     SPI0_IRQHandler
	.word     SPI1_IRQHandler
	.word     USART0_IRQHandler
	.word     USART1_IRQHandler
	.word     USART2_IRQHandler
	.word     EXTI10_15_IRQHandler
	.word     RTC_Alarm_IRQHandler
	.word     USBFS_WKUP_IRQHandler
    .word     0
    .word     0
    .word     0
    .word     0
	.word     0
	.word     EXMC_IRQHandler
	.word     0
	.word     TIMER4_IRQHandler
	.word     SPI2_IRQHandler
	.word     UART3_IRQHandler
	.word     UART4_IRQHandler
	.word     TIMER5_IRQHandler
	.word     TIMER6_IRQHandler
	.word     DMA1_Channel0_IRQHandler
	.word     DMA1_Channel1_IRQHandler
	.word     DMA1_Channel2_IRQHandler
	.word     DMA1_Channel3_IRQHandler
	.word     DMA1_Channel4_IRQHandler
    .word     0
    .word     0
	.word     CAN1_TX_IRQHandler
	.word     CAN1_RX0_IRQHandler
	.word     CAN1_RX1_IRQHandler
	.word     CAN1_EWMC_IRQHandler
	.word     USBFS_IRQHandler

.section .init, "ax"
.global _start
_start:
    .option push
    .option norelax
    la gp, __global_pointer$
    .option pop

    la sp, _stack_top

    # Setup exception trap
    la t0, exception_trap
    csrw mtvec, t0

    # Load interrupt vector table
    la t0, vector_table
    csrw mtvt, t0

    # Setup interrupt service routine handler
    la t0, irq_trap
    ori t0, t0, 1
    csrw mtvt2, t0

    call init_data
    call init_bss
    call kernel_init

    # Setup user mode next
    la t0, user_main
    csrw mepc, t0
    li t0, MSTATUS_MPP
    csrc mstatus, t0

    # Enable global interrupts
    csrs mstatus, MSTATUS_MIE

    # Jump to user mode
    mret

.section .init, "ax"
.global init_data
init_data:
    la t0, _sdata
    la t1, _sidata
    la t2, _sdata
    bgeu t0, t2, 2f
1:
    lw t3, 0(t1)
    sw t3, 0(t0)
    addi t0, t0, 4
    addi t1, t1, 4
    bltu t0, t2, 1b
2:
    ret

.section .init, "ax"
.global init_bss
init_bss:
    la t0, _sbss
    la t1, _ebss
    bgeu t0, t1, 2f
1:
    sw zero, 0(t0)
    addi t0, t0, 4
    bltu t0, t1, 1b
2:
    ret

.section .init.trap, "ax"
.global exception_trap
exception_trap:
    SAVE_CONTEXT

    mv a0, sp
    # Must return the updated mepc value
    jal ra, Exception_Handler
    csrw mepc, a0

    RESTORE_CONTEXT

    # Enable global interrupts
    csrs mstatus, MSTATUS_MIE

    # Back to previous machine mode
    mret

.section .init.trap, "ax"
.global irq_trap
irq_trap:
    SAVE_CONTEXT

    mv a0, sp

    # Jump to interrupt vector table entry and allow pre-empting
    csrrw ra, jalmnxti, ra

    # Disable global interrupts 
    csrc mstatus, MSTATUS_MIE

    RESTORE_CONTEXT

    # Back to previous machine mode
    mret

.section .text.handler, "ax"
.global Default_Exception_Handler
Default_Exception_Handler:
    lw a0, 18*REG_BYTES(a0)
    addi a0, a0, 4
    ret

.section .text.handler, "ax"
.global Default_IRQHandler
Default_IRQHandler:
    ret

.section .text, "ax"
.global idle
idle:
    wfi
    j idle

.section .text, "ax"
.global abort
abort:
    j abort