use std::{ptr::null_mut, ffi::c_void};
use libopcodes_sys::{init_disassemble_info, sys::{disassemble_info, stream_state_t, bfd_architecture_bfd_arch_i386, bfd_mach_x86_64, buffer_read_memory, disassemble_init_for_target, disassembler, bfd_flavour_bfd_target_elf_flavour}};

pub struct Disassembler {
    disassemble: unsafe extern "C" fn(u64, *mut disassemble_info) -> i32,
}

pub enum Arch {
    I386,
}

pub enum Mach {
    X86_64,
}

static SUFFIX: &[u8] = b"suffix\0";

impl Disassembler {
    pub fn new(_arch: Arch, big: bool, _mach: Mach) -> Option<Disassembler> {
        let disasm = unsafe {
            disassembler(bfd_architecture_bfd_arch_i386, big, bfd_mach_x86_64 as u64, null_mut())
        };
        
        disasm.map(|disassemble| Disassembler {
            disassemble,
        })
    }

    pub fn disassemble<'a>(&'a mut self, input: &'a [u8]) -> Instructions<'a> {
        let mut disasm_info = disassemble_info::default();
        unsafe {
            init_disassemble_info(&mut disasm_info)
        }

        disasm_info.arch = bfd_architecture_bfd_arch_i386;
        disasm_info.mach = bfd_mach_x86_64 as u64;
        disasm_info.read_memory_func = Some(buffer_read_memory);
        disasm_info.buffer = input.as_ptr() as *mut u8;
        disasm_info.buffer_vma = 0;
        disasm_info.buffer_length = input.len();
        disasm_info.flavour = bfd_flavour_bfd_target_elf_flavour;
        disasm_info.disassembler_options = SUFFIX.as_ptr() as *const i8;
        unsafe {
            disassemble_init_for_target(&mut disasm_info);
        }
        
        Instructions {
            input,
            disasm_info,
            pc: 0,
            disassembler: self,
        }
    }
}

pub struct Instructions<'a> {
    input: &'a [u8],
    disasm_info: disassemble_info,
    pc: u64,
    disassembler: &'a Disassembler,
}

impl<'a> Iterator for Instructions<'a> {
    type Item = (&'a [u8], String);

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0u8; 4096];
        let mut stream = stream_state_t {
            ptr: buf.as_mut_ptr() as *mut _,
            len: buf.len() as u64,
        };

        let insn_size = unsafe {
            self.disasm_info.stream = &mut stream as *mut _ as *mut c_void;
            let insn_size = (self.disassembler.disassemble)(self.pc, &mut self.disasm_info);
            self.disasm_info.stream = null_mut();
            if insn_size < 0 {
                return None
            }
            
            insn_size as u64
        };

        let bytes = &self.input[self.pc as usize..(self.pc + insn_size) as usize];
        self.pc += insn_size;

        let str = std::str::from_utf8(&buf[..buf.len() - stream.len as usize])
            .unwrap();

        Some((bytes, str.to_owned()))
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{Disassembler, Arch, Mach};

    #[test]
    pub fn disasm_instruction_ret() {
        let mut d = Disassembler::new(Arch::I386, false, Mach::X86_64).unwrap();
        let mut instrs = d.disassemble(&[
            0xc3
        ]);

        assert_eq!(instrs.next(), Some(([ 0xc3, ].as_ref(), String::from("retq   "))));
        assert_eq!(instrs.next(), None);
    }

    #[test]
    pub fn disasm_instruction_addq() {
        let mut d = Disassembler::new(Arch::I386, false, Mach::X86_64).unwrap();
        let mut instrs = d.disassemble(&[
            0x48, 0x01, 0xD8
        ]);

        assert_eq!(instrs.next(), Some(([ 0x48, 0x01, 0xD8, ].as_ref(), String::from("addq   %rbx,%rax"))));
        assert_eq!(instrs.next(), None);
    }
}