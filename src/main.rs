use tokio::process::Command;

#[tokio::main]
async fn main() {
    let p = Command::new("mount").output().await.unwrap();
    // println!("{:?}",p);
    // println!("{:?}",p.stdout);
    String::from_utf8(p.stdout).unwrap().lines().for_each(|l| println!("{l}"));
}
