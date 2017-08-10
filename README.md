_Author_: Dan Kaufman

_Date_: 8/7/17
    
run 'cargo test' to test functionality
    
The minimum desirable functionality is this:

* Structure:
  
  * A blog has a title and a collection of posts. (Done)
  
  * A post has a title, an author, a body,  a collection of labels, and a collection of comments. (Done)
  
  * A comment has an author and a body. (Done)

* Functionality:

  * CRUD for posts and comments. (Partially done. Cannot add comment to a post in a blog because 'read_post' does not currently take owndership of the Post from the Blog)

  * Find all posts or comments or both by:
    
    * Title search    (Done)
        
    * Author search   (Done)
        
    * Label search    (Done)
        
    * Body search     (Does not compile (currently commented out) -- issue with searching using '.contains' on a String)