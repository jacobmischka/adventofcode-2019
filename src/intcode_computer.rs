#[derive(Debug)]
pub struct IntcodeComputer {
    mem: Vec<u32>,
    pos: usize,
}

impl IntcodeComputer {
    pub fn new() -> Self {
        Self {
            mem: Vec::new(),
            pos: 0 as usize,
        }
    }

    pub fn init(&mut self, raw: &str) -> Result<(), Error> {
        self.mem = raw
            .split(',')
            .map(|s| {
                s.parse::<u32>()
                    .map_err(|_| Error::ProgramParseError(s.to_string()))
            })
            .collect::<Result<Vec<u32>, Error>>()?;
        self.pos = 0;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let mut next_inst = self.get_inst();
        while let Ok(inst) = next_inst {
            match inst {
                Opcode::Add(lhs, rhs) => {
                    let pos = self.read_next();
                    self.write(pos, lhs + rhs);
                }
                Opcode::Multiply(lhs, rhs) => {
                    let pos = self.read_next();
                    self.write(pos, lhs * rhs);
                }
                Opcode::Exit => {
                    break;
                }
            }
            next_inst = self.get_inst();
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
    fn execute_and_dump(raw: &str) -> Result<String, Error> {
        let mut p = Self::new();
        p.init(raw)?;
        p.run()?;
        Ok(p.dump())
    }

    fn get_inst(&mut self) -> Result<Instruction, Error> {
		let val = self.read_next();
		let opcode = val % 100;

        let operation = match val {
            1 => {
				Ok(Opcode::Add)
                // let lhs_pos = self.read_next();
                // let rhs_pos = self.read_next();
                // Ok(Opcode::Add(self.read(lhs_pos), self.read(rhs_pos)))
            }
            2 => {
				Ok(Opcode::Multiply)
                // let lhs_pos = self.read_next();
                // let rhs_pos = self.read_next();
                // Ok(Opcode::Multiply(self.read(lhs_pos), self.read(rhs_pos)))
            }
            3 => {
				Ok(Opcode::Input)
                let address = self.read_next();
                Ok(Opcode::Input(address))
            }
            4 => {
                let address = self.read_next();
                Ok(Opcode::Output(self.read(address)))
            }
            99 => Ok(Opcode::Exit),
            x => Err(Error::OpcodeParseError(x)),
        }?;

		match operation {
		}
    }

    pub fn read(&self, pos: u32) -> u32 {
        self.mem[pos as usize]
    }

    pub fn read_next(&mut self) -> u32 {
        let val = self.mem[self.pos];
        self.inc();
        val
    }

    pub fn write(&mut self, pos: u32, val: u32) -> () {
        self.mem[pos as usize] = val;
    }

    pub fn inc(&mut self) -> () {
        self.pos += 1;
    }
}

type Int = u32;

#[derive(Debug)]
struct Instruction {
	operation: Operation,
	modes: Vec<ParameterMode>,
	params: Vec<Int>
}

#[derive(Debug)]
enum ParameterMode {
	Position,
	Immediate
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
    Input,
    Output,
    Exit,
}

#[test]
fn it_works() {
    assert_eq!(
        IntcodeComputer::execute_and_dump("1,0,0,0,99").unwrap(),
        "2,0,0,0,99"
    );
    assert_eq!(
        IntcodeComputer::execute_and_dump("2,3,0,3,99").unwrap(),
        "2,3,0,6,99"
    );
    assert_eq!(
        IntcodeComputer::execute_and_dump("2,4,4,5,99,0").unwrap(),
        "2,4,4,5,99,9801"
    );
    assert_eq!(
        IntcodeComputer::execute_and_dump("1,1,1,4,99,5,6,0,99").unwrap(),
        "30,1,1,4,2,5,6,0,99"
    );
}

#[derive(Debug)]
pub enum Error {
    ProgramParseError(String),
    OpcodeParseError(u32),
}
