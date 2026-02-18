use crate::{config, context, game_play_state, main_menu_state, pause_state, resource, state};
use raylib::prelude::*;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = config::Config::load("assets/config.toml")?;

    let screen_width = app_config.window.width as i32;
    let screen_height = app_config.window.height as i32;

    let (mut rl, thread) = init()
        .size(screen_width, screen_height)
        .title(app_config.window.title.as_str())
        .build();

    rl.set_target_fps(app_config.game.target_fps);
    rl.set_exit_key(None);

    let input_config = crate::input::InputConfig::load("assets/input.toml")?;
    let input_map = crate::input::InputMap::new(input_config);

    let mut game_ctx = context::GameCtx::new(&mut rl, &thread, &input_map);

    let mut assets = resource::Assets::new();
    assets.texture_manager_mut().load_texture(
        "assets/img/player.png".as_ref(),
        "player",
        game_ctx.rl_mut(),
        &thread,
    )?;

    let player_config = app_config.player.clone();
    let pause_state_builder: fn() -> Box<dyn state::State> =
        || Box::new(pause_state::PauseState::new());
    let state_builder: Box<dyn Fn() -> Box<dyn state::State>> = Box::new(move || {
        Box::new(game_play_state::GamePlayState::new(
            player_config.clone(),
            pause_state_builder,
        ))
    });
    let initial_state: Box<dyn state::State> =
        Box::new(main_menu_state::MainMenuState::new(state_builder));
    let mut state_stack = state::StateStack::new(initial_state);

    while !game_ctx.rl().window_should_close() && state_stack.has_state() {
        game_ctx.load_delta_time();

        state_stack.update(&game_ctx);

        let mut d = game_ctx.begin_drawing();
        d.clear_background(Color::BLACK);

        state_stack.draw(&mut d, &assets);
    }

    Ok(())
}
