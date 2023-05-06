mod player;
mod keyboard;
mod mouse;
mod jump;
mod vector;

use fyrox::core::pool::Handle;
use fyrox::engine::Engine;
use fyrox::resource::model::ModelResourceExtension;
use fyrox::scene::Scene;
use crate::player::Player;

pub struct Game {
    scene: Handle<Scene>,
    pub player: Player,
}

impl Game {
    pub async fn new(engine: &mut Engine) -> Self {
        let mut scene = Scene::new();

        engine
            .resource_manager
            .request("data/scene.rgs")
            .await
            .unwrap()
            .instantiate(&mut scene);
        Self {
            player: Player::new(&mut scene),
            scene: engine.scenes.add(scene)
        }
    }

    pub fn update(&mut self, engine: &mut Engine) {
        self.player.update_player(&mut engine.scenes[self.scene]);
    }
}
