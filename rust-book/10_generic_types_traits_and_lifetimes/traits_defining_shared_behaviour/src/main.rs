use aggregator::{Summary, Tweet, NewsArticle};

fn main() {
    let tweet = Tweet {
        username: String::from("hose_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Chapionship!"),
        location: String::from("Pittsburg, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best \
            hockey team in NHL."),
    };

    println!("New Article available! {}", article.summarize());
}
