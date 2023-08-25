
slint::slint! {
    export component MainWindow inherits Window {
        height: 200px;
        width: 300px;
        Text {
            text: "Hello Slint UI!";
            color: blue;
        }
     }
}

fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

