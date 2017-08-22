/*************************************************
    Author: Dan Kaufman
    Date: 8/7/17

    run 'cargo test' to test functionality

    The minimum desirable functionality is this:

        Structure:
        A blog has a title and a collection of posts. (Done)
        A post has a title, an author, a body,  a collection of labels, and a collection of comments. (Done)
        A comment has an author and a body. (Done)

    Functionality:
        CRUD for posts and comments. (Partially done. Cannot add comment to a post in a blog
                                      because 'read_post' does not currently take owndership of the Post from the Blog)

        Find all posts or comments or both by:
            Title search    (Done)
            Author search   (Done)
            Label search    (Done)
            Body search     (Does not compile -- issue with searching using '.contains' on a String)
***************************************************/



#[derive(Debug)]
pub struct Blog {
    posts: Vec<Post>,
}

#[derive(Debug, PartialEq)]
pub struct Post {
    title: String,
    author: String,
    body: String,
    labels: Vec<String>,
    comments: Vec<Comment>,
}

#[derive(Debug, PartialEq)]
pub struct Comment {
    author: String,
    body: String,
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
    pub fn read_post(&mut self, post_index: usize) -> &Post {
        &self.posts[post_index]
    }

    pub fn get_post(&mut self, post_index: usize) -> &mut Post {
        &mut self.posts[post_index]
    }

    // change title, body or labels of a post
    pub fn update_post(&mut self, post_index: usize, title: Option<String>, body: Option<String>, labels: Option<Vec<String>>) {

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
    pub fn search_by_author(&mut self, author: String) -> (Vec<usize>, Vec<(usize, usize)>) {
        let mut post_matches: Vec<usize> = Vec::new();
        let mut comment_matches: Vec<(usize, usize)> = Vec::new();

        if self.posts.len() > 0 {
            for count in 0 .. self.posts.len() {
                if self.posts[count].author == author {
                    post_matches.push(count);
                }

                if self.posts[count].comments.len() > 0 {
                    for com_count in 0 .. self.posts[count].comments.len() {
                        if self.posts[count].comments[com_count].author == author {
                            comment_matches.push((count, com_count));
                        }
                    }
                }
            }
        }

        // couldn't figure out how to do this with iterators
        /*
        let mut iter = self.posts.iter();
        let mut done = false;

        while !done {
            let next_post = iter.next();

            if let Some()


            if let Some(i) = iter.find(|post| post.author == author) {
                match_indexes.push(i);
            } else {
                done = true;
            }
        }
        */

        (post_matches, comment_matches)
    }

    pub fn search_by_title(&mut self, title: String) -> Vec<usize> {
        let mut post_matches: Vec<usize> = Vec::new();

        for (index, post) in self.posts.iter().enumerate() {
            if *post.title == title {
                post_matches.push(index);
            }
        }

        post_matches
    }

    pub fn search_by_label(&mut self, label: String) -> Vec<usize> {
        let mut post_matches: Vec<usize> = Vec::new();

        for (index, post) in self.posts.iter().enumerate() {
            for lab in post.labels.iter() {
                if *lab == label {
                    post_matches.push(index);
                }
            }
        }

        post_matches
    }

    /*pub fn search_by_body(&mut self, text: String) -> (Vec<usize>, Vec<(usize, usize)>) {
        let mut post_matches: Vec<usize> = Vec::new();
        let mut comment_matches: Vec<(usize, usize)> = Vec::new();

        for count in 0 .. self.posts.len() {
            if self.posts[count].body.as_str().contains(text) {
                post_matches.push(count);
            }

            for com_count in 0 .. self.posts[count].comments.len() {
                if self.posts[count].comments[count].body.as_str().contains(text) {
                    comment_matches.push((count, com_count));
                }
            }
        }

        (post_matches, comment_matches)
    }*/

    pub fn create_comment(&mut self, post_index: usize, author: String, body: String) -> usize {
        self.posts[post_index].create_comment(author, body)
    }

    pub fn update_comment(&mut self, post_index: usize, comment_index: usize, body: String) {
        self.posts[post_index].update_comment(comment_index, body);
    }

}


impl Post {
    pub fn create_comment(&mut self, author: String, body: String) -> usize{
        let new_comment = Comment {
            author: author,
            body: body,
        };

        self.comments.push(new_comment);
        self.comments.len()
    }

    pub fn read_comment(&mut self, comment_index: usize) -> &Comment {
        &self.comments[comment_index]
    }

    // only possible to update body of comment -- changing the author does not really make sense
    pub fn update_comment(&mut self, comment_index: usize, body: String) {
        self.comments[comment_index].body = body;
    }

    pub fn delete_comment(&mut self, comment_index: usize) -> Comment {
        self.comments.remove(comment_index)
    }
}

// run with "-- --nocapture" to see stdout for more debug info
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_comment() {
        let comment1 = Comment {
            author: String::from("John Doe"),
            body: String::from("This comment is insightful."),
        };

        assert_eq!("John Doe", comment1.author);
        assert_eq!("This comment is insightful.", comment1.body);
    }

    #[test]
    fn build_post() {
        let post1 = Post {
            title: String::from("Post Title"),
            author: String::from("John Doe"),
            body: String::from("Post body"),
            labels: Vec::new(),
            comments: Vec::new(),
        };

        assert_eq!("Post Title", post1.title);
        assert_eq!("John Doe", post1.author);
        assert_eq!("Post body", post1.body);
    }

    #[test]
    fn create_comment_test() {
        let mut post1 = Post {
            title: String::from("Post Title"),
            author: String::from("John Doe"),
            body: String::from("Post body"),
            labels: Vec::new(),
            comments: Vec::new(),
        };

        post1.create_comment(String::from("Jane Doe"),
                             String::from("Comment content"));

        assert_eq!("Jane Doe", post1.comments[0].author);
        assert_eq!("Comment content", post1.comments[0].body);
    }

