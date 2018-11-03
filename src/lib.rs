
extern crate cluLog;

use std::fs::File;
use cluLog::log_write::mutex::MutexWrite;
use cluLog::log_lock::mutex::LogSafeMutexLock;
use std::path::Path;
use cluLog::DefLogPanic;
use cluLog::log::default_one::LogOneDefault;
use std::io;
use std::io::BufWriter;

mod kmsg_shape;
pub use self::kmsg_shape::*;


#[inline(always)]
pub fn default_open_kmsg<'a>() -> io::Result< LogOneDefault<'a, KmsgShape, DefLogPanic, MutexWrite<'a, BufWriter<File>>, LogSafeMutexLock<'a, BufWriter<File>>> > {
    open_kmsg("/dev/kmsg")
}

#[inline(always)]
pub fn open_kmsg<'a, P: AsRef<Path>>(path: P) -> io::Result< LogOneDefault<'a, KmsgShape, DefLogPanic, MutexWrite<'a, BufWriter<File>>, LogSafeMutexLock<'a, BufWriter<File>>> > {
    self::cluLog::log_addition::file::buf::create_file(path)
}


#[inline(always)]
pub fn new_file<'a>(file: File) -> LogOneDefault<'a, KmsgShape, DefLogPanic, MutexWrite<'a, BufWriter<File>>, LogSafeMutexLock<'a, BufWriter<File>>> {
    self::cluLog::log_addition::file::buf::new_file(file)
}

#[inline(always)]
pub fn create_file<'a, PA: AsRef<Path>>(path: PA) -> io::Result< LogOneDefault<'a, KmsgShape, DefLogPanic, MutexWrite<'a, BufWriter<File>>, LogSafeMutexLock<'a, BufWriter<File>>> > {
    self::cluLog::log_addition::file::buf::create_file(path)
}


