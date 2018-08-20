
extern crate cluLog;

use kmsg_shape::KmsgShape;
use std::fs::File;
use cluLog::log_write::mutex::MutexWrite;
use cluLog::log_lock::mutex::LogSafeMutexLock;
use std::path::Path;
use cluLog::DefLogPanic;
use cluLog::log::default_one::LogOneDefault;
use std::io;
use std::io::BufWriter;

pub mod kmsg_shape;



#[inline]
pub fn default_open_kmsg<'a>() -> io::Result< LogOneDefault<'a, KmsgShape, DefLogPanic, MutexWrite<'a, BufWriter<File>>, LogSafeMutexLock<'a, BufWriter<File>>> > {
    open_kmsg("/dev/kmsg")
}

#[inline]
pub fn open_kmsg<'a, P: AsRef<Path>>(path: P) -> io::Result< LogOneDefault<'a, KmsgShape, DefLogPanic, MutexWrite<'a, BufWriter<File>>, LogSafeMutexLock<'a, BufWriter<File>>> > {
    cluLog::log_addition::file::buf::create_file(path)
}


#[inline]
pub fn new_file<'a>(file: File) -> LogOneDefault<'a, KmsgShape, DefLogPanic, MutexWrite<'a, BufWriter<File>>, LogSafeMutexLock<'a, BufWriter<File>>> {
    cluLog::log_addition::file::buf::new_file(file)
}

#[inline]
pub fn create_file<'a, PA: AsRef<Path>>(path: PA) -> io::Result< LogOneDefault<'a, KmsgShape, DefLogPanic, MutexWrite<'a, BufWriter<File>>, LogSafeMutexLock<'a, BufWriter<File>>> > {
    cluLog::log_addition::file::buf::create_file(path)
}


