

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use strings::StringExt;
        let mut test = String::from("Hello world");
        test.pop_f();
        assert_eq!('e', test.pop_f());
        assert_eq!("llo world", &test);
    }
}

pub mod strings {

    pub trait StringExt {

        fn pop_f(&mut self) -> char;



    }
    impl StringExt for String {
        fn pop_f(&mut self) -> char {
            self.remove(0)
        }
    }

}
