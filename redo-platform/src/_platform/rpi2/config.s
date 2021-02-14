.section ".text._boot_config"

.global _boot_config
_boot_config:
    .word 0x8000 // stack pointer for primary core when launching.
