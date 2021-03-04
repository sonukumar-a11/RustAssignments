use crate::arithmetical::ImplementComplexNumber;
use crate::implement_student::{Score, StudentDetail};

#[test]
fn adding_complex_number() {
    let complex1 = ImplementComplexNumber {
        real: 42,
        imaginary: 23,
    };
    let complex2 = ImplementComplexNumber {
        real: 15,
        imaginary: 30,
    };
    let output: String = complex1.addition(complex2);
    assert_eq!(output, "57+i53");
}

#[test]
fn subtract_complex_number() {
    let complex1 = ImplementComplexNumber {
        real: 42,
        imaginary: 30,
    };
    let complex2 = ImplementComplexNumber {
        real: 15,
        imaginary: 23,
    };
    let output: String = complex1.subtraction(complex2);
    assert_eq!(output, "27-i7");
}

#[test]
fn multiply_complex_number() {
    let complex1 = ImplementComplexNumber {
        real: 4,
        imaginary: 25,
    };
    let complex2 = ImplementComplexNumber {
        real: 0,
        imaginary: 2,
    };
    let output: String = complex1.multiplication(complex2);
    assert_eq!(output, "0+i50");
}

#[test]
fn checking_new_in_student() {
    let student_marks1 = Score {
        hindi: 30,
        english: 32,
        maths: 34,
        science: 64,
    };
    let student1 = StudentDetail::new(&student_marks1);
    assert_eq!(student1.name, "Sonu");
    assert_eq!(student1.roll_number, 0);
    assert_eq!(student1.department, "intermediate");
    assert_eq!(student1.school, "DPS");
    assert_eq!(student1.score_of_each_subject.hindi, 30);
    assert_eq!(student1.score_of_each_subject.english, 32);
    assert_eq!(student1.score_of_each_subject.maths, 34);
    assert_eq!(student1.score_of_each_subject.science, 64);
}

#[test]
fn get_average() {
    let student_marks1 = Score {
        hindi: 30,
        english: 32,
        maths: 34,
        science: 64,
    };
    let check_test = StudentDetail::new(&student_marks1);
    assert_eq!(check_test.get_average(), 40 as f32);
}

#[test]
fn pass_student() {
    let student_marks1 = Score {
        hindi: 30,
        english: 32,
        maths: 34,
        science: 65,
    };
    let mut check_test = StudentDetail::new(&student_marks1);
    check_test.pass_student();
    assert_eq!(check_test.score_of_each_subject.hindi, 35);
    assert_eq!(check_test.score_of_each_subject.english, 35);
    assert_eq!(check_test.score_of_each_subject.maths, 35);
    assert_eq!(check_test.score_of_each_subject.science, 65);
}
