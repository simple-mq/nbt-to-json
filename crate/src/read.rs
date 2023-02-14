#![allow(unused)]
use std::fmt;

#[derive(Debug, Clone)]
pub struct NbtProp {
    key: String,
    _len: usize,
    value: NbtValue,
}

impl fmt::Display for NbtProp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            NbtValue::EndOfTheCompoud => write!(f, "{}", self.value)?,
            NbtValue::Compoud(_) => {
                if self.key == "Root" {
                    write!(f, "{{ \"{}\": {{ {} }}", self.key, self.value)?
                } else {
                    write!(f, "\"{}\": {{ {}", self.key, self.value)?
                }
            }
            _ => write!(f, "\"{}\": {}", self.key, self.value)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum NbtValue {
    Byte(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String, u16),
    List(Vec<NbtValue>),
    Compoud(Vec<NbtProp>),
    EndOfTheCompoud,
    ByteList(Vec<u8>),
    IntList(Vec<i32>),
    LongList(Vec<i64>),
}

impl fmt::Display for NbtValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NbtValue::Byte(n) => write!(f, "{n}"),
            NbtValue::I16(n) => write!(f, "{n}"),
            NbtValue::I32(n) => write!(f, "{n}"),
            NbtValue::I64(n) => write!(f, "{n}"),
            NbtValue::F32(n) => write!(f, "{n}"),
            NbtValue::F64(n) => write!(f, "{n}"),
            NbtValue::String(s, _) => write!(f, "\"{s}\""),
            NbtValue::List(l) => {
                write!(f, "[")?;
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    if let NbtValue::Compoud(_) = *v {
                        write!(f, "{{")?;
                    }
                    write!(f, "{v}")?;
                }
                write!(f, "]")
            }
            NbtValue::ByteList(l) => {
                write!(f, "[")?;
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{v}")?;
                }
                write!(f, "]")
            }
            NbtValue::IntList(l) => {
                write!(f, "[")?;
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{v}")?;
                }
                write!(f, "]")
            }
            NbtValue::LongList(l) => {
                write!(f, "[")?;
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{v}")?;
                }
                write!(f, "]")
            }
            NbtValue::Compoud(l) => {
                for (i, v) in l.iter().enumerate() {
                    if i > 0
                        && if let NbtValue::EndOfTheCompoud = v.value {
                            false
                        } else {
                            true
                        }
                    {
                        write!(f, ", ")?;
                    }
                    write!(f, "{v}")?;
                }
                Ok(())
            }
            NbtValue::EndOfTheCompoud => write!(f, " }}"),
            _ => panic!("Display Err: 解析失败：未知的类型！"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NBTErr {
    #[error("IO错误：{0}")]
    FileErr(#[from] std::io::Error),

    #[error("无法读取类型{1:#x}，读取终止在第{0}字节")]
    TypeErr(usize, u8),

    #[error("无法读取值, 错误：{0}")]
    ValueErr(#[from] std::array::TryFromSliceError),

    #[error("生成JSON失败，错误：{0}")]
    JSONERR(#[from] json::Error),

    #[error("未知的错误！{0}")]
    Unknow(#[from] anyhow::Error),
}

pub fn convert(bytes: &Vec<u8>, pretty: bool) -> Result<String, NBTErr> {
    let buf = json::parse(&format!("{}", read_compoud(bytes, &mut 0)?))?;
    Ok(if pretty {
        buf.pretty(4)
    } else {
        buf.to_string()
    })
}

use clap::builder::Str;
use wasm_bindgen::{JsError, JsValue};
pub fn wasm_convert(bytes: &Vec<u8>) -> Result<String, JsError> {
    let buf = json::parse(&format!("{}", read_compoud(bytes, &mut 0)?))?;
    Ok(buf.pretty(4))
}

fn add(i: &mut usize, n: usize) -> usize {
    *i += n;
    *i
}

fn read_nbt(b: &Vec<u8>, i: &mut usize, t: u8, vb: &[u8]) -> Result<NbtValue, NBTErr> {
    let nbt = match t {
        1 => Some(NbtValue::Byte(i8::from_le_bytes(vb.try_into()?))),
        2 => Some(NbtValue::I16(i16::from_le_bytes(vb.try_into()?))),
        3 => Some(NbtValue::I32(i32::from_le_bytes(vb.try_into()?))),
        4 => Some(NbtValue::I64(i64::from_le_bytes(vb.try_into()?))),
        5 => Some(NbtValue::F32(f32::from_le_bytes(vb.try_into()?))),
        6 => Some(NbtValue::F64(f64::from_le_bytes(vb.try_into()?))),
        7 => {
            let c = u32::from_le_bytes(b[*i..add(i, 4)].try_into()?);
            let mut v: Vec<u8> = vec![];
            if c > 0 {
                for _ in 1..=c {
                    v.push(b[*i]);
                    *i += 1;
                }
            }
            Some(NbtValue::ByteList(v))
        }
        8 => {
            let vlen = u16::from_le_bytes(b[*i..add(i, 2)].try_into()?);
            Some(NbtValue::String(
                b[*i..add(i, vlen as usize)].escape_ascii().to_string(),
                vlen,
            ))
        }
        9 => Some(read_list(
            b,
            b[*i],
            u32::from_le_bytes(b[*i + 1..*i + 5].try_into()?),
            i,
        )?),
        10 => Some(read_compoud(b, i)?),
        11 => {
            let c = u32::from_le_bytes(b[*i..add(i, 4)].try_into()?);
            let mut v: Vec<i32> = vec![];
            if c > 0 {
                for _ in 1..=c {
                    v.push(i32::from_le_bytes(b[*i..add(i, 4)].try_into()?));
                }
            }
            Some(NbtValue::IntList(v))
        }
        12 => {
            let c = u32::from_le_bytes(b[*i..add(i, 4)].try_into()?);
            let mut v: Vec<i64> = vec![];
            if c > 0 {
                for _ in 1..=c {
                    v.push(i64::from_le_bytes(b[*i..add(i, 8)].try_into()?));
                }
            }
            Some(NbtValue::LongList(v))
        }
        _ => None,
    };
    if let Some(v) = nbt {
        Ok(v)
    } else {
        Err(NBTErr::TypeErr(*i, t))
    }
}

fn read_list(b: &Vec<u8>, t: u8, c: u32, i: &mut usize) -> Result<NbtValue, NBTErr> {
    *i += 5;
    if c == 0 {
        return Ok(NbtValue::List(vec![]));
    }
    let size = [1, 1, 2, 4, 8, 4, 8, 0, 0, 0, 0, 0, 0]
        .get(t as usize)
        .ok_or(NBTErr::TypeErr(*i, t))?;
    let mut list: Vec<NbtValue> = vec![];
    let mut buf = &b[*i..*i + size * c as usize];
    let mut vb: &[u8];

    for _ in 1..=c {
        (vb, buf) = buf.split_at(*size);
        list.push(read_nbt(b, i, t, vb)?)
    }
    *i += (c * *size as u32) as usize;
    Ok(NbtValue::List(list))
}

fn read_compoud(b: &Vec<u8>, i: &mut usize) -> Result<NbtValue, NBTErr> {
    let mut buf: Vec<NbtProp> = vec![];
    while *i < b.len() {
        let t = b[*i];
        let mut len = 0;
        let mut key = String::new();
        let mut value = NbtValue::EndOfTheCompoud;
        let size = [1, 1, 2, 4, 8, 4, 8, 0, 0, 0, 0, 0, 0]
            .get(t as usize)
            .ok_or(NBTErr::TypeErr(*i, t))?;
        if t == 0 {
            buf.push(NbtProp {
                key,
                _len: len,
                value,
            });
            *i += 1;
            break;
        } else {
            len = i16::from_le_bytes(b[*i + 1..add(i, 3)].try_into()?) as usize;
            if len > 0 {
                key = b[*i..add(i, len)].escape_ascii().to_string();
            }
            if *i == 3 {
                key = String::from("Root");
            }
            let vb = &b[*i..add(i, *size)];
            value = read_nbt(b, i, t, vb)?;
        }
        buf.push(NbtProp {
            key,
            _len: len,
            value,
        });
    }
    Ok(NbtValue::Compoud(buf))
}
