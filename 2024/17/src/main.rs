use std::fs::File;
use std::io::Read;

// Instruction holds opcode and operand
type Instruction = (u8, u64);

#[derive(Debug)]
struct Processor {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rip: u64,
    code: Vec<Instruction>,
    output: Vec<u8>
}

impl Processor {
    fn _parse_reg(&mut self, reg_type: usize, input: &str) {
        let value = input.split(':').collect::<Vec<&str>>()[1].trim().parse::<u64>().unwrap();
        match reg_type {
            0 => self.rax = value,
            1 => self.rbx = value,
            2 => self.rcx = value,
            _ => panic!("Invalid register")
        };
    }

    fn _parse_code(&mut self, input: &str) {
        let instr = input.split(':').collect::<Vec<&str>>()[1].trim();
        let instr_list = instr.split(',').collect::<Vec<&str>>();
        for i in instr_list.chunks(2) {
            self.code.push((i[0].trim().parse::<u8>().unwrap(), i[1].trim().parse::<u64>().unwrap()));
        }
    }

    fn _get_combo(&self, operand: u64) -> u64 {
        match operand {
            0 | 1 | 2 | 3 => operand as u64,
            4 => self.rax,
            5 => self.rbx,
            6 => self.rcx,
            _ => panic!("Invalid operand")
        }
    }

    fn _adv(&mut self, operand: u64) {
        let numerator = self.rax;
        let denominator = 2_u64.pow(self._get_combo(operand) as u32);
        self.rax = numerator / denominator;
        self.rip += 1;
    }
    
    fn _bxl(&mut self, operand: u64) {
        self.rbx = self.rbx ^ operand as u64;
        self.rip += 1;
    }

    fn _bst(&mut self, operand: u64) {
        self.rbx = self._get_combo(operand).rem_euclid(8);
        self.rip += 1;
    }

    fn _jnz(&mut self, operand: u64) {
        if self.rax != 0 {
            self.rip = self._get_combo(operand) / 2;
        } else {
            self.rip += 1;
        }
    }

    fn _bxc(&mut self) {
        self.rbx = self.rbx ^ self.rcx;
        self.rip += 1;
    }

    fn _out(&mut self, operand: u64) {
        let val = self._get_combo(operand).rem_euclid(8);
        self.output.push(val as u8);
        self.rip += 1;
    }

    fn _bdv(&mut self, operand: u64) {
        let numerator = self.rax;
        let denominator = 2_u64.pow(self._get_combo(operand) as u32);
        self.rbx = numerator / denominator;
        self.rip += 1;
    }

    fn _cdv(&mut self, operand: u64) {
        let numerator = self.rax;
        let denominator = 2_u64.pow(self._get_combo(operand) as u32);
        self.rcx = numerator / denominator;
        self.rip += 1;
    }

    fn read_from_file(&mut self, filename: &str) {
        let mut file = File::open(filename).unwrap();
        let mut file_str = String::new();
        file.read_to_string(&mut file_str).unwrap();
        let lines = file_str.lines().collect::<Vec<&str>>();
        for i in 0..3 {
            self._parse_reg(i, lines[i]);
        }
        self._parse_code(lines[4]);
    }

    fn _print_reg(&self) {
        println!("RAX: {}\nRBX: {}\nRCX: {}", self.rax, self.rbx, self.rcx);
    }

    fn execute(&mut self, print: bool) {
        while self.rip < self.code.len() as u64 {
            let instr = self.code[self.rip as usize];

            match instr.0 {
                0 => self._adv(instr.1),
                1 => self._bxl(instr.1),
                2 => self._bst(instr.1),
                3 => self._jnz(instr.1),
                4 => self._bxc(),
                5 => self._out(instr.1),
                6 => self._bdv(instr.1),
                7 => self._cdv(instr.1),
                _ => panic!("Invalid opcode")
            }
        }

        if print && self.output.len() > 0 {
            println!("{}", self.output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));
        }
    }
}

fn calculateRAX(processor: &mut Processor, sequence: Vec<u8>, revcode: &Vec<(u8, u64)>, rax: &u64) -> u64 {
    if revcode.len() == 0 {
        return *rax;
    }

    let mut search = sequence.clone();

    for pair in revcode {
        search.insert(0, pair.1 as u8);
        search.insert(0, pair.0);
        let mut found_RAX = Vec::new();
        for j in rax* 0o100..rax * 0o100 + 0o77{
            processor.rax = j;
            processor.rip = 0;
            processor.output.clear();
            processor.execute(false);
            if processor.output == search {
                found_RAX.push(j);
            }
        }

        let mut mod_revcode = revcode.clone();
        mod_revcode.remove(0);

        for i in &found_RAX {
            let tmp = calculateRAX(processor, search.clone(), &mod_revcode, i);
            if tmp != 0 {
                return tmp;
            }
        }
    }

    0
}

fn main() {
    let mut processor = Processor {
        rax: 0,
        rbx: 0,
        rcx: 0,
        rip: 0,
        code: Vec::new(),
        output: Vec::new()
    };

    processor.read_from_file("input");
    print!("Part 1: ");
    processor.execute(true);

    let mut revcode: Vec<(u8, u64)> = processor.code.clone();
    revcode.reverse();

    let search = Vec::new();
    let found_RAX = calculateRAX(&mut processor, search, &revcode, &0);
    println!("Part 2: {}", found_RAX);

}