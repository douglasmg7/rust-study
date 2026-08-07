fn main() {
    struct DentistApoiment {
        doctor: String,
    }

    impl DentistApoiment {
        // lifetime third rule define that returned reference must be self.
        // If return reference come from other than self, lifetime must be provided.
        fn book(&self, check_in_time: &str, check_out_time: &str) -> &str {
            println!(
                "You a booked from {} to {} with doctor {}.",
                check_in_time, check_out_time, self.doctor
            );
            &self.doctor
        }

        // lifetime must be used because the returned ref is not self.
        fn book_start<'a>(&self, check_in_time: &'a str, check_out_time: &str) -> &'a str {
            println!(
                "You a booked from {} to {} with doctor {}.",
                check_in_time, check_out_time, self.doctor
            );
            check_in_time
        }
    }

    let appt = DentistApoiment {
        doctor: "Eloisio".to_string(),
    };

    let doctor = appt.book("03:00PM", "04:00PM");
    let start = appt.book_start("03:00PM", "04:00PM");
    println!("Doctor: {:?}", doctor);
    println!("Start: {:?}", start);
}
