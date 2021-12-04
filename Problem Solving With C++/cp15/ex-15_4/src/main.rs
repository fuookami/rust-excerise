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
    fn new(specialty: DoctorSpecialty, office_visit_fee: f64) -> Self {
        Doctor {
            specialty: specialty,
            office_visit_fee: office_visit_fee,
        }
    }
}

trait Person {
    fn name(&self) -> &str;
}

#[derive(Clone)]
struct Patient<'a> {
    _name: String,
    _visiting_staff: &'a Doctor,
}

impl<'a> Person for Patient<'a> {
    fn name(&self) -> &str {
        &self._name
    }
}

impl<'a> Patient<'a> {
    fn new(name: String, visiting_staff: &'a Doctor) -> Self {
        Patient {
            _name: name,
            _visiting_staff: visiting_staff,
        }
    }
}

#[derive(Clone)]
struct Billing<'a, 'b> {
    patient: &'a Patient<'b>,
    visiting_staff: &'b Doctor,
    fee: f64,
}

impl<'a, 'b> Billing<'a, 'b> {
    fn new(patient: &'a Patient<'b>, visiting_staff: &'b Doctor, fee: f64) -> Self {
        Billing {
            patient: patient,
            visiting_staff: visiting_staff,
            fee: fee,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
