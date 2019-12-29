use std::collections::VecDeque;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum State {
    Halt,
    Error,
    Wait,
    Run,
    Output,
}

#[derive(Debug, Clone)]
pub struct Computer {
    pos: usize,
    rel_base: isize,
    program: Vec<isize>,

    state: State,
    inputs: VecDeque<isize>,
    outputs: VecDeque<isize>,
}

impl Computer {
    pub fn new(program: Vec<isize>) -> Computer {
        Computer {
            pos: 0,
            rel_base: 0,
            program: program,
            state: State::Run,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
        }
    }

    pub fn from(program: &str) -> Computer {
        Computer::new(
            program
                .trim()
                .split(",")
                .map(|x| x.parse::<isize>().unwrap())
                .collect(),
        )
    }

    pub fn write_input(&mut self, input: isize) {
        self.inputs.push_back(input);
    }

    pub fn write_string_input(&mut self, input: &str) {
        for char in input.chars() {
            self.inputs.push_back(char as isize);
        }
    }

    pub fn pop_output(&mut self) -> Option<isize> {
        self.outputs.pop_front()
    }

    pub fn get_output(&mut self) -> &VecDeque<isize> {
        &self.outputs
    }

    pub fn has_halted(&self) -> bool {
        self.state == State::Halt
    }

    pub fn is_waiting(&self) -> bool {
        self.state == State::Wait
    }

    pub fn has_output(&self) -> bool {
        self.state == State::Output
    }

    pub fn run(&mut self) {
        self.state = State::Run;
        loop {
            match self.state {
                State::Run => self.one_op(),
                State::Halt => break,
                State::Wait => break,
                State::Error => panic!("Error"),
                State::Output => break,
            }
        }
    }

    fn one_op(&mut self) {
        let op = self.read_mem(self.pos);
        let opcode = op % 100;
        let params_mode = parse_mode(op / 100);
        match opcode {
            1 => self.add(params_mode),
            2 => self.mul(params_mode),
            3 => self.input(params_mode),
            4 => self.output(params_mode),
            5 => self.jump_if_true(params_mode),
            6 => self.jump_if_false(params_mode),
            7 => self.lt(params_mode),
            8 => self.eq(params_mode),
            9 => self.adjust_rel_base(params_mode),
            99 => self.state = State::Halt,
            v => {
                println!("Error, unexpected opcode: {:?}", v);
                self.state = State::Error;
            }
        }
    }

    fn read_mem(&mut self, offset: usize) -> isize {
        self.program.get(offset).copied().unwrap_or(0)
    }

    pub fn store_mem(&mut self, offset: usize, value: isize) {
        if offset >= self.program.len() {
            self.program.resize(offset + 1, 0);
        }
        self.program[offset] = value;
    }

    fn pointer_offset(&mut self, mode: isize, offset: usize) -> usize {
        match mode {
            0 => self.read_mem(self.pos + offset) as usize,
            1 => self.pos + offset,
            2 => {
                let rel_offset = self.read_mem(self.pos + offset);
                (self.rel_base + rel_offset) as usize
            }
            _ => {
                println!("Error, unexpected offset mode: {}", mode);
                0
            }
        }
    }

    fn load(&mut self, mode: isize, offset: usize) -> isize {
        let load_pos = self.pointer_offset(mode, offset);
        self.read_mem(load_pos)
    }

    fn store(&mut self, mode: isize, offset: usize, value: isize) {
        let store_pos = self.pointer_offset(mode, offset);
        self.store_mem(store_pos, value);
    }

    fn adjust_rel_base(&mut self, params_mode: Vec<isize>) {
        let first_param = self.load(params_mode[0], 1);
        self.rel_base += first_param;
        self.pos += 2;
    }

    fn add(&mut self, params_mode: Vec<isize>) {
        let first_param = self.load(params_mode[0], 1);
        let second_param = self.load(params_mode[1], 2);
        self.store(params_mode[2], 3, first_param + second_param);
        self.pos += 4;
    }

    fn mul(&mut self, params_mode: Vec<isize>) {
        let first_param = self.load(params_mode[0], 1);
        let second_param = self.load(params_mode[1], 2);
        self.store(params_mode[2], 3, first_param * second_param);
        self.pos += 4;
    }

    fn input(&mut self, params_mode: Vec<isize>) {
        match self.read_input() {
            None => self.state = State::Wait,
            Some(value) => {
                self.store(params_mode[0], 1, value);
                self.pos += 2;
            }
        }
    }

    fn read_input(&mut self) -> Option<isize> {
        self.inputs.pop_front()
    }

    fn output(&mut self, params_mode: Vec<isize>) {
        let out = self.load(params_mode[0], 1);
        self.outputs.push_back(out);
        self.pos += 2;
        self.state = State::Output;
    }

    fn jump_if_true(&mut self, params_mode: Vec<isize>) {
        let first_param = self.load(params_mode[0], 1);
        if first_param != 0 {
            let second_param = self.load(params_mode[1], 2);
            self.pos = second_param as usize;
        } else {
            self.pos += 3;
        }
    }

    fn jump_if_false(&mut self, params_mode: Vec<isize>) {
        let first_param = self.load(params_mode[0], 1);
        if first_param == 0 {
            let second_param = self.load(params_mode[1], 2);
            self.pos = second_param as usize;
        } else {
            self.pos += 3;
        }
    }

    fn lt(&mut self, params_mode: Vec<isize>) {
        let first_param = self.load(params_mode[0], 1);
        let second_param = self.load(params_mode[1], 2);
        if first_param < second_param {
            self.store(params_mode[2], 3, 1);
        } else {
            self.store(params_mode[2], 3, 0);
        }
        self.pos += 4
    }

    fn eq(&mut self, params_mode: Vec<isize>) {
        let first_param = self.load(params_mode[0], 1);
        let second_param = self.load(params_mode[1], 2);
        if first_param == second_param {
            self.store(params_mode[2], 3, 1);
        } else {
            self.store(params_mode[2], 3, 0);
        }
        self.pos += 4
    }
}

fn parse_mode(mode: isize) -> Vec<isize> {
    let mut v = Vec::new();
    v.push(mode % 10);
    v.push(mode / 10 % 10);
    v.push(mode / 100 % 10);
    v
}
