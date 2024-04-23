use tokio;
use chrome_native_messaging::event_loop;
use serde::Serialize;
use serde_json::Value;


#[derive(Serialize)]
struct BasicMessage<'a> {
    payload: &'a str
}

#[tokio::main]
async fn main() {
    event_loop(|value| match value {
         Value::Null => Err("null payload"),
         _ => Ok(BasicMessage { payload: "Hello, World!" })
    });
}




















//#[tokio::main]
//async fn main() {
//    loop {
//        let mut input = String::new();
//        io::stdin()
//            .read_line(&mut input)
//            .expect("Failed to read Native Chrome Message");
//        format!("Received Chrome Native Message: {}", &input.trim());
//
//        io::stdout()
//            .write_all("OK from Native Host".as_bytes())
//            .expect("Failed to write response to Native Chrome Message");
//        io::stdout().flush().expect("Failed to flush output");
//
//        let resp = reqwest::get(format!("http://127.0.0.1:3030/hello/{}", &input.trim()))
//            .await
//            .expect("contact native mesaging server")
//            .json::<HashMap<String, String>>()
//            .await;
//        println!("{resp:#?}");
//    }
//}
