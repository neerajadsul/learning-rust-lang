

slint::slint! {

    struct TileData {
        image: image,
        image_visible: bool,
        solved: bool,
    }

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
        width: 296px;
        height: 148px;

        in property <[TileData]> memory_tiles: [
            {image: @image-url("icons/bicycle.png")},
            {image: @image-url("icons/at.png")},
            {image: @image-url("icons/balance-scale.png")},
            {image: @image-url("icons/bus.png")},
            {image: @image-url("icons/cloud.png")},
            {image: @image-url("icons/cogs.png")},
            {image: @image-url("icons/motorcycle.png")},
            {image: @image-url("icons/video.png")},
        ];

        for tile[i] in memory-tiles: MemoryTile {
            x: mod(i, 4) * 74px;
            y: floor(i / 4) * 74px;
            width: 64px;
            height: 64px;
            icon: tile.image;
            open-curtain: tile.image-visible || tile.solved;
            solved: tile.solved;
            clicked => {
                tile.image-visible = !tile.image-visible;
            }
        }
     }

}

fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

