use std::borrow::Cow;
use std::io::{Cursor, Read, Write};
use std::{error, fmt, io, mem, u32};
use vlq::WriteVlqExt;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
}

fn encodeZigZag32(v: i32) -> u32 {
    unimplemented!()
}

fn encodeZigZag64(v: i64) -> u64 {
    unimplemented!()
}

pub trait WriteSigmaVlqExt {
    /// Length of encoded data
    // fn length(&self) -> usize;

    // TODO remove "type" suffixes?
    fn put_i8(&mut self, v: i8) -> Result<(), Error> {
        Self::put_u8(self, v as u8)
    }

    fn put_u8(&mut self, v: u8) -> Result<(), Error>;

    fn put_i16(&mut self, v: i16) -> Result<(), Error> {
        Self::put_u32(self, encodeZigZag32(v as i32))
    }

    fn put_u16(&mut self, v: u16) -> Result<(), Error> {
        Self::put_u64(self, v as u64)
    }

    fn put_i32(&mut self, v: i32) -> Result<(), Error> {
        Self::put_u64(self, encodeZigZag32(v as i32) as u64)
    }

    fn put_u32(&mut self, v: u32) -> Result<(), Error> {
        Self::put_u64(self, v as u64)
    }

    fn put_i64(&mut self, v: i64) -> Result<(), Error> {
        Self::put_u64(self, encodeZigZag64(v))
    }

    fn put_u64(&mut self, v: u64) -> Result<(), Error>;

    fn put_slice(&mut self, v: &[u8]) -> Result<(), Error>;

    fn put_bits(&mut self, v: &[bool]) -> Result<(), Error> {
        // TODO implement via put_slice
        unimplemented!()
    }

    // fn put_option<T>(&mut self, v: Option<T>) -> Result<(), Error>;

    fn put_bool(&mut self, v: bool) -> Result<(), Error> {
        Self::put_u8(self, if v { 1 } else { 0 })
    }
}

impl<W: Write> WriteSigmaVlqExt for W {
    fn put_u8(&mut self, v: u8) -> Result<(), Error> {
        self.write_all(&[v]).map_err(Error::Io)
    }

    fn put_u64(&mut self, v: u64) -> Result<(), Error> {
        // TODO compare with our VLQ
        self.write_vlq(v).map_err(Error::Io)
    }

    fn put_slice(&mut self, v: &[u8]) -> Result<(), Error> {
        self.write_all(v).map_err(Error::Io)
    }
}