trait SalariedEmployee {}

#[derive(Clone, Copy)]
enum DoctorSpecialty {
    Pediatrician,
    Obstetrician,
    GeneralPractitioner,
}

struct Doctor {
    specialty: DoctorSpecialty,
    office_visit_fee: f64,
}

impl SalariedEmployee for Doctor {}

impl Doctor {
    fn new(specialty: DoctorSpecialty, office_visit_fee: f64) -> Doctor {
        Doctor {
            specialty: specialty,
            office_visit_fee: office_visit_fee,
        }
    }
}

impl Clone for Doctor {
    fn clone(&self) -> Self {
        Doctor {
            specialty: self.specialty,
            office_visit_fee: self.office_visit_fee,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
