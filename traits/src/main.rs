
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}


fn main() {
    let tweet = Tweet {
        username:String::from("marcdhi"),
        content: String::from("Hemlo world!"),
        reply: false,
        retweet: false
    };

    let article = NewsArticle {
        author: String::from("Mardav"),
        headline: String::from("This is the heading"),
        content: String::from("Hola this is the content of my articles")
    };

    println!("Tweet Summed up: {}", tweet.summarize());
    println!("Article Summed up: {}", article.summarize());
}
