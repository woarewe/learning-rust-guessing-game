pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(self: &Self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

pub struct NewsFeed {
   pub messages: Vec<String>
}

impl Summary for NewsFeed {
    fn summarize_author(&self) -> String {
        String::from("Internet")
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_v2<T: Summary>(item: &T) {
    println!("Breaking news v2! {}", item.summarize());
}

pub fn notify_v3<T>(item: &T)
where
    T: Summary
{
    println!("Breaking news v3! {}", item.summarize());
}

impl Summary for String {
    fn summarize(&self) -> String {
        format!("{}", self)
    }

    fn summarize_author(&self) -> String {
        String::from("None")
    }
}