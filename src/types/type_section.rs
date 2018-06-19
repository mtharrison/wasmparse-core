use leb128::ReadLeb128Ext;
use std::io::{Error, Read};

use super::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct TypeSection {
    pub count: u32,
    pub entries: Vec<FunctionType>,
}

impl TypeSection {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<TypeSection, Error> {
        let (count, _) = reader.leb128_unsigned()?;

        let mut entries = Vec::new();

        for _ in 0..count {
            let (form_num, _) = reader.leb128_signed()?;
            let form = ValueType::from_i64(form_num)?;

            let (param_count, _) = reader.leb128_unsigned()?;
            let mut param_types = Vec::new();

            for _ in 0..param_count {
                let (type_num, _) = reader.leb128_signed()?;
                let typ = ValueType::from_i64(type_num)?;
                param_types.push(typ);
            }

            let (return_count, _) = reader.leb128_unsigned()?;
            let return_type = match return_count {
                1 => {
                    let (type_num, _) = reader.leb128_signed()?;
                    let typ = ValueType::from_i64(type_num)?;
                    Some(typ)
                }
                _ => None,
            };

            let entry = FunctionType {
                form,
                param_count: param_count as u32,
                param_types,
                return_count: return_count as u32,
                return_type,
            };

            entries.push(entry);
        }

        Ok(TypeSection {
            count: count as u32,
            entries,
        })
    }
}
