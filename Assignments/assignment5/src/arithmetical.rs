/// This Structure used for encapsulate the complex number
///
/// #Arguments
/// real
/// imaginary

pub struct ImplementComplexNumber {
    pub(crate) real: i32,
    pub(crate) imaginary: i32,
}

impl ImplementComplexNumber {
    /// This method Used to addition of two Complex Number
    ///
    /// #Arguments
    ///
    /// Self :- take real and imaginary number
    /// otherness:- take another complex number to perform some computational task
    ///
    /// #Return
    ///
    /// this method return the string as output of complex number
    pub fn addition(&self, otherness: ImplementComplexNumber) -> String {
        let result_real: i32 = self.real + otherness.real;
        let result_imaginary: i32 = self.imaginary + otherness.imaginary;
        format!("{}+i{}", result_real, result_imaginary)
    }
    /// This method Used to Subtraction of two Complex Number
    ///
    /// #Arguments
    ///
    /// Self :- take real and imaginary number
    /// otherness:- take another complex number to perform some computational task
    ///
    /// #Return
    ///
    /// this method return the string as output of complex number
    pub fn subtraction(&self, otherness: ImplementComplexNumber) -> String {
        let result_real: i32 = self.real - otherness.real;
        let result_imaginary: i32 = self.imaginary - otherness.imaginary;
        format!("{}-i{}", result_real, result_imaginary)
    }
    /// This method Used to Multiplication of two Complex Number
    ///
    /// #Arguments
    ///
    /// Self :- take real and imaginary number
    /// otherness:- take another complex number to perform some computational task
    ///
    /// #Return
    ///
    /// this method return the string as output of complex number
    pub fn multiplication(&self, otherness: ImplementComplexNumber) -> String {
        let result_real: i32 = self.real * otherness.real;
        let result_imaginary: i32 = self.imaginary * otherness.imaginary;
        format!("{}+i{}", result_real, result_imaginary)
    }
}
