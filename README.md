_Author_: Dan Kaufman
    
run 'cargo test' to test functionality
    
The minimum desirable functionality is this:

* Structure:
  
  * A blog has a title and a collection of posts. (Done)
  
  * A post has a title, an author, a body,  a collection of labels, and a collection of comments. (Done)
  
  * A comment has an author and a body. (Done)

* Functionality:

  * CRUD for posts and comments. (Done)

  * Find all posts or comments or both by:
    
    * Title search    (Done)
        
    * Author search   (Done)
        
    * Label search    (Done)
        
    * Body search     (Done)
    
  * All searches return partial matches (if text contains the search string anywhere in it, it will return)
