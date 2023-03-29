#[macro_export]
macro_rules! get_arg {
    ( $args:ident, $name:expr ) => {
        $args
            .get_one::<String>($name)
            .expect(&format!("{} was not provided", $name))
            .clone()
    };
}
