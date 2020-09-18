mod tests{

    struct Student{
        first_name:String,
        last_name:String,
        age:u8,
    }
    impl Student{
        fn new(first_name:&str,last_name:&str,age:u8)->Self{
            Self{
                first_name:first_name.to_string(),
                last_name:last_name.to_string(),
                age,
            }
        }
    }

    use super::{is_power_of_two,sort,sort_by};
    use crate::SortOrder::*;

    #[test]
    fn sort_students_by_age_ascending(){
        let mike=Student::new("Misaki","Akeno",17);
        let shiro=Student::new("Mashiro","Muneya",17);
        let tama=Student::new("Shima","Tateishi",17);
        let mei=Student::new("Mei","Irizaki",16);
        let rin=Student::new("Rin","Shiretoko",16);
    }
}