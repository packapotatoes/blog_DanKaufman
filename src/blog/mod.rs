pub mod post;
use blog::post::Post;

#[derive(Debug)]
pub struct Blog {
    pub posts: Vec<Post>
}

impl Blog {
    // a post must be created before comments can be added. labels are optional
    pub fn create_post(&mut self,
                       title: String,
                       author: String,
                       body: String,
                       labels_opt: Option<Vec<String>>) -> usize {
        let new_post = Post {
            title: title,
            author: author,
            body: body,
            labels: if let Some(lbl) = labels_opt {
                lbl
            }else {
                Vec::new()
            },
            comments: Vec::new(),
        };

        self.posts.push(new_post);

        // return index of new post
        self.posts.len()
    }

    //return reference to post at post_index
    pub fn read_post(&self, post_index: usize) -> &Post {
        &self.posts[post_index]
    }

    pub fn get_post(&mut self, post_index: usize) -> &mut Post {
        &mut self.posts[post_index]
    }

    // change title, body or labels of a post
    pub fn update_post(&mut self,
                       post_index: usize,
                       title: Option<String>,
                       body: Option<String>,
                       labels: Option<Vec<String>>) {

        if let Some(t) = title {
            self.posts[post_index].title = t;
        }

        if let Some(b) = body {
            self.posts[post_index].body = b;
        }

        if let Some(l) = labels {
            self.posts[post_index].labels = l;
        }
    }

    pub fn delete_post(&mut self, post_index: usize) -> Post {
        self.posts.remove(post_index)
    }

    // super basic linear search
    // returns a tuple: first value is vec of post indices that match author
    //                  second value is vec of post and commment indices of comments that match
    pub fn search_by_author(&self, author: &str) -> (Vec<usize>, Vec<(usize, usize)>) {
        let mut post_matches: Vec<usize> = Vec::new();
        let mut comment_matches: Vec<(usize, usize)> = Vec::new();

        for (post_index, post) in self.posts.iter().enumerate() {
            if post.author.contains(author) {
                post_matches.push(post_index);
            }

            for (com_index, comment) in post.comments.iter().enumerate() {
                if comment.author.contains(author) {
                    comment_matches.push((post_index, com_index));
                }
            }
        }

        (post_matches, comment_matches)
    }

    pub fn search_by_title(&self, title: &str) -> Vec<usize> {
        let mut post_matches: Vec<usize> = Vec::new();

        for (index, post) in self.posts.iter().enumerate() {
            if post.title.contains(title) {
                post_matches.push(index);
            }
        }

        post_matches
    }

    pub fn search_by_label(&self, label: &str) -> Vec<usize> {
        let mut post_matches: Vec<usize> = Vec::new();

        for (index, post) in self.posts.iter().enumerate() {
            for lab in post.labels.iter() {
                if lab.contains(label) {
                    post_matches.push(index);
                }
            }
        }

        post_matches
    }

    pub fn search_by_body(&self, text: &str) -> (Vec<usize>, Vec<(usize, usize)>) {
        let mut post_matches: Vec<usize> = Vec::new();
        let mut comment_matches: Vec<(usize, usize)> = Vec::new();

        for (post_index, post) in self.posts.iter().enumerate() {
            if post.body.contains(text) {
                post_matches.push(post_index);
            }

            for (comment_index, comment) in post.comments.iter().enumerate() {
                if comment.body.contains(text) {
                    comment_matches.push((post_index, comment_index));
                }
            }
        }

        (post_matches, comment_matches)
    }

    pub fn create_comment(&mut self, post_index: usize, author: String, body: String) -> usize {
        self.posts[post_index].create_comment(&author, &body)
    }

    pub fn update_comment(&mut self, post_index: usize, comment_index: usize, body: String) {
        self.posts[post_index].update_comment(comment_index, body);
    }

}

