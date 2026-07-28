use std::env;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use std::str::{FromStr, SplitWhitespace};

const COEFFICIENTS_PATH: &str = "data/fb2001.dat";

fn parse_field<T>(fields: &mut SplitWhitespace<'_>, name: &str, line_number: usize) -> io::Result<T>
where
    T: FromStr,
    T::Err: Display,
{
    let value = fields.next().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("FB2001 line {line_number} must have a {name}"),
        )
    })?;

    value.parse().map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid FB2001 {name} on line {line_number}: {error}"),
        )
    })
}

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed={COEFFICIENTS_PATH}");

    let coefficients = fs::read_to_string(COEFFICIENTS_PATH)?;
    let output_path =
        PathBuf::from(env::var_os("OUT_DIR").ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Cargo did not provide OUT_DIR")
        })?)
        .join("fb2001.rs");
    let mut output = BufWriter::new(File::create(output_path)?);

    writeln!(output, "const TERMS: &[Term] = &[")?;
    for (line_index, line) in coefficients.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let line_number = line_index + 1;
        let mut fields = line.split_whitespace();
        let _index: usize = parse_field(&mut fields, "index", line_number)?;
        let power: usize = parse_field(&mut fields, "power", line_number)?;
        let amplitude: f64 = parse_field(&mut fields, "amplitude", line_number)?;
        let frequency: f64 = parse_field(&mut fields, "frequency", line_number)?;
        let phase: f64 = parse_field(&mut fields, "phase", line_number)?;

        if power > 5 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("FB2001 power on line {line_number} must not exceed 5"),
            ));
        }

        writeln!(
            output,
            "    Term {{ power: {power}, amplitude: {amplitude:?}, frequency: {frequency:?}, phase: {phase:?} }},"
        )?;
    }
    writeln!(output, "];")?;

    Ok(())
}
