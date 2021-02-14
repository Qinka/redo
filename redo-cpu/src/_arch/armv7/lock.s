.section ".text"

.global asm_try_lock

asm_try_lock:
    mov         r1, #0 // r1 -> 0
    // stores the value in r1 into the address in r0, and stores the value at the address in r0 into r2
    swp         r2, r1, [r0]
    mov         r0, r2
    blx         lr
