use async_std::sync::{channel, Receiver, Sender};

const BUFFER_SIZE: usize = 50;

pub type Int = i32;

#[derive(Debug)]
pub struct IntcodeComputer {
    state: OperationState,
    mem: Vec<Int>,
    pos: usize,
    input_sender: Option<Sender<Int>>,
    input_receiver: Option<Receiver<Int>>,
    output_sender: Option<Sender<Int>>,
    output_receiver: Option<Receiver<Int>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OperationState {
    Preinit,
    Ready,
    Running,
    Exited,
}

impl IntcodeComputer {
    pub fn new() -> Self {
        Self {
            state: OperationState::Preinit,
            mem: Vec::new(),
            pos: 0 as usize,
            input_sender: None,
            input_receiver: None,
            output_sender: None,
            output_receiver: None,
        }
    }

    pub fn state(&self) -> &OperationState {
        &self.state
    }

    pub fn init(&mut self, raw: &str) -> Result<(), Error> {
        self.state = OperationState::Ready;
        self.mem = raw
            .split(',')
            .map(|s| {
                s.parse::<Int>()
                    .map_err(|_| Error::ProgramParseError(s.to_string()))
            })
            .collect::<Result<Vec<Int>, Error>>()?;
        self.pos = 0;
        let (input_sender, input_receiver) = channel(BUFFER_SIZE);
        let (output_sender, output_receiver) = channel(BUFFER_SIZE);
        self.input_sender = Some(input_sender);
        self.input_receiver = Some(input_receiver);
        self.output_sender = Some(output_sender);
        self.output_receiver = Some(output_receiver);

        Ok(())
    }

    pub async fn add_input(&self, val: Int) {
        if let Some(sender) = &self.input_sender {
            sender.send(val).await;
        }
    }

    async fn get_input(&self) -> Option<Int> {
        if let Some(receiver) = &self.input_receiver {
            receiver.recv().await
        } else {
            None
        }
    }

    pub async fn get_output(&self) -> Option<Int> {
        if let Some(receiver) = &self.output_receiver {
            receiver.recv().await
        } else {
            None
        }
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
                    self.write(self.get(&dest), self.get(&lhs) + self.get(&rhs));
                }
                Multiply(lhs, rhs, dest) => {
                    self.write(self.get(&dest), self.get(&lhs) * self.get(&rhs));
                }
                Input(dest) => {
                    let input = self.get_input().await.unwrap();
                    self.write(self.get(&dest), input);
                }
                Output(src) => {
                    self.output_sender
                        .as_ref()
                        .unwrap()
                        .send(self.get(&src))
                        .await;
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
                    self.write(self.get(&dest), val);
                }
                Equals(lhs, rhs, dest) => {
                    let val = if self.get(&lhs) == self.get(&rhs) {
                        1
                    } else {
                        0
                    };
                    self.write(self.get(&dest), val);
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
        self.input_sender = None;
        self.output_sender = None;
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
