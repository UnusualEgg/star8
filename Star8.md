## Specs
### regs
Size=8 bits

A, B, C, D
### memory
Size=64 bytes

Width=8 bits/1 byte

### address bus
Size=8 bits/1 byte

Max: 0xff/65535

Also has tmp address only set by micro instruction

### micro instructions
- Read mem(toggle)
- Write mem
- Count up(inc address counter)
- Counter load(single ticked)
- Set A
- Set B
- Set C
- Set D
- Read A(toggle)
- Read B(toggle)
- Read C(toggle)
- Read A(toggle)
- Set ALU A
- Set ALU B
- ALU ADD
- ALU SUB
- ALU INC
- ALU Compute(toggle)
- Set temp addr
- Read temp addr(toggle)
- Halt

## Args

### Single Register arg
- A=1000
- B=0100
- C=0010
- D=0001
- None=----(doesn't do anything but uses the same amount of cycles)


## Operating Modes
### **key**
#### symbols
- s=opcode
- r=register1
- R=register2
- A=imm8 **or** addr8
- c=condition
- -=doesn't matter
#### Byte order
\[7654 3210] (left is most significant value)

### Single register mode
ssss rrrr

### Double register mode
ssss rrRR

### Reg-byte mode(imm8/addr8)
ssss rrrr
AAAA AAAA

### Byte mode(addr8/imm8)
ssss ----
AAAA AAAA

### Implied mode
ssss ----

### Condition
ssss cccc

| condition | bits(cccc) |
| --------- | ---------- |
| A\==0     | 0000       |
| A\==B     | 1000       |
| ALU ZERO  | 0100       |
| ALU OVF   | 0010       |
| ALU NEG   | 0001       |


## Instructions

### **key**
xxxx aaaa

x=opcode
a=args

### list of opcodes

| Binary | Decimal | Instruction name | Opmode          |
| ------ | ------- | ---------------- | --------------- |
| 0000   | 0       | NOP              | implied         |
| 0001   | 1       | INC              | single register |
| 0010   | 2       | ADD              | two registers   |
| 0011   | 3       | SUB              | two registers   |
| 0100   | 4       | LDI              | register+imm8   |
| 0101   | 5       | STM              | register+addr8  |
| 0110   | 6       | JMP              | addr8           |
| 0111   | 7       | JIF              | condition       |
| 1000   | 8       | HLT              | implied         |
| 1001   | 9       | LDA              | register+addr8  |


## Instruction Descriptions

### Nop
Ins=0000 (or maybe any unused opcode)

0000 ----

### Add
(r1+=R2)

### Sub
(r1-=R2)

### LTA/LDI (load imm to register)
(LDI in assembly)

Ins=0100

Mode=reg-byte (imm8)

### LDA (mem -> reg/accumulator)
Ins=1001

Mode=reg-byte (addr8)

(reg=mem\[imm8/addr8])

### STM (reg - address)
Ins=0101

Mode=reg-byte (addr8)

(mem\[imm8/addr8]=reg)

### JUMP IF(JIF)
(Not gonna use JIF in assembly)

Ins=0111 AAAA

| Condition | Bits | Instruction                    |
| --------- | ---- | ------------------------------ |
| A\==0     | 0000 | *JAZ* \[Jump A zero]           |
| A\==B     | 0001 | *JEQ* \[Jump EQual]            |
| ZERO      | 0010 | *JIZ* (ALU\==0)\[jump if zero] |
| OVF       | 0100 | *JOV* \[Jump OVerflow]         |
| NEG       | 1000 | *JNG* \[Jump NeGative]         |

Mode=imm8

halts w/o halt flag set when A isn't a predefined condition aka a power of 2

### HALT
Ins=1000 ----

Mode=implied
