use ge_course_deepseek::config;
use ge_course_deepseek::game_state;
use ge_course_deepseek::resources;
use raylib::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = config::Config::load("assets/config.toml")?;

    let screen_width = app_config.window.width as i32;
    let screen_height = app_config.window.height as i32;

    let (mut rl, thread) = init()
        .size(screen_width, screen_height)
        .title(app_config.window.title.as_str())
        .build();

    rl.set_target_fps(app_config.game.target_fps);

    let mut texture_manager = resources::TextureManager::new();
    texture_manager.load_texture(
        "assets/img/player.png".as_ref(),
        "player",
        &mut rl,
        &thread,
    )?;

    let mut game_state = game_state::GameState::new(
        Vector2::new(
            (screen_width / 2 - 50) as f32,
            (screen_height / 2 - 50) as f32,
        ),
        app_config.player,
        texture_manager,
    );

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();

        game_state.update(&rl, delta_time);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        game_state.draw(&mut d);
    }

    Ok(())
}
