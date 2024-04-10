use aws::profiles::get_aws_profiles::get_aws_profiles;

#[tokio::main]
async fn main() {
  let result = get_aws_profiles().unwrap();

  println!("AWS PROFILES {:?}", result);
}