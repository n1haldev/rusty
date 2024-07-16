//use reqwest::blocking::Client;
use reqwest::Client;
use reqwest::Error;

// Blocking request
//fn main() -> Result<(), Error> {
//    let client = Client::new();
//    let user = "testuser".to_string();
//    let passwd : Option<String> = None;
//    let res = client.get("http://httpbin.org/get")
//        .basic_auth(user, passwd)
//        .send().expect("Failed to send request");
//
//    println!("Status: {}", res.status());
//    println!("Headers:\n{:#?}", res.headers());
//    println!("Body:\n{}", res.text()?);
//
//    Ok(())
//}

 //Asynchronous requests
#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();
    let user = "testuser".to_string();
    let passwd : Option<String> = None;
    let res = client.get("http://httpbin.org/get")
        .basic_auth(user, passwd)
        .send().await.expect("Failed to send request");

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", res.text().await?);

    Ok(())
}
