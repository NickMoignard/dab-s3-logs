use std::collections::HashMap;
use std::process::Command;
use std::str;
use anyhow::Result;
use dateparser::parse;
// const TEMP: &str = "aws s3 --profile=production-internal  ls s3://dabble-production-kube-logs/production/ | awk '{print $2}";

const AWS_CLI_CMD: &str = "aws";
const S3_SUB_CMD: &str = "s3";
const LIST_OBJECTS_SUB_CMD: &str = "ls";
const S3_URI_PREFIX: &str = "s3://";

fn build_profile_arg(profile: &str) -> String {
  format!("--profile={}", profile)
}

fn build_s3_uri(bucket: &str, obj_path: &str) -> String {
  format!("{}{}/{}", S3_URI_PREFIX, bucket, obj_path)
}

pub fn test_cmd() {
  let result = recurse_through_bucket("staging-internal", "dabble-staging-kube-logs", Some("staging/")).unwrap().unwrap();

  for (key, value) in result.iter() {
    println!("{}: {:?}", key, value);
  }
}

pub fn query_s3_objects(profile: &str, bucket: &str, obj_path: &str) -> Vec<String> {
  let output = Command::new(AWS_CLI_CMD)
    .arg(S3_SUB_CMD)
    .arg(LIST_OBJECTS_SUB_CMD)
    .arg(build_profile_arg(profile))
    .arg(build_s3_uri(bucket, obj_path))
    .output();
  let stdout = output.unwrap().stdout;

  parse_stdout(stdout)
}

#[derive(Debug)]
enum NestedValue {
  Map(HashMap<String, NestedValue>),
  Value(String),
}


// TODO: split this up onto multiple async threads with tokio
// TODO: add progress display with indicatif
// TODO: add error handling
// TODO: add completions to --prefix, --bucket options
// TODO: check dependancy is installed and available (aws-cli-v2)
fn recurse_through_bucket(profile: &str, bucket: &str, obj_path_option: Option<&str>) -> Result<Option<HashMap<String, NestedValue>>> {
  let obj_path: &str = match obj_path_option {
    Some(path) => path,
    None => "",
  };
  
  let mut map: HashMap<String, NestedValue> = HashMap::new();
  let objects = query_s3_objects(profile, bucket, obj_path);
  for obj in objects {
    if obj.ends_with('/') {
      println!("Recursing into bucket: {}", obj);
      let obj = obj.trim_end_matches('/');
      match parse(obj) {
        Ok(_) => {
          // don't recurse into directories with names that can be parsed into a date
          map.insert(obj.to_string(), NestedValue::Map(HashMap::new()));
        }
        Err(_) => {
          // if the directory name can't be parsed into a date assume it's a directory with nested directories
          let result = recurse_through_bucket(profile, bucket, Some(obj)).unwrap().unwrap();
          map.insert(obj.to_string(), NestedValue::Map(result));
        }
      }
    } else {
      return Ok(None);
    }
  }

  Ok(Some(map))
}

fn parse_stdout(stdout: Vec<u8>) -> Vec<String> {
  let output = str::from_utf8(&stdout).unwrap();
  output
    .split('\n')
    .map(|line| line.trim().to_string().replace("PRE", "").trim().to_string()).filter_map(|item| {
      if item.is_empty() {
        None
      } else {
        Some(item)
      }
    })
    .collect()
}