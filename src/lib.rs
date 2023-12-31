
pub mod cli;

use std::{path::PathBuf, io::{BufReader, BufWriter}, fs::{File, self}};

use anyhow::Result;
use cli::FileFormat;

#[derive(Debug)]
pub enum FileData {
    Json(serde_json::Value),
    Toml(toml::Value),
}

pub fn get_file_format(file: &PathBuf, format: Option<FileFormat>) -> Result<FileFormat> {
    if let Some(format) = format {
        Ok(format)
    } else {
        if let Some(filename) = file.file_name() {
            if let Some(filename) = filename.to_str() {
                match filename {
                    "Cargo.lock" => return Ok(FileFormat::Toml),
                    _ => {},
                }
            }
        }
        let ext = file
            .extension()
            .ok_or(anyhow::anyhow!("Could not get extension"))?
            .to_string_lossy()
            .to_string();
        
        match ext.as_str() {
            "json" => Ok(FileFormat::Json),
            "toml" => Ok(FileFormat::Toml),
            other_ext => Err(anyhow::anyhow!("Extension '{other_ext:?}' not supported")),
        }
    }
}

pub fn deserialize_streaming(file: &PathBuf, format: FileFormat) -> Result<FileData> {
    let reader = BufReader::new(File::open(file)?);
    match format {
        FileFormat::Json => Ok(FileData::Json(serde_json::from_reader(reader)?)),
        FileFormat::Toml => Err(anyhow::anyhow!("Toml does not support streaming deserialize/serialize")),
    }
}

pub fn deserialize_default(file: &PathBuf, format: FileFormat) -> Result<FileData> {
    let data = fs::read_to_string(file)?;
    match format {
        FileFormat::Json => Ok(FileData::Json(serde_json::from_str(&data)?)),
        FileFormat::Toml => Ok(FileData::Toml(toml::from_str(&data)?)),
    }
}

pub fn serialize_streaming(file: &PathBuf, data: FileData, pretty: bool) -> Result<()> {
    let writer = BufWriter::new(File::create(file)?);
    match data {
        FileData::Json(content) => if pretty {
                serde_json::to_writer_pretty(writer, &content)?;
            } else {
                serde_json::to_writer(writer, &content)?;
            },
        FileData::Toml(_) => Err(anyhow::anyhow!("Toml does not support streaming deserialize/serialize"))?,
    }
    Ok(())
}

pub fn serialize_default(file: &PathBuf, data: FileData, pretty: bool) -> Result<()> {
    let content = match data {
        FileData::Json(content) => if pretty {
                serde_json::to_string_pretty(&content)?
            } else {
                serde_json::to_string(&content)?
            },
        FileData::Toml(content) => if pretty {
                toml::to_string_pretty(&content)?
            } else {
                toml::to_string(&content)?
            },
    };
    fs::write(file, content)?;
    Ok(())
}
