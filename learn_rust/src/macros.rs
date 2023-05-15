// Closures can be of 3 types:
//   - Custom #[derive] macros
//   - Attribute-like macros on an item
//   - Function-like macros (e.g.: println!)


pub fn run(){

    // Example of a macro definition
    #[macro_export]
    macro_rules! my_vec {
        // Here i put a macro pattern
        ( $( $x:expr ),* ) => {

            // Here i can use the $x expression (this is a code expression)
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push_back($x);
                )*
                temp_vec
            }
        };
    }
}
