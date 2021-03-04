use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn write_file(path: &str, context: &str) {
    let path = Path::new(path);
    let display = path.display();

    let mut fout = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(fout) => fout,
    };

    match fout.write(context.as_bytes()) {
        Err(why) => panic!("couldn't write {}: {}", display, why),
        Ok(_) => {}
    };
}

fn read_values() -> Vec<u64> {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line!");
    let mut ret = Vec::<u64>::new();
    for s in line.trim().split_ascii_whitespace() {
        ret.push(match s.parse::<u64>() {
            Result::Ok(value) => value,
            Result::Err(_) => panic!("Failed to parse!"),
        })
    }
    ret.sort();
    ret
}

fn to_svg(values: &Vec<u64>) -> String {
    let mut ret = String::new();
    ret.push_str("?<xml version=\"1.0\" standalone=\"no\"?>\n");
    ret.push_str("<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n");
    ret.push_str(
        "<svg width=\"500\" height=\"450\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\">\n",
    );
    ret.push_str(
        "\t<line x1=\"0\" y1=\"400\" x2=\"500\" y2=\"400\" stroke=\"black\" stroke-width=\"1\" />\n",
    );
    ret.push_str(
        "\t<line x1=\"0\" y1=\"0\" x2=\"0\" y2=\"400\" stroke=\"black\" stroke-width=\"2\" />\n",
    );
    let width = (500. / values.len() as f64 / 2.) as u64;
    for i in 0..values.len() {
        let height = (values[i] as f64 / *values.last().unwrap() as f64 * 400.) as u64;
        ret.push_str(&format!(
            "\t<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" file=\"black\" />\n",
            (i * 2 + 1) as u64 * width,
            400 - height,
            width,
            height
        ));
    }
    ret.push_str("</svg>\n");
    ret
}

fn main() {
    println!("Enter values: ");
    let values = read_values();
    write_file("output.svg", &to_svg(&values));
}
