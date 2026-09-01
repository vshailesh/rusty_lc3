
enum KeyboardReg {
    MR_KBSR = 0xFE00,
    MR_KBDR = 0xFE02
}

enum TrapCodes {
    TRAP_GETC,
    TRAP_OUT,
    TRAP_PUTS,
    TRAP_IN,
    TRAP_PUTSP,
    TRAP_HALT,
}

const MEMORY_MAX: u16 = 65535;
static mut memory: [u16; MEMORY_MAX as usize] = [0_u16; MEMORY_MAX as usize];

enum Registers {
    R_R0 = 0,
    R_R1,
    R_R2,
    R_R3,
    R_R4,
    R_R5,
    R_R6,
    R_R7,
    R_PC,
    R_COND,
    R_COUNT
}

static mut reg: [u16; Registers::R_COUNT as usize] = [0_u16; Registers::R_COUNT as usize];

enum OPCodes {
    OP_BR = 0, 
    OP_ADD, 
    OP_LD,
    OP_ST,
    OP_JSR,
    OP_AND,
    OP_LDR,
    OP_STR,
    OP_RTI,
    OP_NOT,
    OP_LDI,
    OP_STI,
    OP_JMP,
    OP_RES,
    OP_LEA,
    OP_TRAP
}

enum ConditionFlags {
    FL_POS = 1 << 0_u16,
    FL_ZRO = 1 << 1_u16,
    FL_NEG = 1 << 2_u16,
}

/*
    Handle these things here 
    1. terminal io 
    2. disable_input_buffering
    3. restore_input_buffering,
    4. check_key ()
    5. handle_interrupt()
*/

fn sign_extend(mut x: u16, bit_count: i32) -> u16 {
    let expr = (x >> (bit_count -1)) & 1;
    if expr == 1 {
        x |= 0xFFFF << bit_count;
    }
    return x;
}

fn swap16(x: u16) -> u16 {
    x << 8 | x >> 8
}

fn update_flags(r: u16) {
    unsafe {
        if reg[r as usize] == 0 {
            reg[Registers::R_COND as usize] = ConditionFlags::FL_ZRO as u16;
        } else if reg[r as usize] >> 15 == 1 {
            reg[Registers::R_COND as usize] = ConditionFlags::FL_NEG as u16;
        } else {
            reg[Registers::R_COND as usize] = ConditionFlags::FL_POS as u16;
        }
    }
}

fn read_image_file() {

}

fn read_image() -> i32{
    32
}

fn mem_write(address: u16, val: u16) {
    unsafe {
        memory[address as usize] = val;
    }
}

fn mem_read(address: u16) -> u16 {
    unsafe{
        if address == KeyboardReg::MR_KBSR as u16 {
            if check_key() {
                memory[KeyboardReg::MR_KBSR as usize] = 1 << 15;
                memory[KeyboardReg::MR_KBDR as usize] = //some input function in rust
            } else {
                memory[KeyboardReg::MR_KBSR as usize] = 0;
            }
        }
    }
    memory[address as usize]  
}

fn fn_op_add(instr: u16) {
    let r0: u16 = (instr >> 9) & 0x7;
    let r1: u16 = (instr >> 6) & 0x7;
    let imm_flag: u16 = (instr >> 5) & 0x1;

    unsafe {
        if imm_flag == 1 {
            let imm5: u16 = sign_extend(instr & 0x1F, 5);
            reg[r0 as usize] = reg[r1 as usize] + imm5;
        } else {
            let r2: u16 = instr & 0x7;
            reg[r0 as usize] = reg[r1 as usize] + reg[r2 as usize];
        }
    }
    update_flags(r0);
}

fn fn_op_and(instr: u16) {
    let r0: u16 = (instr >> 9) & 0x7;
    let r1: u16 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    unsafe {
        if imm_flag == 1 {
            let imm5: u16 = sign_extend(instr & 0x1F, 5);
            reg[r0 as usize] = reg[r1 as usize] & imm5;
        } else {
            let r2: u16 = instr & 0x7;
            reg[r0 as usize] = reg[r1 as usize] & reg[r2 as usize];
        }
    }
    update_flags(r0);
}

