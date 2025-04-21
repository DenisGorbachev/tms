pub trait Shape<Input> {
    /// This type would normally be equal to `()`, but it can be overridden by the implementation to return some data (e.g. diagnostics)
    type Output;
    type Error;

    /// This function should take `&mut self` because that would allow the implementation to change the underlying structure (e.g. cache some data, or interact with the user on the terminal)
    /// This function should return a [`Result`] because it may call other functions which return [`Result`], so it would be convenient to use `?` in the implementation
    fn shape(&mut self, input: &mut Input) -> Result<Self::Output, Self::Error>;
}
