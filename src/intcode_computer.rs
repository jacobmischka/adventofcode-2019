use std::collections::VecDeque;

pub type Int = i32;

#[derive(Debug)]
pub struct IntcodeComputer {
    mem: Vec<Int>,
    pos: usize,
    input_buf: VecDeque<Int>,
    pub output_buf: VecDeque<Int>,
}

impl IntcodeComputer {
    pub fn new() -> Self {
        Self {
            mem: Vec::new(),
            pos: 0 as usize,
            input_buf: VecDeque::new(),
            output_buf: VecDeque::new(),
        }
    }

    pub fn init(&mut self, raw: &str) -> Result<(), Error> {
        self.mem = raw
            .split(',')
            .map(|s| {
                s.parse::<Int>()
                    .map_err(|_| Error::ProgramParseError(s.to_string()))
            })
            .collect::<Result<Vec<Int>, Error>>()?;
        self.pos = 0;
        self.input_buf.clear();
        self.output_buf.clear();

        Ok(())
    }

    pub fn add_input(&mut self, val: Int) {
        self.input_buf.push_back(val);
    }

    async fn get_input(&mut self) -> Int {
        loop {
            if let Some(input) = self.input_buf.pop_front() {
                return input;
            }
        }
    }

    pub async fn get_output(&mut self) -> Option<Int> {
        self.output_buf.pop_front()
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        use Instruction::*;

        loop {
            let inst = self.get_inst().unwrap();
            match inst {
                Add(lhs, rhs, dest) => {
                    self.write(self.get(&dest), self.get(&lhs) + self.get(&rhs));
                }
                Multiply(lhs, rhs, dest) => {
                    self.write(self.get(&dest), self.get(&lhs) * self.get(&rhs));
                }
                Input(dest) => {
                    let input = self.get_input().await;
                    self.write(self.get(&dest), input);
                }
                Output(src) => {
                    self.output_buf.push_back(self.get(&src));
                }
                JumpIfTrue(x, dest) => {
                    if self.get(&x) != 0 {
                        self.pos = self.get(&dest) as _;
                    }
                }
                JumpIfFalse(x, dest) => {
                    if self.get(&x) == 0 {
                        self.pos = self.get(&dest) as _;
                    }
                }
                LessThan(lhs, rhs, dest) => {
                    let val = if self.get(&lhs) < self.get(&rhs) { 1 } else { 0 };
                    self.write(self.get(&dest), val);
                }
                Equals(lhs, rhs, dest) => {
                    let val = if self.get(&lhs) == self.get(&rhs) { 1 } else { 0 };
                    self.write(self.get(&dest), val);
                }
                Exit => {
                    break;
                }
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    fn dump(&self) -> String {
        self.mem
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    #[allow(dead_code)]
    async fn execute_and_dump(raw: &str) -> Result<String, Error> {
        let mut p = Self::new();
        p.init(raw)?;
        p.run().await?;
        Ok(p.dump())
    }

    fn get_inst(&mut self) -> Result<Instruction, Error> {
        use Instruction::*;

        let val = self.read_next();
        let opcode = val % 100;

        match opcode {
            1 => Ok(Add(
                Parameter(self.read_next(), ParameterMode::new(val, 0)?),
                Parameter(self.read_next(), ParameterMode::new(val, 1)?),
                Parameter(self.read_next(), ParameterMode::Immediate),
            )),
            2 => Ok(Multiply(
                Parameter(self.read_next(), ParameterMode::new(val, 0)?),
                Parameter(self.read_next(), ParameterMode::new(val, 1)?),
                Parameter(self.read_next(), ParameterMode::Immediate),
            )),
            3 => Ok(Input(Parameter(
                self.read_next(),
                ParameterMode::Immediate,
            ))),
            4 => Ok(Output(Parameter(
                self.read_next(),
                ParameterMode::new(val, 0)?,
            ))),
            5 => Ok(JumpIfTrue(
                Parameter(self.read_next(), ParameterMode::new(val, 0)?),
                Parameter(self.read_next(), ParameterMode::new(val, 1)?),
            )),
            6 => Ok(JumpIfFalse(
                Parameter(self.read_next(), ParameterMode::new(val, 0)?),
                Parameter(self.read_next(), ParameterMode::new(val, 1)?),
            )),
            7 => Ok(LessThan(
                Parameter(self.read_next(), ParameterMode::new(val, 0)?),
                Parameter(self.read_next(), ParameterMode::new(val, 1)?),
                Parameter(self.read_next(), ParameterMode::Immediate),
            )),
            8 => Ok(Equals(
                Parameter(self.read_next(), ParameterMode::new(val, 0)?),
                Parameter(self.read_next(), ParameterMode::new(val, 1)?),
                Parameter(self.read_next(), ParameterMode::Immediate),
            )),
            99 => Ok(Exit),
            _ => Err(Error::OpcodeParseError(val)),
        }
    }

    fn get(&self, param: &Parameter) -> Int {
        use ParameterMode::*;

        match param.1 {
            Position => self.read(param.0),
            Immediate => param.0,
        }
    }

    pub fn read(&self, pos: Int) -> Int {
        self.mem[pos as usize]
    }

    pub fn read_next(&mut self) -> Int {
        let val = self.mem[self.pos];
        self.inc();
        val
    }

    pub fn write(&mut self, pos: Int, val: Int) -> () {
        self.mem[pos as usize] = val;
    }

    pub fn inc(&mut self) -> () {
        self.pos += 1;
    }
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    fn from_int(val: Int) -> Result<ParameterMode, Error> {
        use ParameterMode::*;

        match val {
            1 => Ok(Immediate),
            0 | _ => Ok(Position),
        }
    }

    fn parse_mode(x: Int, pos: u32) -> Int {
        ((x / 100 / (10i32.pow(pos))) % 10)
    }

    fn new(x: Int, pos: u32) -> Result<ParameterMode, Error> {
        Self::from_int(Self::parse_mode(x, pos))
    }
}

#[derive(Debug)]
struct Parameter(Int, ParameterMode);

#[derive(Debug)]
enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    Exit,
}

#[test]
fn day_2_examples_work() {
    use async_std::task;

    assert_eq!(
        task::block_on(IntcodeComputer::execute_and_dump("1,0,0,0,99")).unwrap(),
        "2,0,0,0,99"
    );
    assert_eq!(
        task::block_on(IntcodeComputer::execute_and_dump("2,3,0,3,99")).unwrap(),
        "2,3,0,6,99"
    );
    assert_eq!(
        task::block_on(IntcodeComputer::execute_and_dump("2,4,4,5,99,0")).unwrap(),
        "2,4,4,5,99,9801"
    );
    assert_eq!(
        task::block_on(IntcodeComputer::execute_and_dump("1,1,1,4,99,5,6,0,99")).unwrap(),
        "30,1,1,4,2,5,6,0,99"
    );
}

#[test]
fn day_5_examples_work() {
    use async_std::task;
    let mut computer = IntcodeComputer::new();
    computer.init("1002,4,3,4,33").unwrap();
    task::block_on(computer.run()).unwrap();

    assert_eq!(computer.read(4), 99);

    // TODO: Add rest of day 5 tests
}

#[derive(Debug)]
pub enum Error {
    ProgramParseError(String),
    OpcodeParseError(Int),
}
