use async_std::sync::{channel, Receiver, Sender};

pub const BUFFER_SIZE: usize = 50;

pub type Int = i64;

#[derive(Debug)]
pub struct IntcodeComputer<'a> {
    state: OperationState,
    mem: Vec<Int>,
    pos: usize,
    relative_base: isize,
    input: &'a Receiver<Int>,
    output: &'a Sender<Int>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OperationState {
    Preinit,
    Ready,
    Running,
    Exited,
}

pub type IOChannels = (Sender<Int>, Receiver<Int>);

impl<'a> IntcodeComputer<'a> {
    pub fn new(input: &'a Receiver<Int>, output: &'a Sender<Int>) -> Self {
        Self {
            state: OperationState::Preinit,
            mem: Vec::new(),
            pos: 0,
            relative_base: 0,
            input,
            output,
        }
    }

    pub fn create_io() -> (IOChannels, IOChannels) {
        (channel(BUFFER_SIZE), channel(BUFFER_SIZE))
    }

    pub fn state(&self) -> &OperationState {
        &self.state
    }

    pub fn init(&mut self, program: &str) -> Result<(), Error> {
        self.state = OperationState::Ready;
        self.mem = program
            .split(',')
            .map(|s| {
                s.parse::<Int>()
                    .map_err(|_| Error::ProgramParseError(s.to_string()))
            })
            .collect::<Result<Vec<Int>, Error>>()?;
        self.pos = 0;
        Ok(())
    }

    async fn get_input(&self) -> Option<Int> {
        self.input.recv().await
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        use Instruction::*;

        loop {
            let inst = self.get_inst().unwrap();
            if cfg!(feature = "debug") {
                dbg!(&inst);
            }
            match inst {
                Add(lhs, rhs, dest) => {
                    let dest = self.get(&dest);
                    let lhs = self.get(&lhs);
                    let rhs = self.get(&rhs);
                    self.write(dest as usize, lhs + rhs);
                }
                Multiply(lhs, rhs, dest) => {
                    let dest = self.get(&dest);
                    let lhs = self.get(&lhs);
                    let rhs = self.get(&rhs);
                    self.write(dest as usize, lhs * rhs);
                }
                Input(dest) => {
                    let input = self.get_input().await.unwrap();
                    let dest = self.get(&dest);
                    self.write(dest as usize, input);
                }
                Output(src) => {
                    self.output.send(self.get(&src)).await;
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
                    let val = if self.get(&lhs) < self.get(&rhs) {
                        1
                    } else {
                        0
                    };
                    let dest = self.get(&dest);
                    self.write(dest as usize, val);
                }
                Equals(lhs, rhs, dest) => {
                    let val = if self.get(&lhs) == self.get(&rhs) {
                        1
                    } else {
                        0
                    };
                    let dest = self.get(&dest);
                    self.write(dest as usize, val);
                }
                RelativeBase(adj) => {
                    self.relative_base += self.get(&adj) as isize;
                }
                Exit => {
                    self.exit();
                    break;
                }
            }
        }

        Ok(())
    }

    fn exit(&mut self) {
        self.state = OperationState::Exited;
    }

    #[allow(dead_code)]
    fn dump(&self) -> String {
        self.mem
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",")
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
            3 => Ok(Input(Parameter(self.read_next(), ParameterMode::Immediate))),
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
            9 => Ok(RelativeBase(Parameter(
                self.read_next(),
                ParameterMode::new(val, 0)?,
            ))),
            99 => Ok(Exit),
            _ => Err(Error::OpcodeParseError(val)),
        }
    }

    fn get(&mut self, param: &Parameter) -> Int {
        use ParameterMode::*;

        match param.1 {
            Position => self.read(param.0 as usize),
            Immediate => param.0,
            Relative => self.read((self.relative_base + param.0 as isize) as usize),
        }
    }

    pub fn read(&mut self, pos: usize) -> Int {
        if pos >= self.mem.len() {
            self.mem.resize(pos.max(self.mem.len() * 2), 0);
        }

        self.mem[pos]
    }

    pub fn read_next(&mut self) -> Int {
        let val = self.read(self.pos);
        self.inc();
        val
    }

    pub fn write(&mut self, pos: usize, val: Int) -> () {
        if pos >= self.mem.len() {
            self.mem.resize(pos.max(self.mem.len() * 2), 0);
        }

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
    Relative,
}

impl ParameterMode {
    fn from_int(val: Int) -> Result<ParameterMode, Error> {
        use ParameterMode::*;

        match val {
            2 => Ok(Relative),
            1 => Ok(Immediate),
            0 | _ => Ok(Position),
        }
    }

    fn parse_mode(x: Int, pos: u32) -> Int {
        ((x / 100 / ((10 as Int).pow(pos))) % 10)
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
    RelativeBase(Parameter),
    Exit,
}

#[test]
fn day_2_examples_work() {
    use async_std::task;

    assert_eq!(
        task::block_on(execute_and_dump("1,0,0,0,99")).unwrap(),
        "2,0,0,0,99"
    );
    assert_eq!(
        task::block_on(execute_and_dump("2,3,0,3,99")).unwrap(),
        "2,3,0,6,99"
    );
    assert_eq!(
        task::block_on(execute_and_dump("2,4,4,5,99,0")).unwrap(),
        "2,4,4,5,99,9801"
    );
    assert_eq!(
        task::block_on(execute_and_dump("1,1,1,4,99,5,6,0,99")).unwrap(),
        "30,1,1,4,2,5,6,0,99"
    );
}

#[allow(dead_code)]
async fn execute_and_dump(raw: &str) -> Result<String, Error> {
    let (inputs, outputs) = IntcodeComputer::create_io();
    let mut p = IntcodeComputer::new(&inputs.1, &outputs.0);
    p.init(raw)?;
    p.run().await?;
    Ok(p.dump())
}

#[test]
fn day_5_examples_work() {
    use async_std::task;
    let (inputs, outputs) = IntcodeComputer::create_io();
    let mut computer = IntcodeComputer::new(&inputs.1, &outputs.0);
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
