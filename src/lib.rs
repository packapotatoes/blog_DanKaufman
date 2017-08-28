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
        CRUD for posts and comments. (Done)

        Find all posts or comments or both by:
            Title search    (Done)
            Author search   (Done)
            Label search    (Done)
            Body search     (Done)
***************************************************/

pub mod blog;

// these tests should probably be moved to various other files
// run with "-- --nocapture" to see stdout for more debug info
#[cfg(test)]
mod tests {
    use blog::Blog;
    use blog::post::Post;
    use blog::post::comment::Comment;
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

        let author = String::from("Jane Doe");
        post1.create_comment(&author,
                             "Comment content");

        println!("{}", author);
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

        post1.create_comment("Jane Doe",
                             "Comment content");

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

        post1.create_comment("Jane Doe", "Comment content");

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

        post1.create_comment("Jane Doe", "Comment content");

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

        let s = String::from("New Post Title");

        blog1.update_post(0,
                          Some(s),
                          Some(String::from("New post body")),
                          Some(vec![String::from("label1"), String::from("label2")]));

        //println!("{}", s);

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

        let (post_results, comment_results) = blog1.search_by_author(&String::from("John Doe"));

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

        blog1.create_post(String::from("Post Title1"),
                          String::from("John Doe"),
                          String::from("Post body"),
                          None);

        blog1.create_post(String::from("Post Title2"),
                          String::from("Jane Doe"),
                          String::from("Post body2"),
                          None);

        blog1.create_post(String::from("Post Title1"),
                          String::from("John Doe"),
                          String::from("Post body3"),
                          None);

        // test full match
        let post_results = blog1.search_by_title(&String::from("Post Title1"));
        let expected_results = vec![0, 2];
        assert_eq!(expected_results, post_results);

        let post_results = blog1.search_by_title(&String::from("Title2"));
        let expected_results = vec![1];
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

        let expected_post_results = vec![0, 1];

        // test full string match
        let post_results = blog1.search_by_label(&String::from("label2"));
        assert_eq!(expected_post_results, post_results);

        // test partial string match
        let post_results = blog1.search_by_label(&String::from("2"));
        assert_eq!(expected_post_results, post_results);

    }

    #[test]
    fn search_by_body_test() {
        let mut blog1 = Blog {
            posts: Vec::new(),
        };

        blog1.create_post(String::from("Post Title"),
                          String::from("John Doe"),
                          String::from("Post body1"),
                          None);


        blog1.create_post(String::from("Post Title"),
                          String::from("John Doe"),
                          String::from("Post body2"),
                          None);


        blog1.create_post(String::from("Post Title"),
                          String::from("John Doe"),
                          String::from("Post body3"),
                          None);

        blog1.create_comment(1,
                             String::from("John Doe"),
                             String::from("comment body0"));

        blog1.create_comment(2,
                             String::from("John Doe"),
                             String::from("comment body1"));

        let (post_results, comment_results) = blog1.search_by_body(&String::from("1"));

        println!("{:?}, {:?}", post_results, comment_results );

        let expected_post_results = vec![0];
        let expected_comment_results = vec![(2, 0)];

        assert_eq!(expected_post_results, post_results);
        assert_eq!(expected_comment_results, comment_results);
    }
}
