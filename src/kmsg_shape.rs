
use cluLog::log_shape::LogShape;
use std::fmt::Arguments;
use std::io::Write;
use std::io;

macro_rules! crate_name {
	() => {
		env!("CARGO_PKG_NAME")
	};
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum KmsgShape {}

impl LogShape for KmsgShape {	
	
	#[inline(always)]
	fn warning<'a, W: Write>(mut write: W, display: Arguments<'a>) -> io::Result<()> {
		write.write_fmt(	format_args!("<4>{}: [WAR] {}\n", crate_name!(), display)	)
	}
	//[WAR] - warning value
	
	#[inline(always)]
	fn info<'a, W: Write>(mut write: W, display: Arguments<'a>) -> io::Result<()> {
		write.write_fmt(	format_args!("<5>{}: [INF] {}\n", crate_name!(), display)	)
	}
	//[INF] - info value
	
	#[inline(always)]
	fn error<'a, W: Write>(mut write: W, display: Arguments<'a>) -> io::Result<()> {
		write.write_fmt(	format_args!("<3>{}: [ERR] {}\n", crate_name!(), display)		)
	}
	//[ERR] - err value
	
	#[inline(always)]
	fn panic<'a, W: Write>(mut write: W, display: Arguments<'a>) -> io::Result<()> {
		write.write_fmt(	format_args!("<3>{}: [PANIC] {}\n", crate_name!(), display)	)
	}
	//[PANIC] - panic program
	
	#[inline(always)]
	fn unknown<'a, W: Write>(mut write: W, name: &'static str, display: Arguments<'a>) -> io::Result<()> {
		write.write_fmt(	format_args!("<6>{}: [{}] {}\n", crate_name!(), name, display)	)
	}
	//[UNK] - unknown 
	
	#[inline(always)]
	fn trace<'s, W: Write>(mut write: W, line: u32, pos: u32, file: &'static str, args: Arguments<'s>) -> io::Result<()> {
		write.write_fmt(	format_args!("<6>{}: [TRACE][{}:{}:{}] {}\n", crate_name!(), file, line, pos, args)	)
	}
	
	#[inline(always)]
	fn print<'a, W: Write>(mut write: W, display: Arguments<'a>) -> io::Result<()> {
		write.write_fmt(	format_args!("<5>{}: [OUT] {}", crate_name!(), display)		)
	}
	//[OUT] - unknown 
	
	#[inline(always)]
	fn eprint<'a, W: Write>(mut write: W, display: Arguments<'a>) -> io::Result<()> {
		write.write_fmt(	format_args!("<3>{}: [EOUT] {}", crate_name!(), display)		)
	}
	//[EOUT] - unknown 
}
