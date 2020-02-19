.section ".text.boot"

.globl _start
_start:
    mrs x0, mpidr_el1
    bic x0, x0, #3
    cbz x0, master
    b   proc_hang

proc_hang:
    b   proc_hang

master:
    ldr x0, =_start
    mov sp, x0
    bl  runtime_init
    b   proc_hang
