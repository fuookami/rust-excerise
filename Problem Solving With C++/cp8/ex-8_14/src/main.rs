fn split_impl<'a>(ret: &mut Vec<&'a str>, context: &'a str, delimiter: &str) {
    match context.find(delimiter) {
        Some(i) => {
            ret.push(context.get(0..i).unwrap());
            split_impl(
                ret,
                context.get((i + delimiter.len())..).unwrap(),
                delimiter,
            );
        }
        None => {}
    };
}

fn split<'a>(context: &'a str, delimiter: &str) -> Vec<&'a str> {
    let mut ret = Vec::new();
    split_impl(&mut ret, context, delimiter);
    ret
}

fn main() {
    let parts = split("10,20,30", ",");
    for part in parts {
        print!("{},", part);
    }
    print!("\n");

    let parts = split("do re mi fa so la ti do", " ");
    for part in parts {
        print!("{},", part);
    }
    print!("\n");
}
