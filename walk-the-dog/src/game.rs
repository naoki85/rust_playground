mod red_hat_boy_states;
mod walk_the_dog_state_machine;

use std::rc::Rc;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use rand::prelude::*;
use red_hat_boy_states::*;
use walk_the_dog_state_machine::*;
use web_sys::HtmlImageElement;

use crate::{
    browser,
    engine::{self, Audio, Cell, Game, Image, KeyState, Point, Rect, Renderer, Sheet, Sound, SpriteSheet},
    segments::*,
};

const HEIGHT: i16 = 600;
const OBSTACLE_BUFFER: i16 = 20;

pub struct Walk {
    obstacle_sheet: Rc<SpriteSheet>,
    boy: RedHatBoy,
    backgrounds: [Image; 2],
    obstacles: Vec<Box<dyn Obstacle>>,
    stone: HtmlImageElement,
    timeline: i16,
}

impl Walk {
    fn velocity(&self) -> i16 {
        -self.boy.walking_speed()
    }

    fn generate_next_segment(&mut self) {
        let next_segment = rand::rng().random_range(0..2);

        let mut next_obstacles = match next_segment {
            0 => stone_and_platform(
            self.stone.clone(),
            self.obstacle_sheet.clone(),
            self.timeline + OBSTACLE_BUFFER,
            ),
            1 => platform_and_stone(
            self.stone.clone(),
            self.obstacle_sheet.clone(),
            self.timeline + OBSTACLE_BUFFER,
            ),
            _ => vec![],
        };

        self.timeline = rightmost(&next_obstacles);
        self.obstacles.append(&mut next_obstacles);
    }

    fn draw(&self, renderer: &Renderer) {
        self.backgrounds.iter().for_each(|background| {
            background.draw(renderer);
        });
        self.boy.draw(renderer);
        self.obstacles.iter().for_each(|obstacle| {
            obstacle.draw(renderer);
        });
    }

    fn knocked_out(&self) -> bool {
        self.boy.knocked_out()
    }

    fn reset(walk: Self) -> Self {
        let starting_obstacles = stone_and_platform(
            walk.stone.clone(),
            walk.obstacle_sheet.clone(),
            0,
        );
        let timeline = rightmost(&starting_obstacles);

        Walk {
            boy: RedHatBoy::reset(walk.boy),
            backgrounds: walk.backgrounds,
            obstacles: starting_obstacles,
            obstacle_sheet: walk.obstacle_sheet,
            stone: walk.stone,
            timeline,
        }
    }
}

pub struct WalkTheDog {
    machine: Option<WalkTheDogStateMachine>,
}

impl WalkTheDog {
    pub fn new() -> Self {
        Self { machine: None }
    }
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        match self.machine{
            None => {
                let background = engine::load_image("BG.png").await?;
                let stone = engine::load_image("Stone.png").await?;

                let platform_sheet = browser::fetch_json("tiles.json").await?;
                let platform_sheet_serde: Option<Sheet> = serde_wasm_bindgen::from_value(platform_sheet)
                    .expect("Could not convert tiles.json into a Sheet structure");
                let sprite_sheet = Rc::new(SpriteSheet::new(
                    platform_sheet_serde.clone().ok_or_else(|| anyhow!("No Sheet Present"))?,
                    engine::load_image("tiles.png").await?,
                ));

                let json = browser::fetch_json("rhb.json").await?;
                let sheet: Option<Sheet> = serde_wasm_bindgen::from_value(json)
                    .expect("Could not convert rhb.json into a Sheet structure");
                let image = Some(engine::load_image("rhb.png").await?);

                let audio = Audio::new()?;
                let sound = audio.load_sound("SFX_Jump_23.mp3").await?;
                let background_music = audio.load_sound("background_song.mp3").await?;
                audio.play_looping_sound(&background_music)?;

                let rhb = RedHatBoy::new(
                    sheet.clone().ok_or_else(|| anyhow!("No Sheet Present"))?,
                    image.clone().ok_or_else(|| anyhow!("No Image Present"))?,
                    audio,
                    sound,
                );
                let background_width = background.width() as i16;

                let starting_obstacles = stone_and_platform(
                    stone.clone(),
                    sprite_sheet.clone(),
                    0,
                );
                let timeline = rightmost(&starting_obstacles);

                let machine = WalkTheDogStateMachine::new(Walk {
                    boy: rhb,
                    backgrounds: [
                        Image::new(background.clone(), Point { x: 0, y: 0 }),
                        Image::new(background, Point { x: background_width, y: 0 }),
                    ],
                    stone,
                    obstacles: starting_obstacles,
                    timeline,
                    obstacle_sheet: sprite_sheet,
                });

                Ok(Box::new(WalkTheDog { machine: Some(machine) }))
            }
            Some(_) => Err(anyhow!("Error: Game is already initialized!")),
        }
    }

    fn update(&mut self, keystate: &KeyState) {
        if let Some(machine) = self.machine.take() {
            self.machine.replace(machine.update(keystate));
        }
        assert!(self.machine.is_some());
    }

    fn draw(&self, renderer: &Renderer) {
        renderer.clear(&Rect::new_from_x_y(0, 0, 600, HEIGHT));
        if let Some(machine) = &self.machine {
            machine.draw(renderer);
        }
    }
}