    #[test]
    fn read_comment_test() {
        let mut post1 = Post {
            title: String::from("Post Title"),
            author: String::from("John Doe"),
            body: String::from("Post body"),
            labels: Vec::new(),
            comments: Vec::new(),
        };

        post1.create_comment(String::from("Jane Doe"),
                             String::from("Comment content"));

        let expected_comment = Comment{
            author: String::from("Jane Doe"),
            body: String::from("Comment content")
        };

        let unexpected_comment = Comment {
            author: String::from("Jimmy Johns"),
            body: String::from("cOmMeNt CoNtEnT")
        };

        println!("Read comment: {:?}", *post1.read_comment(0));

        assert_eq!(expected_comment, *post1.read_comment(0));
        assert_ne!(unexpected_comment, *post1.read_comment(0));
    }

    #[test]
    fn update_comment_test() {
        let mut post1 = Post {
            title: String::from("Post Title"),
            author: String::from("John Doe"),
            body: String::from("Post body"),
            labels: Vec::new(),
            comments: Vec::new(),
        };

        post1.create_comment(String::from("Jane Doe"), String::from("Comment content"));

        assert_eq!("Comment content", post1.comments[0].body);
        post1.update_comment(0, String::from("New comment content"));
        assert_eq!("New comment content", post1.comments[0].body);
    }

    #[test]
    fn delete_comment_test() {
        let mut post1 = Post {
            title: String::from("Post Title"),
            author: String::from("John Doe"),
            body: String::from("Post body"),
            labels: Vec::new(),
            comments: Vec::new(),
        };

        post1.create_comment(String::from("Jane Doe"), String::from("Comment content"));

        let expected_comment = Comment{
            author: String::from("Jane Doe"),
            body: String::from("Comment content")
        };

        assert_eq!(expected_comment, post1.delete_comment(0));
        assert_eq!(0, post1.comments.len());
    }

    #[test]
    fn update_post_test() {
        let expected_post1 = Post {
            title: String::from("Post Title"),
            author: String::from("John Doe"),
            body: String::from("Post body"),
            labels: Vec::new(),
            comments: Vec::new(),
        };

        let mut blog1 = Blog {
            posts: Vec::new(),
        };

        blog1.create_post(String::from("Post Title"),
                          String::from("John Doe"),
                          String::from("Post body"), None);

        assert_eq!(expected_post1, blog1.posts[0]);

        blog1.update_post(0,
                          Some(String::from("New Post Title")),
                          Some(String::from("New post body")),
                          Some(vec![String::from("label1"), String::from("label2")]));

        let expected_post2 = Post {
            title: String::from("New Post Title"),
            author: String::from("John Doe"),
            body: String::from("New post body"),
            labels: vec![String::from("label1"), String::from("label2")],
            comments: Vec::new(),
        };

        assert_eq!(expected_post2, blog1.posts[0]);
    }

    #[test]
    fn search_by_author_test() {
        let mut blog1 = Blog {
            posts: Vec::new(),
        };

        blog1.create_post(String::from("Post Title"),
                          String::from("John Doe"),
                          String::from("Post body"),
                          None);

        blog1.create_post(String::from("Post Title2"),
                          String::from("Jane Doe"),
                          String::from("Post body2"),
                          None);

        blog1.create_post(String::from("Post Title3"),
                          String::from("John Doe"),
                          String::from("Post body3"),
                          None);

        blog1.create_comment(1,
                             String::from("John Doe"),
                             String::from("comment body"));

        let (post_results, comment_results) = blog1.search_by_author(String::from("John Doe"));

        let expected_post_results = vec![0, 2];
        let expected_comment_results = vec![(1, 0)];

        println!("{:?}", comment_results);
        println!("{:?}", blog1.posts[1]);

        assert_eq!(expected_post_results, post_results);
        assert_eq!(expected_comment_results, comment_results);
    }

    #[test]
    fn search_by_title_test() {
        let mut blog1 = Blog {
            posts: Vec::new(),
        };

        blog1.create_post(String::from("Post Title"),
                          String::from("John Doe"),
                          String::from("Post body"),
                          None);

        blog1.create_post(String::from("Post Title2"),
                          String::from("Jane Doe"),
                          String::from("Post body2"),
                          None);

        blog1.create_post(String::from("Post Title"),
                          String::from("John Doe"),
                          String::from("Post body3"),
                          None);

        let post_results = blog1.search_by_title(String::from("Post Title"));

        let expected_results = vec![0, 2];

        assert_eq!(expected_results, post_results);
    }

    #[test]
    fn search_by_label_test() {
        let mut blog1 = Blog {
            posts: Vec::new(),
        };

        blog1.create_post(String::from("Post Title"),
                          String::from("John Doe"),
                          String::from("Post body"),
                          Some(vec![String::from("label1"),
                                    String::from("label2"),
                                    String::from("label3")]));

        blog1.create_post(String::from("Post Title2"),
                          String::from("Jane Doe"),
                          String::from("Post body2"),
                          Some(vec![String::from("label4"),
                                    String::from("label2"),
                                    String::from("label3")]));

        blog1.create_post(String::from("Post Title"),
                          String::from("John Doe"),
                          String::from("Post body3"),
                          Some(vec![String::from("label1"),
                                    String::from("label4"),
                                    String::from("label3")]));



        let post_results = blog1.search_by_label(String::from("label2"));

        let expected_post_results = vec![0, 1];

        assert_eq!(expected_post_results, post_results);

    }

}
