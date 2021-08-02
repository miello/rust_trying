use day9::{ NewsArticle, Summary, notify, returns_summarizable };
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn test(s: &str) -> &str{
    s
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article: {}", article.summarize());
    notify(&article);
    let tmp = returns_summarizable();
    println!("{}", tmp.summarize());

    let r = 1;

    {
        let x = 5;
        // Error: Lifetimes is not long enough
        // r = &x;
    }

    // let string1 = String::from("abcd");
    // let result;
    // {
    //     let string2 = String::from("xyz");

    //     result = longest(string1.as_str(), string2.as_str());
    // }

    // println!("{}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence
    };

    i.announce_and_return_part("Hello");
    println!("{:?}", i);

    println!("r: {}", r);

    
    let s: &'static str = "I have static lifetime";

    test(s);

    // Remark to review about lifetimes

    let s = "Hello";
    let t = "LLL";

    println!("{}", s);
    longest_with_an_announcement(s, t, String::from("Hello World"));
}

/*
&i32 -> Ref
&'a i32 -> Ref with explicit lifetime
&'a mut i32 -> Mutable ref with explicit lifetime
*/
