
// Trait is similar to Interface and looks closely 
// related to High order function which can be 
// used as cross cutting concerns, beauty is it is not hardcoded inside
// struct type and can be used by any of the types
pub trait Summary {
    // it can simple like below or it can have
    // default implementations
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: Srring,
    pub author: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

// implementation provided for news article
// note that we can implement display trait from std:: 
// but we cant implement external trait on extrnal type
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format("{} by {}", self.headline, self.author)
    }
}

// implementation provided for tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format("{} by {}", self.username, self.content)
    }
}

// accepts any item which implements Summary trait
fn notify(any_item: &impl Summary) {
    println!("Any summary {}", any_item.summarize());
}

// above syntacti sugar, this is longer syntax which is more
// verbose - trait bound syntax
fn notify_generic_trait<T: Summary>(item: &T) {
    println!("Any summary {}", item.summarize());
}

// multiple traits
// fn notify_multiple_traits(item: &(impl Summary + Display)) {}
// fn notify_multiple_traits<T: Summary + Display>(item: &T) {}

// where syntax
// fn some_fn<T, U>(t: &T, u: &U) -> i32 
//     where T: Display + Clone,
//           U: Clone + Debug
// {

// }

// return
// fn returns_summarizable() -> impl Summary { }

fn main() {
    let article = NewsArticle {
        headline: String::from("Article headline"),
        author: String::from("Me"),
    };

    println!("New article available: {}", article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Tweeting..."),
    };

    println!("New tweet available: {}", tweet.summarize());
}
