# eighty-eighty
A Intel 8080 Emulator written in Rust

Compatible with Windows, Linux & Mac OS


## Emulator compatibility

* Passes all Intel 8080 CPU Tests.
* Space Invaders* 

*See compatibility.


### CPU Tests

#### The Intel 8080 Instruction Exerciser

*Note: The 8080EXM binary is modified for the 8080 by Ian Bartholemew. Based on the zexlax.z80 Z80 Instruction Exerciser by Frank D. Cringle, Copyright (C) 1994.*
*The 8080EXM binary is modified by Mike Douglas and prints the CRC along with OK or ERROR (works better for compatibility for the original Intel 8080).*

```
Test loaded: "8080EXM.COM" Bytes: 4608
8080 instruction exerciser
dad <b,d,h,sp>................  PASS! crc is:14474ba6
aluop nn......................  PASS! crc is:9e922f9e
aluop <b,c,d,e,h,l,m,a>.......  PASS! crc is:cf762c86
<daa,cma,stc,cmc>.............  PASS! crc is:bb3f030c
<inr,dcr> a...................  PASS! crc is:adb6460e
<inr,dcr> b...................  PASS! crc is:83ed1345
<inx,dcx> b...................  PASS! crc is:f79287cd
<inr,dcr> c...................  PASS! crc is:e5f6721b
<inr,dcr> d...................  PASS! crc is:15b5579a
<inx,dcx> d...................  PASS! crc is:7f4e2501
<inr,dcr> e...................  PASS! crc is:cf2ab396
<inr,dcr> h...................  PASS! crc is:12b2952c
<inx,dcx> h...................  PASS! crc is:9f2b23c0
<inr,dcr> l...................  PASS! crc is:ff57d356
<inr,dcr> m...................  PASS! crc is:92e963bd
<inx,dcx> sp..................  PASS! crc is:d5702fab
lhld nnnn.....................  PASS! crc is:a9c3d5cb
shld nnnn.....................  PASS! crc is:e8864f26
lxi <b,d,h,sp>,nnnn...........  PASS! crc is:fcf46e12
ldax <b,d>....................  PASS! crc is:2b821d5f
mvi <b,c,d,e,h,l,m,a>,nn......  PASS! crc is:eaa72044
mov <bcdehla>,<bcdehla>.......  PASS! crc is:10b58cee
sta nnnn / lda nnnn...........  PASS! crc is:ed57af72
<rlc,rrc,ral,rar>.............  PASS! crc is:e0d89235
stax <b,d>....................  PASS! crc is:2b0471e9
Tests complete
Jump to 0 from 0137
```




#### Diagnostics II v1.2 by by Supersoft Associates (1981):

```
Test loaded: "CPUTEST.COM" Bytes: 19200
      
DIAGNOSTICS II V1.2 - CPU TEST
COPYRIGHT (C) 1981 - SUPERSOFT ASSOCIATES

ABCDEFGHIJKLMNOPQRSTUVWXYZ
CPU IS 8080/8085
BEGIN TIMING TEST
END TIMING TEST
CPU TESTS OK

Jump to 0 from 3B25
```
#### Microcosm Associates 8080/8085 CPU Diagnostics v1.0:
```
Test loaded: "TEST.COM" Bytes: 1793
MICROCOSM ASSOCIATES 8080/8085 CPU DIAGNOSTIC VERSION 1.0  (C) 1980

CPU IS OPERATIONAL
Jump to 0 from 014F
test test::tests::test_com ... ok
```
#### Preliminary Exerciser (*prelim z80 by Frank D. Cringle, modified by Ian Bartholemew for the 8080*):
``` 
Test loaded: "8080PRE.COM" Bytes: 1024
8080 Preliminary tests complete
Jump to 0 from 032F
test test::tests::preliminary ... ok
```
--- 


### Space Invaders
![Invaders](https://github.com/stianeklund/eighty-eighty/blob/master/assets/screenshot.png)



##### Compatiblity

* No sound implemented.
* No user input.
* Shift registers implemented.
* Interrupts (but have bugs).

---

## How to build & run Eighty Eighty

#### Running CPU tests:

With Rust & cargo installed:

Run tests from the terminal you can use `cargo test` or, for `stdout` output:

Test names:
* `test::tests::preliminary`
* `test::tests::cpu_exer`
* `test::tests::cpu_test`
* `test::tests::test_com`


E.g: `cargo test --package eighty-eighty --bin eighty-eighty test::tests::preliminary -- --nocapture --exact`


* Note there is currently no support for other games nor separate game files.

If you have multiple files you can merge them with `cat` or a similar util.

---

### References used:

* http://computerarcheology.com/Arcade/SpaceInvaders/
* http://www.emulator101.com/welcome.html
* http://www.pastraiser.com/cpu/i8080/i8080_opcodes.html
* [The Intel 8080 Programmers Manual](http://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf)
* [The Reddit Emudev community & Slack channels](https://reddit.com/r/emudev)
* https://github.com/begoon/i8080-core
* [CPU Tests](altairclone.com/downloads/cpu_tests/)
