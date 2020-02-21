use piston_window::*;
use clap::{App, Arg};

mod active;
mod tetris;
mod tetromino;

// cargo run -- -m
fn main() {

    let matches = App::new("rust-tetris")
        .about("Simple Tetris clone written in Rust")
        .version("0.0.1")
        .arg(
            Arg::with_name("initial_stack_size")
                .short("i")
                .long("initial_stack_size")
                .help("Deteremines the number of lines to be filled randomly")
                .multiple(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("music_off")
                .short("o")
                .long("music_off")
                .help("Turns off the music")
                .multiple(false),
        )
        .arg(
            Arg::with_name("mini")
                .short("m")
                .long("mini")
                .help("Minified rendering for screens < 600x800")
                .multiple(false),
        )
        .get_matches();

    let mut initial_stack_size: usize = 0;

    // 设置初始化stack大小
    if let Some(ref stack_size_str) = matches.value_of("initial_stack_size") {
        initial_stack_size = stack_size_str.parse::<usize>().unwrap();
    }

     //
    // 设置大框架还是小框架
    //
    let mini = matches.is_present("mini");
    let (width, height) = ( tetris::WINDOW_WIDTH, tetris::WINDOW_HEIGHT);
    let (width, height) = if mini {
        (width / 2, height / 2)
    } else {
        (width, height)
    };

    // 创建窗口
    let mut window: PistonWindow =
        WindowSettings::new("Tetris", [width, height] )
        .exit_on_esc(true)
        .graphics_api(OpenGL::V3_2)
        .build()
        .unwrap();

    // 只有输入事件才会导致重绘
    // window.set_lazy(true);
    
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let basic_block = assets.join("block.png");

    // 加载底块
    let basic_block: G2dTexture = Texture::from_path(
            &mut window.create_texture_context(),
            &basic_block,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

     //
    // 创建Tetris整个世界创建起来了
    //
    let mut game = tetris::Tetris::new(
        if mini { 0.5 } else { 1.0 },
        &basic_block,
        initial_stack_size,
    );
    
    //
    // 消息循环
    //
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, gl, _| {
            
            //
            // 先把画面清空,
            //
            clear([1.0; 4], gl);

            //
            // WM_PAINT消息,画上所有的更新
            //
            game.render(&c, gl);
        });

        if let Some(uargs) = e.update_args() {
            game.update(&uargs);
        }

        //
        // 键盘按下消息
        //
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_press(&key);
        }

        //
        // 键盘松开消息
        //
        if let Some(Button::Keyboard(key)) = e.release_args() {
            game.key_release(&key);
        }
    }

    /*
    播放音乐
    music::start::<Music, _>(|| {
        music::bind_file(Music::Waves,
                         &(assets.join("airtone_-_gravitationalWaves.mp3")));
        if !music_off {
            music::set_volume(0.2);
            music::play(&Music::Waves, music::Repeat::Forever);
        }
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, gl| {
                clear([1.0; 4], gl);
                game.render(&c, gl);
            });

            if let Some(uargs) = e.update_args() {
                game.update(&uargs);
            }

            if let Some(Button::Keyboard(key)) = e.press_args() {
                game.key_press(&key);
            }

            if let Some(Button::Keyboard(key)) = e.release_args() {
                game.key_release(&key);
            }
        }
    })
    */
}