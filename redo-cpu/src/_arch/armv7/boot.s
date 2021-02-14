
.section ".text._boot"

.global _boot_config
.global _boot

_boot: // _boot will be executed first.
    // Read process id
    mrc     p15, #0, r0, c0, c0, #5
    // If id is not 0, jump to secondary_core_launch
    tst     r0, #3
    bne     _secondary
    b       _primary
_primary: // For primary core
    ldr     r0, =_boot_config
    ldr     sp, [r0]
    @ ldr     sp, r0 //
    @ mov     r0, [_boot_config + 0x0]
    @ mov     sp, [r0]
    b primary_core_launch
_secondary: // For secondary core
    // Other inits
    // Jump to secondary_core_launch
    b secondary_core_launch


@ .sp_addr:
@     .word 0x8000
@     .word 0x7000
@     .word 0x6000
@     .word 0x5000

@ _start:
@                                     // aka first address before label _start
@     mrc     p15, 0, r0, c0, c0, 5   // Read multiprocessor affinity register
@     and     r0, r0, #3              // Clear all bits except [1:0], which hold core id
@     cmp     r0, #0                  // If core 0, jump to _load_core0
@     beq     _init_in_core0
@ _slave_core_init:
@     @ b       _wait                   // Remove if enable other slave cores
@     mrc     p15, 0, r0, c0, c0, 5
@     and     r0, r0, #3
@     cmp     r0, #1                  // If core 1, jump to _load_core1
@     beq     _load_core1
@     cmp     r0, #2
@     beq     _load_core2             // If core 2, jump to _load_core2
@     cmp     r0, #3
@     beq     _load_core3             // If core 3, jump to _load_core3
@     b       _wait
@ _wait:                              // wait and loop
@     wfe
@     b       _wait
@ _init_in_core0:
@ _zero_bss:
@     ldr     r4, =__bss_start
@     ldr     r9, =__bss_end
@     mov     r5, #0
@     mov     r6, #0
@     mov     r7, #0
@     mov     r8, #0
@     b       _is_end_bss
@ _do_zero:
@     stmia   r4!, {r5-r8}
@ _is_end_bss:
@     cmp     r4, r9
@     blo     _do_zero
@ _load_core0:                        // Here, if core 0
@     ldr     sp, .sp_addr             // Set start of stack to before our code,
@     @ ldr     r0, =core0_init       // Jump to core0_init
@     bl      core0_init
@     b       _wait             // Should never be here - jump to _wait
@ _load_core1:                        // Here, if core 1. Same with core 0
@     ldr     sp, .sp_addr+4             // Set start of stack to before our code,
@     @ ldr     r0, =core1_init             // Jump to core1_init
@     bl      core1_init
@     b       _wait             // Should never be here - jump to _wait
@ _load_core2:                        // Here, if core 2. Same with core 0
@     ldr     sp, .sp_addr+8             // Set start of stack to before our code,
@     @ ldr     r0, =core2_init             // Jump to core2_init
@     bl      core2_init
@     b       _wait             // Should never be here - jump to _wait
@ _load_core3:                        // Here, if core 3. Same with core 0
@     ldr     sp, .sp_addr+12             // Set start of stack to before our code,
@     @ ldr     r0, =core3_init             // Jump to core3_init
@     bl      core3_init
@     b       _wait             // Should never be here - jump to _wait
