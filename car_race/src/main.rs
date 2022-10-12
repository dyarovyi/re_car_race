use rusty_engine::prelude::*;
use rand::prelude::*;
use game_state::GameState;

mod game_state;

const STARTUP_HEIGHT: f32 = 300.0;
const STARTUP_WIDTH: f32 = 500.0;

const MOVEMENT_SPEED: f32 = 150.0;
const ROAD_SPEED: f32 = 300.0;

fn main() {
    let mut game = Game::new();
    game.window_settings(WindowDescriptor {
        title: "Race Game".to_string(),
        height: STARTUP_HEIGHT,
        width: STARTUP_WIDTH,
        ..Default::default()
    });

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.scale = 0.7;
    player.translation = Vec2::new(-180.0, 0.0);
    player.collision = true;

    for i in 1..=10 {
        let roadline = game.add_sprite(format!("roadline_{}", i), SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.translation.x = STARTUP_WIDTH / -2.0 + 50.0 * i as f32;
    }

    let obstacle_presets = vec![SpritePreset::RacingBarrelBlue, SpritePreset::RacingBarrierRed, SpritePreset::RacingConeStraight];
    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.scale = 0.7;
        obstacle.translation.x = thread_rng().gen_range(STARTUP_WIDTH..STARTUP_WIDTH * 1.5);
        obstacle.translation.y = thread_rng().gen_range(STARTUP_HEIGHT / -2.0 ..STARTUP_HEIGHT / 2.0);
        obstacle.collision = true;
    }

    let health_label = game.add_text("health_label", "Health: 5".to_string());
    health_label.translation = Vec2::new(STARTUP_WIDTH / 3.0, STARTUP_HEIGHT * 0.4);

    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);
    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {    
    if game_state.lost {
        return;
    }

    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("roadline") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x <= engine.window_dimensions.x / -2.0 {
                sprite.translation.x += engine.window_dimensions.x;
            }
        }
        if sprite.label.starts_with("obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x <= engine.window_dimensions.x / -2.0 {
                sprite.translation.x = thread_rng().gen_range(STARTUP_WIDTH..STARTUP_WIDTH * 1.5);
                sprite.translation.y = thread_rng().gen_range(STARTUP_HEIGHT / -2.0 ..STARTUP_HEIGHT / 2.0);
            }
        }
    }

    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            for label in [event.pair.0, event.pair.1] {
                if label.starts_with("obstacle") {
                    let sprite = engine.sprites.get_mut(&label).unwrap();
                    sprite.translation.x = thread_rng().gen_range(STARTUP_WIDTH..STARTUP_WIDTH * 1.5);
                    sprite.translation.y = thread_rng().gen_range(STARTUP_HEIGHT / -2.0 ..STARTUP_HEIGHT / 2.0);
                }
            }
            let health_label = engine.texts.get_mut("health_label").unwrap();
            game_state.health -= 1;
            health_label.value = format!("Health: {}", game_state.health);
            if game_state.health == 0 {
                game_state.lost = true;
            }
            engine.audio_manager.play_music(SfxPreset::Impact3, 0.5);
        }
    }

    let mut player = engine.sprites.get_mut("player").unwrap();
    player.rotation = 0.0;

    if engine.keyboard_state.pressed_any(&[KeyCode::Up, KeyCode::W]) {
        if player.translation.y <= engine.window_dimensions.y / 2.0 {
            player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
            player.rotation = 0.30;
        }
    } 
    if engine.keyboard_state.pressed_any(&[KeyCode::Down, KeyCode::S]) {
        if player.translation.y >= engine.window_dimensions.y / -2.0 {
            player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
            player.rotation = -0.30;
        }
    } 

    if game_state.lost {
        engine.audio_manager.stop_music();
        engine.audio_manager.play_music(SfxPreset::Jingle3, 0.5);
        let game_over_label = engine.add_text("game_over_label", "Game Over".to_string());
        game_over_label.font_size = 72.0;
    }
}