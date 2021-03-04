#[derive(Debug)]
/// This Structure give the Marks of student
///
/// #Arguments
/// hindi
/// english
/// maths
/// science

pub struct Score {
    pub(crate) hindi: i32,
    pub(crate) english: i32,
    pub(crate) maths: i32,
    pub(crate) science: i32,
}
/// This Structure give the Marks of student
///
/// #Arguments
///name
/// roll_number
/// score of student
/// department
/// school
pub struct StudentDetail {
    pub name: String,
    pub roll_number: i32,
    pub(crate) score_of_each_subject: Score,
    pub department: String,
    pub school: String,
}

//implementing Student Structure
impl StudentDetail {
    /// This method Used to initialised the input of student
    ///
    /// #Arguments
    ///
    /// score_of_each_subject :- Take a Score of student in particular marks
    ///
    /// #Return
    ///
    /// this method used to store initiated the data as constructor
    pub fn new(score_of_each_subject: &Score) -> StudentDetail {
        StudentDetail {
            name: "Sonu".to_owned(),
            roll_number: 0,
            score_of_each_subject: Score {
                hindi: score_of_each_subject.hindi,
                english: score_of_each_subject.english,
                maths: score_of_each_subject.maths,
                science: score_of_each_subject.science,
            },
            department: "intermediate".to_owned(),
            school: "DPS".to_owned(),
        }
    }
    /// This method  check the number marks is greater than 35 means to pass the student need marks
    ///
    /// #Arguments
    ///
    /// &self->to with vector
    ///
    /// #Return
    ///
    /// return vector of grace marks
    pub fn pass_student(&mut self) {
        if self.score_of_each_subject.hindi < 35 {
            let difference = 35 - self.score_of_each_subject.hindi;
            self.score_of_each_subject.hindi  += difference;
        }
        if self.score_of_each_subject.english < 35 {
            let difference = 35 - self.score_of_each_subject.english;
            self.score_of_each_subject.english += difference;
        }
        if self.score_of_each_subject.maths < 35 {
            let difference = 35 - self.score_of_each_subject.maths;
            self.score_of_each_subject.maths += difference;
        }
        if self.score_of_each_subject.science < 35 {
            let difference = 35 - self.score_of_each_subject.science;
            self.score_of_each_subject.science += difference;
        }
    }
    /// This method  return the average of number
    ///
    /// #Arguments
    ///
    /// Take a Argument as &self
    ///
    /// #Return
    ///
    /// return average of marks
    ///
    pub fn get_average(&self) -> f32 {
        let average: f32 = ((self.score_of_each_subject.science
            + self.score_of_each_subject.maths
            + self.score_of_each_subject.hindi
            + self.score_of_each_subject.english)
            / 4) as f32;
        average
    }
    /// This method  return the Comparison of Student
    ///
    /// #Arguments
    ///
    /// Take a Argument as &self and another Student
    ///
    /// #Return
    ///
    /// printStudentDetail
    ///
    pub fn compare_student(&self, another_student: &Score) {

        println!("For HindiDifference {}", (self.score_of_each_subject.hindi - another_student.hindi).abs());
        println!("For english_difference {}", (self.score_of_each_subject.english - another_student.english).abs());
        println!("For maths_difference {}", (self.score_of_each_subject.maths - another_student.maths).abs());
        println!("For science_difference {}", (self.score_of_each_subject.science - another_student.science).abs())
    }
}
