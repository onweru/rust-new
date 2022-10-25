pub trait Summary {
  fn summarize(&self) -> String;

  fn content(&self) -> String {
    format!("loading... {}", &self.summarize_author())
  }

  fn summarize_author(&self) -> String;
}

// struct has reference as the value of one if fields, hence an explicit lifetime is needed
pub struct NewsArticle<'a> {
  pub headline: String,
  pub location: String,
  pub author: &'a String,
  pub content: String,
}

impl Summary for NewsArticle<'_> {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }

  fn summarize_author(&self) -> String {
    format!("@{}", &self.author)
  }
}

pub struct Tweet<'a> {
  pub username: &'a String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}
impl Summary for Tweet<'_> {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }

  fn summarize_author(&self) -> String {
    format!("@{}", &self.username)
  }
}

pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}