use crate::context::UserDetails;

enum ProviderStatus {
    NoSuchUser,
    IncorrectPassword,
    NoSuchUserOrPassword    
}

/// An authentication provider
/// Types that implement this can connect to a database and fetch the correct user details
/// If authentication has failed it should return a ProviderError
trait AuthenticationProvider<T: UserDetails> {
    fn provide<S: Into<String>> (&self, username: S, password: S) -> Result<T, ProviderStatus>;
}