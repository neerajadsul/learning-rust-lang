

slint::slint! {
     component MemoryTile inherits Rectangle {
        callback clicked;
        in property <bool> open_curtain;
        in property <bool> solved;
        in property <image> icon;

        width: 64px;
        height: 64px;
        background: solved ? green : gray;
        animate background { duration: 1000ms; }


        Image {
            source: icon;
            width: parent.width;
            height: parent.height;
        }

        // Left curtain
        Rectangle {
            background: #193076;
            x: 0px;
            width: open-curtain ? 0px : (parent.width / 2);
            height: parent.height;
            animate width { duration: 300ms; easing: ease-in; }
        }

        // Right curtain
        Rectangle {
            background: #193076;
            x: open-curtain ? parent.width : (parent.width / 2);
            width: open-curtain ? 0px : (parent.width / 2);
            height: parent.height;
            animate width { duration: 300ms; easing: ease-in; }
            animate x { duration: 300ms; easing: ease-in; }
        }

        TouchArea {
            clicked => {
                root.clicked();
            }
        }

     } 
    export component MainWindow inherits Window {
        MemoryTile {
            icon: @image-url("icons/bicycle.png");
            clicked => {
                self.open-curtain = !self.open-curtain;
            }
        }
     }

}

fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

