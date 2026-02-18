// The User struct should contain 3 fields. email, which is a String; 
// password, which is also a String; and requires_2fa, which is a boolean. 
pub struct User {
    user: String,
    email: String,
    requires_2fa: bool,
}

impl User {
    fn new(user: String, email: String, requires_2fa: bool ) -> Self {
        User{
            user: user,
            email: email,
            requires_2fa: requires_2fa,
        }
    } // add a constructor function called `new`
}