fn fn_op_not(instr: u16) {
    let r0: u16 = (instr >> 9) & 0x7;
    let r1: u16 = (instr >> 6) & 0x7;
    unsafe {
        reg[r0 as usize] = !reg[r1 as usize];
    }
    update_flags(r0);
}

fn fn_op_br(instr: u16) {
    let pc_offset: u16 = sign_extend(instr & 0x1FF, 9);
    let cond_flag: u16 = (instr >> 9) & 0x7;
    unsafe {
        if cond_flag & reg[Registers::R_COND as usize] == 1 {
            reg[Registers::R_PC as usize] = reg[Registers::R_PC as usize] + pc_offset;
        }
    }
}

fn fn_op_jmp(instr: u16) {
    let r1: u16 = (instr >> 6) & 0x7;
    unsafe {
        reg[Registers::R_PC as usize] = reg[r1 as usize];
    }
}

fn fn_op_jsr(instr: u16) {
    unsafe {
        reg[Registers::R_R7 as usize] = reg[Registers::R_PC as usize];
        if ((instr >> 11) & 0x1) == 0 {
            let BaseR: u16 = (instr >> 6) & 0x7;
            reg[Registers::R_PC as usize] = reg[BaseR as usize];
        } else {
            reg[Registers::R_PC as usize] = reg[Registers::R_PC as usize] + sign_extend((instr >> 0x7FF), 11);
        }
    }
}

fn fn_op_ld(instr: u16) {
    let r0: u16 = (instr >> 9) & 0x7;
    unsafe {
        reg[r0 as usize] = mem_read(reg[Registers::R_PC as usize] + sign_extend(instr & 0x1FF, 9));
    }
    update_flags(r0);
}

fn fn_op_ldi(instr: u16) {
    let r0: u16 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    unsafe {
        reg[r0 as usize] = mem_read(mem_read(reg[Registers::R_PC as usize] + pc_offset));
    }
    update_flags(r0);
}

fn fn_op_ldr(instr: u16) {
    let r0: u16 = (instr >> 9) & 0x7;
    let BaseR: u16 = (instr >> 6) & 0x7;
    let pc_offset6 = (instr & 0x3F);

    unsafe {
        reg[r0 as usize] = mem_read(reg[BaseR as usize] + sign_extend(pc_offset6, 6));
    }
    update_flags(r0);
}

fn fn_op_lea(instr: u16) {
    let r0: u16 = (instr >> 9) & 0x7;
    let pc_offset9 = instr & 0x1FF;
    unsafe {
        reg[r0 as usize] = reg[Registers::R_PC as usize] + sign_extend(pc_offset9, 9);
    }
    update_flags(r0);
}

fn fn_op_st(instr: u16) {
    let r0: u16 = (instr >> 9) & 0x7;
    let pc_offset9 = sign_extend(instr & 0x1FF, 9);
    unsafe {
        mem_write(reg[Registers::R_PC as usize] + pc_offset9, reg[r0 as usize]);
    }
}

fn fn_op_sti(instr: u16) {
    let r0: u16 = (instr >> 9) & 0x7;
    let pc_offset9: u16 = sign_extend(instr & 0x1FF, 9);
    unsafe {
        mem_write(mem_read(reg[Registers::R_PC as usize]), reg[r0 as usize]);
    }
}

fn fn_op_str(instr: u16) {
    let r0: u16 = (instr >> 9) & 0x7;
    let r1: u16 = (instr >> 6) & 0x7;
    let pc_offset6 = sign_extend(instr & 0x3F, 6);
    unsafe {
        mem_write(reg[r1 as usize] + pc_offset6, reg[r0 as usize]);
    }
}