#[derive(Clone)]
pub enum RedHatBoyStateMachine {
    Idle(RedHatBoyState<Idle>),
    Running(RedHatBoyState<Running>),
    Sliding(RedHatBoyState<Sliding>),
    Jumping(RedHatBoyState<Jumping>),
    Falling(RedHatBoyState<Falling>),
    KnockedOut(RedHatBoyState<KnockedOut>),
}

impl From<RedHatBoyState<Idle>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Idle>) -> Self {
        RedHatBoyStateMachine::Idle(state)
    }
}

impl From<RedHatBoyState<Running>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Running>) -> Self {
        RedHatBoyStateMachine::Running(state)
    }
}

impl From<RedHatBoyState<Sliding>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Sliding>) -> Self {
        RedHatBoyStateMachine::Sliding(state)
    }
}

impl From<RedHatBoyState<Jumping>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Jumping>) -> Self {
        RedHatBoyStateMachine::Jumping(state)
    }
}

impl From<RedHatBoyState<Falling>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Falling>) -> Self {
        RedHatBoyStateMachine::Falling(state)
    }
}

impl From<RedHatBoyState<KnockedOut>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<KnockedOut>) -> Self {
        RedHatBoyStateMachine::KnockedOut(state)
    }
}

impl From<SlidingEndState> for RedHatBoyStateMachine {
    fn from(state: SlidingEndState) -> Self {
        match state {
            SlidingEndState::Sliding(sliding) => sliding.into(),
            SlidingEndState::Running(running) => running.into(),
        }
    }
}

impl From<JumpingEndState> for RedHatBoyStateMachine {
    fn from(state: JumpingEndState) -> Self {
        match state {
            JumpingEndState::Jumping(jumping) => jumping.into(),
            JumpingEndState::Landing(landing) => landing.into(),
        }
    }
}

impl From<FallingEndState> for RedHatBoyStateMachine {
    fn from(state: FallingEndState) -> Self {
        match state {
            FallingEndState::Falling(falling) => falling.into(),
            FallingEndState::KnockedOut(knocked_out) => knocked_out.into(),
        }
    }
}

pub enum Event {
    Run,
    Slide,
    Update,
    Jump,
    KnockOut,
    Land(i16),
}

impl RedHatBoyStateMachine {
    fn transition(self, event: Event) -> Self {
        match (self.clone(), event) {
            (RedHatBoyStateMachine::Idle(state), Event::Run) => state.run().into(),
            (RedHatBoyStateMachine::Running(state), Event::Slide) => state.slide().into(),
            (RedHatBoyStateMachine::Idle(state), Event::Update) => state.update().into(),
            (RedHatBoyStateMachine::Running(state), Event::Update) => state.update().into(),
            (RedHatBoyStateMachine::Sliding(state), Event::Update) => state.update().into(),
            (RedHatBoyStateMachine::Running(state), Event::Jump) => state.jump().into(),
            (RedHatBoyStateMachine::Jumping(state), Event::Update) => state.update().into(),
            (RedHatBoyStateMachine::Running(state), Event::KnockOut) => state.knock_out().into(),
            (RedHatBoyStateMachine::Jumping(state), Event::KnockOut) => state.knock_out().into(),
            (RedHatBoyStateMachine::Sliding(state), Event::KnockOut) => state.knock_out().into(),
            (RedHatBoyStateMachine::Falling(state), Event::Update) => state.update().into(),
            (RedHatBoyStateMachine::Running(state), Event::Land(position)) => {
                state.land_on(position).into()
            },
            (RedHatBoyStateMachine::Jumping(state), Event::Land(position)) =>  {
                state.land_on(position).into()
            },
            (RedHatBoyStateMachine::Sliding(state), Event::Land(position)) => {
                state.land_on(position).into()
            }, 
            _ => self,
        }
    }

