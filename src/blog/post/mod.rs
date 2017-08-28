pub mod comment;
use self::comment::Comment;

#[derive(Debug, PartialEq)]
pub struct Post {
    pub title: String,
    pub author: String,
    pub body: String,
    pub labels: Vec<String>,
    pub comments: Vec<Comment>,
}

impl Post {
    pub fn create_comment(&mut self, author: &str, body: &str) -> usize{
        let mut new_comment = Comment {
            author: String::new(),
            body: String::new(),
        };

        new_comment.author.push_str(author);
        new_comment.body.push_str(body);

        self.comments.push(new_comment);
        self.comments.len()
    }

    pub fn read_comment(&self, comment_index: usize) -> &Comment {
        &self.comments[comment_index]
    }

    // only possible to update body of comment
    // -- changing the author does not really make sense
    pub fn update_comment(&mut self, comment_index: usize, body: &str) {
        let mut new_body = String::new();
        new_body.push_str(body);
        self.comments[comment_index].body = new_body;
    }

    pub fn delete_comment(&mut self, comment_index: usize) -> Comment {
        self.comments.remove(comment_index)
    }
}

