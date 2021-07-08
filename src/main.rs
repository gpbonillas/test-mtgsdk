use mtgsdk::cards;

#[tokio::main]
async fn main() {
    let result = cards::find(46012).await;

    if let Ok(card) = result{
        println!("Card name: {}", card.name)
    };
}