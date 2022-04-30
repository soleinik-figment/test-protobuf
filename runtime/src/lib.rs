pub trait AscSomething{

}
pub trait ToAscObj<T: ?Sized>{
    fn to(&self);
}

pub trait FromAscObj<T: ?Sized>{
    //fn from();
}