    fn frame_name(&self) -> &str {
        match self {
            RedHatBoyStateMachine::Idle(state) => &state.frame_name(),
            RedHatBoyStateMachine::Running(state) => &state.frame_name(),
            RedHatBoyStateMachine::Sliding(state) => &state.frame_name(),
            RedHatBoyStateMachine::Jumping(state) => &state.frame_name(),
            RedHatBoyStateMachine::Falling(state) => &state.frame_name(),
            RedHatBoyStateMachine::KnockedOut(state) => &state.frame_name(),
        }
    }

    fn context(&self) -> &RedHatBoyContext {
        match self {
            RedHatBoyStateMachine::Idle(state) => &state.context(),
            RedHatBoyStateMachine::Running(state) => &state.context(),
            RedHatBoyStateMachine::Sliding(state) => &state.context(),
            RedHatBoyStateMachine::Jumping(state) => &state.context(),
            RedHatBoyStateMachine::Falling(state) => &state.context(),
            RedHatBoyStateMachine::KnockedOut(state) => &state.context(),
        }
    }

    fn update(self) -> Self {
        self.transition(Event::Update)
    }

    fn knocked_out(&self) -> bool {
        matches!(self, RedHatBoyStateMachine::KnockedOut(_))
    }
}

pub struct RedHatBoy {
    state_machine: RedHatBoyStateMachine,
    sprite_sheet: Sheet,
    image: HtmlImageElement,
}

impl RedHatBoy {
    fn new(sheet: Sheet, image: HtmlImageElement, audio: Audio, sound: Sound) -> Self {
        RedHatBoy {
            state_machine: RedHatBoyStateMachine::Idle(RedHatBoyState::new(audio, sound)),
            sprite_sheet: sheet,
            image,
        }
    }

    fn draw(&self, renderer: &Renderer) {
        let sprite = self.current_sprite().expect("Cell not found");

        renderer.draw_image(
            &self.image,
            &Rect::new_from_x_y(
                sprite.frame.x,
                sprite.frame.y,
                sprite.frame.w,
                sprite.frame.h,
        ),
            &self.destination_box(),
        );
    }

    fn update(&mut self) {
        self.state_machine = self.state_machine.clone().update();
    }

    fn run_right(&mut self) {
        self.state_machine = self.state_machine.clone().transition(Event::Run);
    }

    fn slide(&mut self) {
        self.state_machine = self.state_machine.clone().transition(Event::Slide);
    }

    fn jump(&mut self) {
        self.state_machine = self.state_machine.clone().transition(Event::Jump);
    }

    fn land_on(&mut self, position: i16) {
        self.state_machine = self.state_machine.clone().transition(Event::Land(position));
    }

    fn pos_y(&self) -> i16 {
        self.state_machine.context().position.y
    }

    fn velocity_y(&self) -> i16 {
        self.state_machine.context().velocity.y
    }

    fn frame_name(&self) -> String {
        format!(
            "{} ({}).png",
            self.state_machine.frame_name(),
            (self.state_machine.context().frame / 3) + 1
        )
    }

    fn current_sprite(&self) -> Option<&Cell> {
        self.sprite_sheet
            .frames
            .get(&self.frame_name())
    }

    fn bounding_box(&self) -> Rect {
        const X_OFFSET: i16 = 18;
        const Y_OFFSET: i16 = 14;
        const WIDTH_OFFSET: i16 = 28;
        Rect::new_from_x_y(
            self.destination_box().x() + X_OFFSET,
            self.destination_box().y() + Y_OFFSET,
            self.destination_box().width - WIDTH_OFFSET,
            self.destination_box().height - Y_OFFSET)
    }

    fn destination_box(&self) -> Rect {
        let sprite = self.current_sprite().expect("Cell not found");

        Rect::new_from_x_y(
            self.state_machine.context().position.x + sprite.sprite_source_size.x,
            self.state_machine.context().position.y + sprite.sprite_source_size.y,
            sprite.frame.w,
            sprite.frame.h,
        )
    }

    fn knock_out(&mut self) {
        self.state_machine = self.state_machine.clone().transition(Event::KnockOut);
    }

    fn knocked_out(&self) -> bool {
        self.state_machine.knocked_out()
    }

    fn walking_speed(&self) -> i16 {
        self.state_machine.context().velocity.x
    }

    fn reset(boy: Self) -> Self {
        RedHatBoy::new(
            boy.sprite_sheet, boy.image, boy.state_machine.context().audio.clone(),
            boy.state_machine.context().jump_sound.clone(),
        )
    }
} 

pub struct Platform {
    sheet: Rc<SpriteSheet>,
    bounding_boxes: Vec<Rect>,
    sprites: Vec<Cell>,
    position: Point,
}

