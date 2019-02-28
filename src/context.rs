/// This enum reflects the state of an authentication context.
/// 
/// It can be either be not authenticated (anynomous) or authenticated.
/// If the context is authenticated, extra details must be provided
/// about the current user
pub enum AuthenticationStatus {
    Anynomous,
    Authenticated
}

/// This trait provides some methods to fetch authentication information
/// from the authentication context
pub trait UserDetails {
    /// Returns true if the user has given role
    fn has_role(&self, role: String) -> bool;

    /// Returns a list of roles the user has
    fn get_roles(&self) -> Vec<String>;

    /// Returns the username of the user
    fn get_username(&self) -> String;

    /// Returns the password of the user
    fn get_password(&self) -> String;
}

/// This struct defines an authentication context
/// It will get populated when user authentication is successfull
/// and passed down to request handlers
pub struct AuthenticationContext<T: UserDetails> {
    status: AuthenticationStatus,
    details: Option<T>
}

impl<T: UserDetails> AuthenticationContext<T> {
    /// Create a new authenticaton context
    pub fn new() -> AuthenticationContext<T> {
        Self::anynomous()
    }

    /// Create an anynomous authentication context
    pub fn anynomous() -> AuthenticationContext<T>  {
        AuthenticationContext {
            status: AuthenticationStatus::Anynomous,
            details: None
        }
    }

    /// create an authenticated authentication context
    pub fn authenticated(details: T) -> AuthenticationContext<T> {
        AuthenticationContext {
            status: AuthenticationStatus::Authenticated,
            details: Some(details),
        }
    }
}