use crate::exception::Exception;

// TODO: paste reference
pub const REGISTERS_COUNT: usize = 32;

pub const POINTER_TO_DTB: u64 = 0x1020;
pub struct Cpu {
    pub xregs: XRegisters,
    pub pc: u64,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            xregs: XRegisters::new(),
            pc: 0,
        }
    }
}

#[derive(Debug)]
pub struct XRegisters {
    xregs: [u64; REGISTERS_COUNT],
}

impl XRegisters {
    pub fn new() -> Self {
        let mut xregs = [0; REGISTERS_COUNT];
        // P.14 There is no dedicated stack pointer or subroutine return address link register in the Base Integer ISA; 
        // the instruction encoding allows any x register to be used for these purposes. However,
        // the standard software calling convention uses register x1 to hold the return address for a call,
        // with register x5 available as an alternate link register. The standard calling convention uses register x2 as the stack pointer.

        xregs[2] = DRAM_BASE + DRAM_SIZE;

        // x10 := a0, x11 := a1
        // from rvemu
        // ```
        // From riscv-pk:
        // https://github.com/riscv/riscv-pk/blob/master/machine/mentry.S#L233-L235
        //   save a0 and a1; arguments from previous boot loader stage:
        //   // li x10, 0
        //   // li x11, 0
        // void init_first_hart(uintptr_t hartid, uintptr_t dtb)
        //   x10 (a0): hartid
        //   x11 (a1): pointer to dtb
        // So, we need to set registers register to the state as they are when a bootloader finished.
        // ```
        // ref: P. 100
        // DTB.. Device Tree Blob
        // ref: https://qiita.com/koara-local/items/ed99a7b96a0ca252fc4e
        xregs[10] = 0;
        xregs[11] = POINTER_TO_DTB;
        Self { xregs: xregs }
    }

    pub fn read(&self, index: u64) -> u64 {
        self.xregs[index as usize]
    }

    pub fn write(&mut self, index: u64, value: u64) {
        if index != 0 {
            self.xregs[index as usize] = value;
        }
    }
}
impl Cpu {
    
fn execute(&mut self, inst: u64) -> Result<(), Exception> {
    // 0~6
    let opcode = inst & 0b111111;
    // 7~11
    let rd = (inst >> 7) & 0b11111;
    // 15~19
    let rs1 = (inst >> 15) & 0b11111;
    // 20~25
    let rs2 = (inst >> 20) & 0b11111;
    // 12~14
    let funct3 = (inst >> 12) & 0b111;
    match opcode {
        // 0x33 R-Type
        0b0110011 => {
            match 

        },
        // 0x13 I-Type
        0b0010011 =>  {
            // そりゃ演算が符号付きですから...? 即値って負もあるんか？
            // P.18 sign-extended 12-bit immediate
            // ありそう
            let imm = ((inst as i32 as i64) >> 20) as u64;
            let imm = (inst >> 20);
            match funct3 {
                0b000 => {
                    // wrapping
                    self.xregs.write(rd, self.xregs.read(rs1).wrapping_add(imm));
                },
                    _ => unimplemented!()
            }
        },
        _ => {
            return Err(Exception::IllegalInstruction(inst))
        }
    }
}
}

#[cfg(test)]
mod tests {
    #[test]
    fn 
}