use std::ops::{Add, Mul};

pub struct Computer {
    opcodes: Vec<i64>,
    i: usize,
}

pub enum Step {
    Nothing,
    Output(i64),
    Terminated,
}

use Step::*;

impl Computer {
    pub fn parse(input: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
        input
            .trim_end()
            .split(",")
            .map(|opcode| opcode.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()
    }

    pub fn new(opcodes: &Vec<i64>) -> Computer {
        Computer {
            opcodes: opcodes.clone(),
            i: 0,
        }
    }

    pub fn new_overrides(opcodes: &Vec<i64>, overrides: Vec<(usize, i64)>) -> Computer {
        let mut computer = Computer::new(opcodes);

        for (i, val) in overrides.iter() {
            computer.opcodes[*i] = *val;
        }

        computer
    }

    /// Runs the computer to completion, returns the value at opcode 0.
    pub fn run<Input>(&mut self, input: &mut Input) -> Result<i64, failure::Error>
    where
        Input: Iterator<Item = Result<i64, failure::Error>>,
    {
        self.step(input).and_then(|step_result| match step_result {
            Nothing => self.run(input),
            Output(output) => {
                println!("{}", output);
                self.run(input)
            }
            Terminated => Ok(self.opcodes[0]),
        })
    }

    /// Runs the computer a single step.
    pub fn step<Input>(&mut self, input: &mut Input) -> Result<Step, failure::Error>
    where
        Input: Iterator<Item = Result<i64, failure::Error>>,
    {
        let opcode = self.opcodes[self.i];

        match opcode % 100 {
            1 => self.operator_step(i64::add),
            2 => self.operator_step(i64::mul),
            3 => self.input_step(input),
            4 => self.output_step(),
            5 => self.jump_step(i64::ne),
            6 => self.jump_step(i64::eq),
            7 => self.store_step(i64::lt),
            8 => self.store_step(i64::eq),
            99 => Ok(Terminated),
            _ => Err(failure::err_msg(format!(
                "invalid opcode {}",
                self.opcodes[self.i]
            ))),
        }
    }

    fn input_step<Input>(&mut self, input: &mut Input) -> Result<Step, failure::Error>
    where
        Input: Iterator<Item = Result<i64, failure::Error>>,
    {
        let dest = self.opcodes[self.i + 1];
        self.opcodes[dest as usize] = input
            .next()
            .ok_or_else(|| failure::err_msg(format!("no input at {}", self.i)))??;
        self.i = self.i + 2;
        Ok(Nothing)
    }

    fn output_step(&mut self) -> Result<Step, failure::Error> {
        let res = self.arg(1);
        self.i = self.i + 2;
        Ok(Output(res))
    }

    fn operator_step<F>(&mut self, operator: F) -> Result<Step, failure::Error>
    where
        F: Fn(i64, i64) -> i64,
    {
        let dest = self.opcodes[self.i + 3] as usize;
        self.opcodes[dest] = operator(self.arg(1), self.arg(2));
        self.i = self.i + 4;
        Ok(Nothing)
    }

    fn jump_step<F>(&mut self, operator: F) -> Result<Step, failure::Error>
    where
        F: Fn(&i64, &i64) -> bool,
    {
        self.i = if operator(&self.arg(1), &0) {
            self.arg(2) as usize
        } else {
            self.i + 3
        };

        Ok(Nothing)
    }

    fn store_step<F>(&mut self, operator: F) -> Result<Step, failure::Error>
    where
        F: Fn(&i64, &i64) -> bool,
    {
        let dest = self.opcodes[self.i + 3] as usize;
        self.opcodes[dest] = if operator(&self.arg(1), &self.arg(2)) {
            1
        } else {
            0
        };

        self.i = self.i + 4;
        Ok(Nothing)
    }

    fn arg(&mut self, argi: usize) -> i64 {
        let val = self.opcodes[self.i + argi];
        if self.opcodes[self.i] / (i64::pow(10, argi as u32 + 1)) % 10 == 1 {
            val
        } else {
            self.opcodes[val as usize]
        }
    }
}
