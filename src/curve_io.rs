//! IO utilities
#![allow(dead_code)]

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use group::{prime::PrimeCurveAffine, GroupEncoding, UncompressedEncoding};
use std::io::{self, Read, Write};

pub trait GroupWriter: Write {
    fn write_group_uncompressed<Enc: UncompressedEncoding>(&mut self, e: &Enc) -> io::Result<()> {
        self.write_all(e.to_uncompressed().as_ref())
    }
    fn write_group<Enc: GroupEncoding>(&mut self, e: &Enc) -> io::Result<()> {
        self.write_all(e.to_bytes().as_ref())
    }
    fn write_groups<Enc: GroupEncoding>(&mut self, es: &[Enc]) -> io::Result<()> {
        self.write_u64::<BigEndian>(es.len() as u64)?;
        for e in es {
            self.write_group(e)?;
        }
        Ok(())
    }
    fn write_groups_uncompressed<Enc: UncompressedEncoding>(
        &mut self,
        es: &[Enc],
    ) -> io::Result<()> {
        self.write_u64::<BigEndian>(es.len() as u64)?;
        for e in es {
            self.write_group_uncompressed(e)?;
        }
        Ok(())
    }
}

pub trait GroupReader: Read {
    fn read_group_uncompressed<Enc: UncompressedEncoding + PrimeCurveAffine>(
        &mut self,
        checked: bool,
        allow_zero: bool,
    ) -> io::Result<Enc> {
        let mut repr = <Enc as UncompressedEncoding>::Uncompressed::default();
        self.read_exact(repr.as_mut())?;

        let affine = if checked {
            Enc::from_uncompressed(&repr)
        } else {
            Enc::from_uncompressed_unchecked(&repr)
        };

        let affine = if affine.is_some().into() {
            Ok(affine.unwrap())
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "invalid group"))
        }?;

        if allow_zero {
            Ok(affine)
        } else if affine.is_identity().into() {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "point at infinity",
            ))
        } else {
            Ok(affine)
        }
    }
    fn read_group<Enc: GroupEncoding + PrimeCurveAffine>(
        &mut self,
        checked: bool,
        allow_zero: bool,
    ) -> io::Result<Enc> {
        let mut repr = <Enc as GroupEncoding>::Repr::default();
        self.read_exact(repr.as_mut())?;

        let affine = if checked {
            Enc::from_bytes(&repr)
        } else {
            Enc::from_bytes_unchecked(&repr)
        };

        let affine = if affine.is_some().into() {
            Ok(affine.unwrap())
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "invalid group"))
        }?;

        if allow_zero {
            Ok(affine)
        } else if affine.is_identity().into() {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "point at infinity",
            ))
        } else {
            Ok(affine)
        }
    }
    fn read_groups_uncompressed<Enc: UncompressedEncoding + PrimeCurveAffine>(
        &mut self,
        checked: bool,
        allow_zero: bool,
    ) -> io::Result<Vec<Enc>> {
        let len = self.read_u64::<BigEndian>()? as usize;
        let mut groups = Vec::new();
        for _ in 0..len {
            groups.push(self.read_group_uncompressed(checked, allow_zero)?);
        }
        Ok(groups)
    }
    fn read_groups<Enc: PrimeCurveAffine>(
        &mut self,
        checked: bool,
        allow_zero: bool,
    ) -> io::Result<Vec<Enc>> {
        let len = self.read_u64::<BigEndian>()? as usize;
        let mut groups = Vec::new();
        for _ in 0..len {
            groups.push(self.read_group(checked, allow_zero)?);
        }
        Ok(groups)
    }
}

impl<R: Read> GroupReader for R {}
impl<W: Write> GroupWriter for W {}
