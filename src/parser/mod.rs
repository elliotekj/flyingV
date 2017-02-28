use std::collections::HashMap;

pub struct Post {
    pub title: String,
}

pub fn post(post_string: String) {
    if let Some((frontmatter, post)) = separate_frontmatter(post_string) {
        println!("{}", post);
    } else {
    }
}

fn separate_frontmatter(post_string: String) -> Option<(HashMap<String, String>, String)> {
    if let Some(frontmatter_len) = post_string.find("\n\n") {
        let frontmatter = HashMap::new();
        let frontmatter_string = &post_string[..frontmatter_len];
        let post = &post_string[frontmatter_len..];
        println!("{}", frontmatter_string);

        Some((frontmatter, post.to_owned()))
    } else {
        None
    }
}
