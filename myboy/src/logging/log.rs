use crate::cpu::cpu;

pub enum Log {
    Msg(String),
    SerialOutput(char),
    CPUState(cpu::CPUState),
}

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub(crate) trait Logger {
    fn log(&mut self, level: LogLevel, log_type: Log);

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
pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&mut self, _level: LogLevel, log_type: Log) {
        match log_type {
            Log::Msg(msg) => {
                println!("{}", msg);
            }
            Log::SerialOutput(c) => {
                print!("{}", c);
            }
            Log::CPUState(state) => {
                println!("{:?}", state);
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
