use anyhow::{Context as _, Result};
use log::debug;
use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Read, Seek, SeekFrom, Write},
    path::Path,
};

pub fn lint_files<I, P>(files: I, fix: bool) -> Result<bool>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let mut ret = true;
    for f in files {
        debug!("checking {}", f.as_ref().to_string_lossy());
        let passed = lint(&mut BufReader::new(File::open(&f)?))?;
        if !passed {
            println!(
                "{}: no newline at end of file",
                f.as_ref().to_string_lossy()
            );
            if fix {
                let mut file = OpenOptions::new().append(true).open(&f)?;
                file.write(b"\n").with_context(|| {
                    format!(
                        "failed to append newline to {}",
                        f.as_ref().to_string_lossy()
                    )
                })?;
            }
        }
        ret &= passed;
    }

    Ok(ret)
}

pub fn lint(reader: &mut (impl Read + Seek)) -> Result<bool> {
    if is_binary(reader)? {
        debug!("binary file skipped");
        return Ok(true);
    }

    let n = reader.seek(SeekFrom::End(0))?;
    if n == 0 {
        debug!("empty file skipped");
        return Ok(true);
    }

    reader.seek(SeekFrom::End(-1))?;
    let eof = reader.bytes().next().transpose()?;
    Ok(eof == Some(b'\n'))
}

/// https://git.kernel.org/pub/scm/git/git.git/tree/xdiff-interface.c?h=v2.37.1#n192
fn is_binary(file: &mut impl Read) -> Result<bool> {
    const FIRST_FEW_BYTES: usize = 8000;
    let mut head = vec![0; FIRST_FEW_BYTES];
    let n = file.read(&mut head)?;
    Ok(head[..n].contains(&0))
}

#[cfg(test)]
mod tests {
    use super::{is_binary, lint};
    use std::io::Cursor;

    #[test]
    fn empty() {
        assert!(lint(&mut Cursor::new("".as_bytes())).unwrap());
    }

    #[test]
    fn valid_eof() {
        assert!(lint(&mut Cursor::new("text\n".as_bytes())).unwrap());
    }

    #[test]
    fn invalid_eof() {
        assert!(!lint(&mut Cursor::new("text".as_bytes())).unwrap());
    }

    #[test]
    fn text_is_not_binary() {
        assert!(!is_binary(&mut "text".as_bytes()).unwrap());
    }

    #[test]
    fn null_is_binary() {
        assert!(is_binary(&mut [0, 1].as_slice()).unwrap());
    }
}
