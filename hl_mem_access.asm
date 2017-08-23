    org 100h
    lxi hl, 4110
    lxi hl, hlval
    mov a,m
    cpi 0a5h
    jnz 0
    lxi h, hlval +1
    mov a,m
    cpi 03ch
    jnz 0
    hlt
    hlval: db 0a5h, 03ch
