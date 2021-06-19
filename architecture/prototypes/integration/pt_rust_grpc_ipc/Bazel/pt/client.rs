#[tokio::main]
pub async fn main() {
    let hello_world = tokio::spawn( async {
        println!("Hello, world!");
    });
    hello_world.await.unwrap();
}