impl From<u16> for OPCodes {
    fn from(item: u16) -> Self {
        if item == 0_u16 {
            OPCodes::OP_BR
        } else if item == 1_u16 {
            OPCodes::OP_ADD
        } else if item == 2_16 {
            OPCodes::OP_LD
        } else if item == 3_16 {
            OPCodes::OP_ST
        } else if item == 2_16 {
            OPCodes::OP_JSR
        } else if item == 2_16 {
            OPCodes::OP_AND
        } else if item == 2_16 {
            OPCodes::OP_LDR
        } else if item == 2_16 {
            OPCodes::OP_LDR
        } else if item == 2_16 {
            OPCodes::OP_STR
        } else if item == 2_16 {
            OPCodes::OP_RTI
        } else if item == 2_16 {
            OPCodes::OP_NOT
        } else if item == 2_16 {
            OPCodes::OP_JMP
        } else if item == 2_16 {
            OPCodes::OP_RES
        } else if item == 2_16 {
            OPCodes::OP_LEA
        } else {
            OPCodes::OP_TRAP
        }
    }
}

impl From<u16> for TrapCodes {
    fn from(item: u16) -> Self {
        let hexval = format!("{:X}", item);
        match hexval.as_str() {
            "20" => {
                TrapCodes::TRAP_GETC
            },
            "21" => {
                TrapCodes::TRAP_OUT
            },
            "22" => {
                TrapCodes::TRAP_PUTS
            }
            "23" => {
                TrapCodes::TRAP_IN
            }, 
            "24" => {
                TrapCodes::TRAP_PUTSP
            },
            "25" => {
                TrapCodes::TRAP_HALT
            },
            _ => {
                std::process::abort()
            }
        }
    }
}

fn main() {
    
    unsafe {
        reg[Registers::R_COND as usize] = ConditionFlags::FL_ZRO as u16;
    }

    let PC_START: u16 = 0x3000;
    
    unsafe {
        reg[Registers::R_PC as usize] = PC_START;
    }

    let running: bool = true;
    while(running) {
        unsafe {
            let instr: u16 = mem_read(reg[Registers::R_PC as usize]);
            reg[Registers::R_PC as usize] += 1;
            let op: u16 = instr >> 12;
            
            match OPCodes::from(op) {
                OPCodes::OP_ADD => {
                    fn_op_add(instr);
                },
                OPCodes::OP_AND => {
                    fn_op_and(instr);
                },
                OPCodes::OP_NOT => {
                    fn_op_not(instr);
                }, 
                OPCodes::OP_BR => {
                    fn_op_br(instr);
                },
                OPCodes::OP_JMP => {
                    fn_op_jmp(instr);
                },
                OPCodes::OP_JSR => {
                    fn_op_jsr(instr);
                },
                OPCodes::OP_LD => {
                    fn_op_ld(instr);
                },
                OPCodes::OP_LDI => {
                    fn_op_ldi(instr);
                },
                OPCodes::OP_LDR => {
                    fn_op_ldr(instr);
                }, 
                OPCodes::OP_LEA => {
                    fn_op_lea(instr);
                }, 
                OPCodes::OP_ST => {
                    fn_op_st(instr);
                },
                OPCodes::OP_STI => {
                    fn_op_sti(instr);
                },
                OPCodes::OP_TRAP => {
                    unsafe {
                        reg[Registers::R_R7 as usize] = reg[Registers::R_PC as usize];
                        let match_val = instr & 0xFF;
                        let iohandle = std::io::stdin();
                        match TrapCodes::from(match_val) {
                            TrapCodes::TRAP_GETC => {
                                unsafe {
                                    // reg[Registers::R_R0 as usize] = iohandle.
                                }
                            },
                            TrapCodes::TRAP_OUT => {

                            }, 
                            TrapCodes::TRAP_PUTS => {

                            }, 
                            TrapCodes::TRAP_IN => {

                            }, 
                            TrapCodes::TRAP_PUTSP => {

                            },
                            TrapCodes::TRAP_HALT => {

                            }
                        } 
                    }
                }
                OPCodes::OP_RES => {
                    ()
                },
                OPCodes::OP_RTI => {
                    ()
                }
                _ => {
                    std::process::abort();
                }
            }
        }  
    }
    // call restore_input_buffering()

}