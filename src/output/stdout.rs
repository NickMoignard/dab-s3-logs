use std::{io::{self, Read, Write}, path::Path};
use file_format::FileFormat;
use flate2::read::GzDecoder;
use serde_json::{Result, Value};

pub fn output_logfile (path: &Path) {
  if is_gzipped(path) {
      let val = parse_gzipped_logfile(path).unwrap();
      pipe_logs_to_stdout(val);
  }
}

fn is_gzipped (path: &Path) -> bool {
  let fmt = FileFormat::from_file(path).unwrap();

  match fmt {
      FileFormat::Gzip => true,
      _ => false
  }
}

fn parse_gzipped_logfile (path: &Path) -> Result<Value> {
  let bytes = std::fs::read(path).unwrap();
  let mut d = GzDecoder::new(&bytes[..]);

  let mut s = String::new();
  d.read_to_string(&mut s).unwrap();

  serde_json::from_str(&s)
}

fn pipe_logs_to_stdout (val: Value) {
  if val.is_array() {
      val.as_array().unwrap().iter().for_each(|obj| {
          let _ = pipe_json_obj_to_stdout(obj.clone());
      });
  }

  if val.is_object() {
      let _ = pipe_json_obj_to_stdout(val);
  }
}

fn pipe_json_obj_to_stdout (val: Value) -> Result<()>{
  let mut stdout = io::stdout();
  let mut bytes: Vec<u8> = Vec::new();
  serde_json::to_writer(&mut bytes, &val).unwrap();

  let _ = stdout.write_all(&bytes);
  
  Ok(())
}