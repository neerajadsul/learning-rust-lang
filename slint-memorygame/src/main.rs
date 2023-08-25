

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
        width: 374px;
        height: 374px;

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
    use slint::Model;

    let main_window = MainWindow::new().unwrap();

    // Fetch the tiles from the model above in .slint macros
    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().collect();
    // Duplicating the tiles to create pairs
    tiles.extend(tiles.clone());

    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));
    main_window.set_memory_tiles(tiles_model.into());

    main_window.run().unwrap();
}

