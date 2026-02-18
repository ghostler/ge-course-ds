use ge_course_deepseek::game_play_state;
use ge_course_deepseek::main_menu_state;
use ge_course_deepseek::resource;
use ge_course_deepseek::state::State;
use ge_course_deepseek::{config, pause_state, state};
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
    rl.set_exit_key(None);

    let mut assets = resource::Assets::new();
    assets.texture_manager_mut().load_texture(
        "assets/img/player.png".as_ref(),
        "player",
        &mut rl,
        &thread,
    )?;

    let player_config = app_config.player.clone();
    let pause_state_builder: fn() -> Box<dyn State> = || Box::new(pause_state::PauseState::new());
    let state_builder: Box<dyn Fn() -> Box<dyn State>> = Box::new(move || {
        Box::new(game_play_state::GamePlayState::new(
            player_config.clone(),
            pause_state_builder,
        ))
    });
    let initial_state: Box<dyn State> =
        Box::new(main_menu_state::MainMenuState::new(state_builder));
    let mut state_stack = state::StateStack::new(initial_state);

    while !rl.window_should_close() && state_stack.has_state() {
        let delta_time = rl.get_frame_time();

        state_stack.update(&rl, delta_time);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        state_stack.draw(&mut d, &assets);
    }

    Ok(())
}
