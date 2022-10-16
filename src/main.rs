use std::io::Read;
use std::{env, fmt, fs, mem, process};

const MAGIC: [u8; 16] = [
    0x53, 0x42, 0x4F, 0x4D, 0xD6, 0xBA, 0x2E, 0xAC, //
    0xA3, 0xE6, 0x7A, 0x52, 0xAA, 0xEE, 0x3B, 0xAF, //
];

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct uSWID<'a> {
    magic: [u8; 16],
    header_ver: u8,
    header_size: u16,
    payload_size: u16,
    payload: &'a [u8],
}

struct uSWIDData {
    data: Vec<u8>,
}

use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
impl Decodable for uSWIDData {
    fn decode<D: Decoder>(d: &mut D) -> Result<uSWIDData, D::Error> {
        // Read the tag number and throw it away. YOU MUST DO THIS!
        d.read_u8();
        // The *next* data item is the actual data.
        Ok(uSWIDData {
            data: Decodable::decode(d).unwrap_or_default(),
        })
    }
}

impl<'a> fmt::Display for uSWID<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = cbor::Decoder::from_reader(self.payload);
        let items: Vec<(u8, Vec<u8>)> = d.decode().collect::<Result<_, _>>().unwrap();

        write!(
            f,
            "Magic: {} ({:x?})\n  Version: {}\n  Header size: {}\n  Payload size: {}\nPayload start/end: {:02x?}{:02x?}\n  {:?}",
            std::str::from_utf8(&self.magic[..4]).expect(""),
            self.magic,
            self.header_ver,
            self.header_size,
            self.payload_size,
            &self.payload[..4],
            &self.payload[self.payload_size as usize-4..self.payload_size as usize],
            items
        )
    }
}

impl<'a> uSWID<'a> {
    pub fn new(data: &[u8]) -> Result<(uSWID, usize), String> {
        let mut i = 16;

        while i + mem::size_of::<uSWID>() <= data.len() {
            if data[i..i + 16] == MAGIC {
                let header_ver = data[i + 16];
                let header_size = u16::from_le_bytes([data[i + 17], data[i + 18]]);
                let payload_size = u16::from_le_bytes([data[i + 19], data[i + 20]]);
                let hsz = header_size as usize;
                let psz = payload_size as usize;
                let payload = &data[i + hsz..i + hsz + psz];
                return Ok((
                    uSWID {
                        magic: data[i..i + 16].try_into().expect(""),
                        header_ver,
                        header_size,
                        payload_size,
                        payload,
                    },
                    i,
                ));
            }
            // NOTE: This assumes 16 byte alignment for now.
            i += 16;
        }
        Err(format!("nope"))
    }
}

fn analyze(data: &Vec<u8>) -> Result<(), String> {
    match uSWID::new(&data) {
        Ok((uswid, offset)) => {
            println!("SBOM found at {:x}\n{}", offset, uswid);
        }
        _ => {}
    }
    Ok(())
}

fn romulan(path: &str) -> Result<(), String> {
    println!("{}", path);

    let mut data = Vec::new();
    fs::File::open(path)
        .map_err(|err| format!("failed to open {}: {}", path, err))?
        .read_to_end(&mut data)
        .map_err(|err| format!("failed to read {}: {}", path, err))?;

    let _ = analyze(&data);
    Ok(())
}

fn main() {
    for arg in env::args().skip(1) {
        if let Err(err) = romulan(&arg) {
            eprintln!("Error: {}: {}", arg, err);
            process::exit(1);
        }
    }
}
