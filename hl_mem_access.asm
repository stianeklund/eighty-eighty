    org 100h
    lxi hl, 4110 ; 100e
    lxi hl, hlval
    mov a,m
    cpi 0a5h
    jnz 0
    lxi h, hlval +1
    mov a,m
    cpi 03ch
    jnz 0

    hlval: db 0a5h, 03ch
