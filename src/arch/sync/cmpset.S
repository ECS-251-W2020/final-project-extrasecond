/*
 * Atomic compare values and set new if equal. Returns previous.
 *  int atomic_cmpset(void* p, int c, int t)
 */
 
.global atomic_cmpset
.type   atomic_cmpset, %function

atomic_cmpset:
    mov   x4, x0
_atomic_cmpset:
    ldxr  w0, [x4]
    cmp   w1, w0
    bne   _exit_err
    stxr  w3, w2, [x4]
    cbnz  w3, _atomic_cmpset
    dmb   sy
    ret

_exit_err:
    clrex
    ret