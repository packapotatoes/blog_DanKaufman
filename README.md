_Author_: Dan Kaufman

_Date_: 8/7/17
    
run 'cargo test' to test functionality
    
In the name of practice and because I need a porject to work on, I am continuing to work on this in a new branch.In the name of practice and because I need a porject to work on, I am continuing to work on this in a new branch.

The minimum desirable functionality is this:

* Structure:
  
  * A blog has a title and a collection of posts. (Done)
  
  * A post has a title, an author, a body,  a collection of labels, and a collection of comments. (Done)
  
  * A comment has an author and a body. (Done)

* Functionality:

  * CRUD for posts and comments. (Partially done. Adding comment to a post that is part of a blog does not work. Fixed in branch `updates` )

  * Find all posts or comments or both by:
    
    * Title search    (Done)
        
    * Author search   (Done)
        
    * Label search    (Done)
        
    * Body search     (Does not compile (currently commented out) -- issue with searching using `.contains` on a String)
    
    

