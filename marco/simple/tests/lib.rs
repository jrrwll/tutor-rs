#![feature(log_syntax)]
#![feature(trace_macros)]

#[cfg(test)]
mod stringify;
#[cfg(test)]
mod trace;
#[cfg(test)]
mod vec;

macro_rules! four {
    () => {
        2 + 2
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, four!());
        assert_eq!(2 + 2, four![]);
        assert_eq!(2 + 2, four! {});
    }
}
