
#[macro_use]
pub mod g_macro {
    #[macro_export]
    macro_rules! each {
        ($vec_name:ident, $var_name:ident, $code:block) => (
            for $var_name in $vec_name $code
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use strings::StringExt;
        let mut test = String::from("Hello world");
        //test.pop_f();
        //assert_eq!('e', test.pop_f());
        //assert_eq!("llo world", &test);
        assert_eq!("H e l l o   w o r l d ",test.after_each(' '));
        let test = vec!([1,2,3,4,5]);
        each!(test, x, {println!("{:?}",x);})
    }
}

pub mod strings {

    pub trait StringExt {


        //simple reverse pop function
        fn pop_f(&mut self) -> char;
        //function adding char after each char in string
        // Filter support to be added?
        fn after_each(&mut self, c: char) -> Self;


    }
    impl StringExt for String {
        fn pop_f(&mut self) -> char {
            self.remove(0)
        }

        fn after_each(&mut self, c: char ) -> Self {
            let mut t: String = String::new();

            for e in self.chars() {
                t.push(e);
                t.push(c);
            }
            t.clone()


        }


    }




}

pub mod vecs {
    pub trait VecExt {
        fn find(&mut self, s: &str) -> usize;
    }

    impl<T> VecExt for Vec<T> where T: String{

    }

}
