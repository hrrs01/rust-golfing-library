


#[macro_use]
pub mod g_macro {
    #[macro_export]
    // Loop through
    macro_rules! each {
        ($vec_name:ident, $var_name:ident, $code:block) => (
            for $var_name in $vec_name $code
        )
    }
    #[macro_export]
    //e Enumerate each
    macro_rules! enu {
        ($vec_name:ident, $indx_name:ident, $var_name:ident, $code:block) => (
            for ($indx_name, $var_name) in $vec_name.iter().enumerate() $code
        )

    }
    #[macro_export]
    // As usize index
    macro_rules! uzi {
        ($value:expr) => (
            $value as usize
        )
    }
    #[macro_export]
    macro_rules! str {
        ($var_name:ident, $value:expr) => {
            let $varname = String::from($value);
        }
    }



}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use strings::StringExt;
        use vecs::VecExt;
        use nums::*;
        let mut test = String::from("Hello world");
        //test.pop_f();
        //assert_eq!('e', test.pop_f());
        //assert_eq!("llo world", &test);
        test.after_each(" ");
        assert_eq!(String::from("5").int::<i32>(), 5);




    }
}

pub mod nums {

    // Custom Num trait for all number types
    pub trait Num: ::std::str::FromStr {}
    impl Num for u32{}
    impl Num for u64{}
    impl Num for i32{}
    impl Num for i64{}
    impl Num for f32{}
    impl Num for f64{}

    pub trait NumExt{
        fn root(&mut self, x: f64) -> f64;
    }

    impl NumExt for f64{
        fn root(&mut self, x: f64) -> f64{
            self.powf(x)
        }
    }
}


pub mod strings {
    use nums::Num;
    pub trait StringExt {
        //simple reverse pop function
        fn pop_f(&mut self) -> String;
        //function adding char after each char in string
        // Filter support to be added?
        fn after_each(&mut self, c: &str) -> String;

        // prints a string, forwards and backwards ?: replace with .rev()?

        fn reflect(&mut self);

        // Parse string to int?
        fn int<T>(&mut self) -> T where T: Num, <T as ::std::str::FromStr>::Err : ::std::fmt::Debug;

    }
    impl StringExt for String {
        fn pop_f(&mut self) -> String {
            let mut x: String = self.clone();
            x.remove(0);
            *self = t.clone();
            x
        }

        fn int<T>(&mut self) -> T where T: Num, <T as ::std::str::FromStr>::Err : ::std::fmt::Debug{
            self.parse::<T>().expect("Not supported type")
        }

        fn after_each(&mut self, c: &str ) -> String{
            let mut t: String = String::new();

            for e in self.chars() {
                t.push(e);
                t.push_str(c);
            }
            *self = t.clone();
            t

        }
        fn reflect(&mut self){
            println!("{}{}", self,self.chars().rev().collect::<String>());
        }


    }
    impl StringExt for &'static str {
        fn pop_f(&mut self) -> String {
            // does not acctually modify string :? solve with T generic type for return?
            let mut t = self.to_string();
            let x = t.remove(0);
            t

        }

        fn int<T>(&mut self) -> T where T: Num, <T as ::std::str::FromStr>::Err : ::std::fmt::Debug{
            self.parse::<T>().expect("Not supported type")
        }

        fn after_each(&mut self, c: &str ) -> String{
            let mut t: String = String::new();

            for e in self.chars() {
                t.push(e);
                t.push_str(c);
            }
            t

        }
        fn reflect(&mut self){
            println!("{}{}", self,self.chars().rev().collect::<String>());
        }
    }
}

pub mod vecs {
    pub trait VecExt<T> {


    }





}
