struct CPU {
    registers: [u8; 16],
    program_counter: usize,
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn read_op_code(&self) -> u16 {
        let counter = self.program_counter;
        let op_byte1 = self.memory[counter] as u16;
        let op_byte2 = self.memory[counter + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let op_code = self.read_op_code();
            self.program_counter += 2;

            let nnn = op_code & 0xFFF;

            let c = ((op_code & 0xF000) >> 12) as u8;
            let x = ((op_code & 0x0F00) >> 8) as u8;
            let y = ((op_code & 0x00F0) >> 4) as u8;
            let d = ((op_code & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!(),
            }
        }
    }

    fn call(&mut self, addr: u16) {
        let stack_pointer = self.stack_pointer;
        let stack = &mut self.stack;

        if stack_pointer > stack.len() {
            panic!("Stack overflow")
        }

        stack[stack_pointer] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.program_counter = call_addr as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        if let Some(sum) = arg1.checked_add(arg2) {
            self.registers[x as usize] = sum;
            self.registers[0xF] = 0;
        } else {
            self.registers[0xF] = 1;
        }
    }
}

fn main() {
    let mut cpu = CPU {
        program_counter: 0,
        memory: [0; 4096],
        registers: [0; 16],
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    mem[0x000] = 0x21;
    mem[0x001] = 0x00;
    mem[0x002] = 0x21;
    mem[0x003] = 0x00;
    mem[0x004] = 0x00;
    mem[0x005] = 0x00;

    mem[0x100] = 0x80;
    mem[0x101] = 0x14;
    mem[0x102] = 0x80;
    mem[0x103] = 0x14;
    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);

    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0])
}
