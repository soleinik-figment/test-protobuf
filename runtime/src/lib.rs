pub trait AscSomething{

}
pub trait ToAscObj<T: ?Sized>{
    fn to(&self);
}

pub trait FromAscObj<T: ?Sized>{
    //fn from();
}


pub trait AscType: Sized {
    /// Transform the Rust representation of this instance into an sequence of
    /// bytes that is precisely the memory layout of a corresponding Asc instance.
    fn to_asc_bytes(&self);// -> Result<Vec<u8>, DeterministicHostError>;

    // /// The Rust representation of an Asc object as layed out in Asc memory.
    // fn from_asc_bytes(
    //     asc_obj: &[u8],
    //     api_version: &Version,
    // ) -> Result<Self, DeterministicHostError>;

    // fn content_len(&self, asc_bytes: &[u8]) -> usize {
    //     asc_bytes.len()
    // }

    // /// Size of the corresponding Asc instance in bytes.
    // /// Only used for version <= 0.0.3.
    // fn asc_size<H: AscHeap + ?Sized>(
    //     _ptr: AscPtr<Self>,
    //     _heap: &H,
    //     _gas: &GasCounter,
    // ) -> Result<u32, DeterministicHostError> {
    //     Ok(std::mem::size_of::<Self>() as u32)
    // }
}

pub trait AscValue: AscType + Copy + Default {}



pub trait AscIndexId {
    /// Constant string with the name of the type in AssemblyScript.
    /// This is used to get the identifier for the type in memory layout.
    /// Info about memory layout:
    /// https://www.assemblyscript.org/memory.html#common-header-layout.
    /// Info about identifier (`idof<T>`):
    /// https://www.assemblyscript.org/garbage-collection.html#runtime-interface
    const INDEX_ASC_TYPE_ID: IndexForAscTypeId;
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum IndexForAscTypeId {
    String = 0,
    /* trimmed */
}
