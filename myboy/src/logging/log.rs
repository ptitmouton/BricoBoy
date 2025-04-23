use clap::ValueEnum;

use crate::cpu::CPUState;

#[derive(Debug)]
pub enum Log {
    Msg(String),
    SerialOutput(char),
    CPUState(CPUState),
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum LogOutput {
    Message,
    SerialData,
    CPUState,
}

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub(crate) trait Logger {
    fn log(&mut self, level: LogLevel, log_type: Log);

    fn set_disabled_outputs(&mut self, _outputs: Vec<LogOutput>) {
        // Default implementation does nothing
    }

    fn info(&mut self, log: Log) {
        self.log(LogLevel::Info, log)
    }

    fn warn(&mut self, log: Log) {
        self.log(LogLevel::Warning, log)
    }

    fn error(&mut self, log: Log) {
        self.log(LogLevel::Error, log)
    }
}

#[derive(Default)]
pub struct ConsoleLogger {
    disabled_outputs: Vec<LogOutput>,
}

impl Logger for ConsoleLogger {
    fn set_disabled_outputs(&mut self, outputs: Vec<LogOutput>) {
        self.disabled_outputs = outputs;
    }
    fn log(&mut self, _level: LogLevel, log_type: Log) {
        match log_type {
            Log::Msg(msg) => {
                if !self.disabled_outputs.contains(&LogOutput::Message) {
                    println!("{}", msg);
                }
            }
            Log::SerialOutput(c) => {
                if !self.disabled_outputs.contains(&LogOutput::SerialData) {
                    print!("{}", c);
                }
            }
            Log::CPUState(state) => {
                if !self.disabled_outputs.contains(&LogOutput::CPUState) {
                    println!("{:?}", state);
                }
            }
        }
    }
}

#[derive(Default)]
pub struct InMemoryLogger(Vec<(LogLevel, Log)>);

impl Logger for InMemoryLogger {
    fn log(&mut self, level: LogLevel, log_type: Log) {
        self.0.push((level, log_type));
    }
}
