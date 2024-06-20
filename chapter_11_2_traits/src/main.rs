// Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.

trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle{
    headline:String,
    location:String,
    author:String,
    content:String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})" , self.headline, self.author, self.location)
    }
}

struct Tweet{
    username:String,
    content:String,
    reply:bool,
    retweet:bool
}

impl  Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}" , self.username, self.content)
    // }
}

// Default Trait

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Breaking News!"),
        location: String::from("Istanbul"),
        author: String::from("Jane Doe"),
        content: String::from("Lorem ipsum dolor sit amet..."),
    };

    let tweet = Tweet {
        username: String::from("@user"),
        content: String::from("Hello, world!"),
        reply: false,
        retweet: false,
    };

    println!("New article available! {}", article.summarize());
    println!("New tweet posted! {}", tweet.summarize());

    notify(&article);
    notify2(&tweet);
}
