



pub trait AscSomething{

}
pub trait ToAscObj<T: ?Sized>{
    fn to();
}

pub trait FromAscObj<T: ?Sized>{
    fn from();
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
