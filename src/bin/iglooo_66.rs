use std::fs;

struct LineWriter<'a> {
    holds: &'a mut String,
}

impl<'a> LineWriter<'a> {
    fn push_lines(&mut self, someline: &str) {
        self.holds.push_str(someline);
        self.holds.push('\n');
    }
}

fn insert_footer(footer_input: &mut String) -> &mut String {
    footer_input.push_str("\n---8 lines---");
    footer_input
}

fn render(input: &mut String, path: &str) {
    let mut some_struct = LineWriter { holds: input };
    some_struct.push_lines("=== server.log ===");
    let lines = fs::read_to_string(path).ok().unwrap();

    for i in lines.lines() {
        some_struct.push_lines(i);
    }

    // we can use input
    insert_footer(input);
}

fn main() {
    let mut some_string = String::from("");
    render(&mut some_string, "server.log");
}
