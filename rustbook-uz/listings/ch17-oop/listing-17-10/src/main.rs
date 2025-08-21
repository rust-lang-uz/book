use gui::Screen;

fn main() {
    let screen = Screen {
        components: vec![Box::new(String::from("Salom"))],
    };

    screen.run();
}
