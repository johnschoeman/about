use pulldown_cmark::{Parser, html};

pub struct BlogPost {
    pub slug: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub markdown: &'static str,
}

impl BlogPost {
    pub fn render_html(&self) -> String {
        let parser = Parser::new(self.markdown);
        let mut output = String::new();
        html::push_html(&mut output, parser);
        output
    }
}

pub fn all_posts() -> Vec<BlogPost> {
    let mut posts = vec![
        BlogPost {
            slug: "hello-world",
            title: "Hello, World",
            date: "2026-03-10",
            markdown: include_str!("../../../../content/blog/hello-world.md"),
        },
    ];
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    posts
}

pub fn find_post(slug: &str) -> Option<BlogPost> {
    all_posts().into_iter().find(|p| p.slug == slug)
}