impl Platform {
    pub fn new(sheet: Rc<SpriteSheet>, position: Point, sprite_names: &[&str], bounding_boxes: &[Rect]) -> Self {
        let sprites = sprite_names.iter().filter_map(|sprite_name| sheet.cell(sprite_name).cloned()).collect();
        let bounding_boxes = bounding_boxes.iter().map(|bounding_box| {
            Rect::new_from_x_y(
                bounding_box.x() + position.x,
                bounding_box.y() + position.y,
                bounding_box.width,
                bounding_box.height,
            )
        }).collect();

        Platform {
            sheet,
            position,
            sprites,
            bounding_boxes,
        }
    }

    fn bounding_boxes(&self) -> &Vec<Rect> {
        &self.bounding_boxes
    }
}

pub trait Obstacle {
    fn check_intersection(&self, boy: &mut RedHatBoy);
    fn move_horizontally(&mut self, x: i16);
    fn draw(&self, renderer: &Renderer);
    fn right(&self) -> i16;
}

impl Obstacle for Platform {
    fn draw(&self, renderer: &Renderer) {
        let mut x = 0;
        self.sprites.iter().for_each(|sprite| {
            self.sheet.draw(
                renderer,
                &Rect::new_from_x_y(
                    sprite.frame.x,
                    sprite.frame.y,
                    sprite.frame.w,
                    sprite.frame.h,
                ),
                &Rect::new_from_x_y(
                    self.position.x + x,
                    self.position.y,
                    sprite.frame.w,
                    sprite.frame.h,
                ),
            );
            x += sprite.frame.w;
        });
    }

    fn move_horizontally(&mut self, x: i16) {
        self.position.x += x;
        self.bounding_boxes.iter_mut().for_each(|bounding_box| {
            bounding_box.set_x(bounding_box.position.x + x);
        });
    }

    fn check_intersection(&self, boy: &mut RedHatBoy) {
        if let Some(box_to_land_on) = self
        .bounding_boxes()
        .iter()
        .find(|&bounding_box| boy.bounding_box().intersects(bounding_box)) {
            if boy.velocity_y() > 0 && boy.pos_y() < self.position.y {
                boy.land_on(box_to_land_on.y());
            } else {
                boy.knock_out();
            }
        }
    }

    fn right(&self) -> i16 {
        self.bounding_boxes().last().unwrap_or(&Rect::default()).right()
    }
}
pub struct Barrier {
    image: Image,
}

impl Barrier {
    pub fn new(image: Image) -> Self {
        Barrier {
            image, 
        }
    }
}

impl Obstacle for Barrier {
    fn check_intersection(&self, boy: &mut RedHatBoy) {
        if boy.bounding_box().intersects(self.image.bounding_box()) {
            boy.knock_out();
        }
    }

    fn draw(&self, renderer: &Renderer) {
        self.image.draw(renderer);
    }

    fn move_horizontally(&mut self, x: i16) {
        self.image.move_horizontally(x);
    }

    fn right(&self) -> i16 {
        self.image.right()
    }
}

fn rightmost(obstacle_list: &[Box<dyn Obstacle>]) -> i16 {
    obstacle_list
        .iter()
        .map(|obstacle| obstacle.right())
        .max_by(|x, y| x.cmp(&y))
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::channel::mpsc::unbounded;
    use std::collections::HashMap;
    use web_sys::{AudioBuffer, AudioBufferOptions};
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_transition_from_game_over_to_new_game() {
        let (_, receiver) = unbounded();
        let image = HtmlImageElement::new().unwrap();
        let audio = Audio::new().unwrap();
        let options = AudioBufferOptions::new(1, 3000.0);
        let sound = Sound {
            buffer: AudioBuffer::new(&options).unwrap(),
        };
        let rhb = RedHatBoy::new(
            Sheet {
                frames: HashMap::new(),
            },
            image.clone(),
            audio,
            sound,
        );
        let sprite_sheet = SpriteSheet::new(
            Sheet {
                frames: HashMap::new(),
            },
            image.clone(),
        );
        let walk = Walk {
            boy: rhb,
            backgrounds: [
                Image::new(image.clone(), Point { x: 0, y: 0 }),
                Image::new(image.clone(), Point { x: 0, y: 0 }),
            ],
            obstacles: vec![],
            obstacle_sheet: Rc::new(sprite_sheet),
            stone: image.clone(),
            timeline: 0,
        };
        let document = browser::document().unwrap();
        document.body().unwrap().insert_adjacent_html("afterbegin", "<div id='ui'></div>").unwrap();
        browser::draw_ui("<p>This is the UI</p>").unwrap();
        let state = WalkTheDogState {
            _state: GameOver { new_game_event: receiver },
            walk,
        };

        state.new_game();
        let ui = browser::find_html_element_by_id("ui").unwrap();
        assert_eq!(ui.child_element_count(), 0);
    }
}