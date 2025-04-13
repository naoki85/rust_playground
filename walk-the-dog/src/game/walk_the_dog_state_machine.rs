use futures::channel::mpsc::UnboundedReceiver;

use crate::{browser, engine::{self, KeyState, Renderer}};

use super::Walk;

const TIMELINE_MINIMUM: i16 = 1000;

pub enum WalkTheDogStateMachine {
    Ready(WalkTheDogState<Ready>),
    Walking(WalkTheDogState<Walking>),
    GameOver(WalkTheDogState<GameOver>),
}

impl WalkTheDogStateMachine {
    pub fn new(walk: Walk) -> Self {
        WalkTheDogStateMachine::Ready(WalkTheDogState::new(walk))
    }

    pub fn update(self, keystate: &KeyState) -> Self {
        match self {
            WalkTheDogStateMachine::Ready(state) => state.update(keystate).into(),
            WalkTheDogStateMachine::Walking(state) => state.update(keystate).into(),
            WalkTheDogStateMachine::GameOver(state) => state.update().into(),
        }
    }

    pub fn draw(&self, renderer: &Renderer) {
        match self {
            WalkTheDogStateMachine::Ready(state) => state.draw(renderer),
            WalkTheDogStateMachine::Walking(state) => state.draw(renderer),
            WalkTheDogStateMachine::GameOver(state) => state.draw(renderer),
        }
    }
}

pub struct WalkTheDogState<T> {
    pub _state: T,
    pub walk: Walk,
}

impl<T> WalkTheDogState<T> {
    pub fn draw(&self, renderer: &Renderer) {
        self.walk.draw(renderer);
    }
}

pub struct Ready;
pub struct Walking;
pub struct GameOver {
    pub new_game_event: UnboundedReceiver<()>,
}

impl GameOver {
    fn new_game_pressed(&mut self) -> bool {
        matches!(self.new_game_event.try_next(), Ok(Some(())))
    }
}

impl WalkTheDogState<Ready> {
    fn new(walk: Walk) -> Self {
        WalkTheDogState {
            _state: Ready,
            walk,
        }
    }

    fn update(mut self, keystate: &KeyState) -> ReadyEndState {
        self.walk.boy.update();
        if keystate.is_pressed("ArrowRight") {
            ReadyEndState::Complete(self.start_running())
        } else {
            ReadyEndState::Continue(self)
        }
    }

    fn start_running(mut self) -> WalkTheDogState<Walking> {
        self.run_right();
        WalkTheDogState {
            _state: Walking,
            walk: self.walk,
        }
    }

    fn run_right(&mut self) {
        self.walk.boy.run_right();
    }
}

impl From<WalkTheDogState<Ready>> for WalkTheDogStateMachine {
    fn from(state: WalkTheDogState<Ready>) -> Self {
        WalkTheDogStateMachine::Ready(state)
    }
}

impl WalkTheDogState<Walking> {
    fn update(mut self, keystate: &KeyState) -> WalkingEndState {
        if keystate.is_pressed("ArrowDown") {
            self.walk.boy.slide();
        }

        if keystate.is_pressed("Space") {
            self.walk.boy.jump();
        }

        self.walk.boy.update();

        let walking_speed = self.walk.velocity();
        let [first_background, second_background] = &mut self.walk.backgrounds;
        first_background.move_horizontally(walking_speed);
        second_background.move_horizontally(walking_speed);

        if first_background.right() < 0 {
            first_background.set_x(second_background.right());
        }

        if second_background.right() < 0 {
            second_background.set_x(first_background.right());
        }

        self.walk.obstacles.retain(|obstacle| obstacle.right() > 0);

        self.walk.obstacles.iter_mut().for_each(|obstacle| {
            obstacle.move_horizontally(walking_speed);
            obstacle.check_intersection(&mut self.walk.boy);
        });

        if self.walk.timeline < TIMELINE_MINIMUM {
            self.walk.generate_next_segment();
        } else {
            self.walk.timeline += walking_speed;
        }

        if self.walk.knocked_out() {
            WalkingEndState::Complete(self.end_game())
        } else {
            WalkingEndState::Continue(self)
        }
    }

    fn end_game(self) -> WalkTheDogState<GameOver> {
        let receiver = browser::draw_ui("<button id='new_game'>New Game</button>")
        .and_then(|_unit| browser::find_html_element_by_id("new_game"))
        .map(|element| engine::add_click_handler(element))
        .unwrap();

        WalkTheDogState {
            _state: GameOver {new_game_event: receiver},
            walk: self.walk,
        }
    }
}

impl WalkTheDogState<GameOver> {
    fn update(mut self) -> GameOverEndState {
        if self._state.new_game_pressed() {
            GameOverEndState::Complete(self.new_game())
        } else {
            GameOverEndState::Continue(self)
        }
    }

    pub fn new_game(self) -> WalkTheDogState<Ready> {
        if let Err(err) = browser::hide_ui() {
            error!("Error hiding the browser {:#?}", err);
        }

        WalkTheDogState {
            _state: Ready,
            walk: Walk::reset(self.walk),
        }
    }
}

impl From<WalkTheDogState<Walking>> for WalkTheDogStateMachine {
    fn from(state: WalkTheDogState<Walking>) -> Self {
        WalkTheDogStateMachine::Walking(state)
    }
}

impl From<WalkTheDogState<GameOver>> for WalkTheDogStateMachine {
    fn from(state: WalkTheDogState<GameOver>) -> Self {
        WalkTheDogStateMachine::GameOver(state)
    }
}

enum ReadyEndState {
    Complete(WalkTheDogState<Walking>),
    Continue(WalkTheDogState<Ready>),
}

impl From<ReadyEndState> for WalkTheDogStateMachine {
    fn from(state: ReadyEndState) -> Self {
        match state {
            ReadyEndState::Complete(walking) => walking.into(),
            ReadyEndState::Continue(ready) => ready.into(),
        }
    }
}

enum WalkingEndState {
    Complete(WalkTheDogState<GameOver>),
    Continue(WalkTheDogState<Walking>),
}

impl From<WalkingEndState> for WalkTheDogStateMachine {
    fn from(state: WalkingEndState) -> Self {
        match state {
            WalkingEndState::Complete(knocked_out) => knocked_out.into(),
            WalkingEndState::Continue(walking) => walking.into(),
        }
    }
}

enum GameOverEndState {
    Complete(WalkTheDogState<Ready>),
    Continue(WalkTheDogState<GameOver>),
}

impl From<GameOverEndState> for WalkTheDogStateMachine {
    fn from(state: GameOverEndState) -> Self {
        match state {
            GameOverEndState::Complete(new_game) => new_game.into(),
            GameOverEndState::Continue(game_over) => game_over.into(),
        }
    }
}