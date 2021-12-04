trait SalariedEmployee {}

#[derive(Clone, Copy)]
enum DoctorSpecialty {
    Pediatrician,
    Obstetrician,
    GeneralPractitioner,
}

#[derive(Clone)]
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

fn main() {
    println!("Hello, world!");
}
