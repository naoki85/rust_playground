use crate::engine::Point;

use super::RedHatBoyStateMachine;
const FLOOR: i16 = 479;
const STARTING_POINT: i16 = -20;
const IDLE_FRAME_NAME: &str = "Idle";
const RUN_FRAME_NAME: &str = "Run";
const SLIDING_FRAME_NAME: &str = "Slide";
const JUMPING_FRAME_NAME: &str = "Jump";
const FALING_FRAME_NAME: &str = "Dead";
const IDLE_FRAMES: u8 = 29;
const RUNNING_FRAMES: u8 = 23;
const RUNNING_SPEED: i16 = 3;
const SLIDING_FRAMES: u8 = 14;
const JUMPING_FRAMES: u8 = 35;
const JUMP_SPEED: i16 = -25;
const FALING_FRAMES: u8 = 29;
const GRAVITY: i16 = 1;


#[derive(Copy, Clone)]
pub struct RedHatBoyState<S> {
    context: RedHatBoyContext,
    _state: S,
}

impl<S> RedHatBoyState<S> {
  pub fn context(&self) -> &RedHatBoyContext {
    &self.context
  }
}

impl RedHatBoyState<Idle> {
    pub fn new() -> Self {
        RedHatBoyState {
            context: RedHatBoyContext {
                frame: 0,
                position: Point { x: STARTING_POINT, y: FLOOR },
                velocity: Point { x: 0, y: 0 },
            },
            _state: Idle {},
        }
    }
    pub fn run(self) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context.reset_frame().run_right(),
            _state: Running {},
        }
    }

    pub fn frame_name(&self) -> &str {
      IDLE_FRAME_NAME
    }

    pub fn update(mut self) -> Self {
      self.context = self.context.update(IDLE_FRAMES);
      self
    }
}

impl RedHatBoyState<Running> {
    pub fn frame_name(&self) -> &str {
      RUN_FRAME_NAME
    }
    pub fn update(mut self) -> Self {
      self.context = self.context.update(RUNNING_FRAMES);
      self
    }

    pub fn slide(self) -> RedHatBoyState<Sliding> {
      RedHatBoyState { context: self.context.reset_frame(), _state: Sliding {}, }
    }

    pub fn jump(self) -> RedHatBoyState<Jumping> {
      RedHatBoyState {
        context: self.context.set_vertical_velocity(JUMP_SPEED).reset_frame(),
        _state: Jumping {},
      }
    }

    pub fn knock_out(self) -> RedHatBoyState<Falling> {
      RedHatBoyState {
        context: self.context.reset_frame().stop(),
        _state: Falling {},
      }
    }
}

impl RedHatBoyState<Sliding> {
    pub fn frame_name(&self) -> &str {
      SLIDING_FRAME_NAME
    }

    pub fn update(mut self) -> SlidingEndState {
      self.context = self.context.update(SLIDING_FRAMES);

      if self.context.frame >= SLIDING_FRAMES {
        SlidingEndState::Complete(self.stand())
      } else {
        SlidingEndState::Sliding(self) 
      }
    }

    pub fn stand(self) -> RedHatBoyState<Running> {
      RedHatBoyState { context: self.context.reset_frame(), _state: Running, }
    }

    pub fn knock_out(self) -> RedHatBoyState<Falling> {
      RedHatBoyState {
        context: self.context,
        _state: Falling {},
      }
    }
}

impl RedHatBoyState<Jumping> {
    pub fn frame_name(&self) -> &str {
        JUMPING_FRAME_NAME
    }

    pub fn update(mut self) -> JumpingEndState {
      self.context = self.context.update(JUMPING_FRAMES);
      if self.context.position.y >= FLOOR {
        JumpingEndState::Complete(self.land_on(FLOOR as f32))
      } else {
        JumpingEndState::Jumping(self)
      }
    }

    pub fn land_on(mut self, position: f32) -> RedHatBoyState<Running> {
      self.context.position.y = position as i16;
      RedHatBoyState { context: self.context.reset_frame(), _state: Running, } 
    }

    pub fn knock_out(self) -> RedHatBoyState<Falling> {
      RedHatBoyState {
        context: self.context,
        _state: Falling {},
      }
    }
}

impl RedHatBoyState<Falling> {
    pub fn frame_name(&self) -> &str {
      FALING_FRAME_NAME
    }

    pub fn update(mut self) -> FallingEndState {
      self.context = self.context.update(FALING_FRAMES);

      if self.context.frame >= FALING_FRAMES {
        FallingEndState::KnockedOut(self.knock_out())
      } else {
        FallingEndState::Falling(self) 
      }
    }

    pub fn knock_out(self) -> RedHatBoyState<KnockedOut> {
      RedHatBoyState {
        context: self.context,
        _state: KnockedOut {},
      }
    }
}

impl RedHatBoyState<KnockedOut> {
    pub fn frame_name(&self) -> &str {
      FALING_FRAME_NAME
    }
  }

pub enum SlidingEndState {
  Complete(RedHatBoyState<Running>),
  Sliding(RedHatBoyState<Sliding>),
}

pub enum JumpingEndState {
  Complete(RedHatBoyState<Running>),
  Jumping(RedHatBoyState<Jumping>),
}

pub enum FallingEndState {
  KnockedOut(RedHatBoyState<KnockedOut>),
  Falling(RedHatBoyState<Falling>),
}

impl From<SlidingEndState> for RedHatBoyStateMachine {
  fn from(end_state: SlidingEndState) -> Self {
    match end_state {
      SlidingEndState::Complete(running_state) => running_state.into(),
      SlidingEndState::Sliding(sliding_state) => sliding_state.into(),
    }
  }
}

impl From<JumpingEndState> for RedHatBoyStateMachine {
  fn from(end_state: JumpingEndState) -> Self {
    match end_state {
      JumpingEndState::Complete(running_state) => running_state.into(),
      JumpingEndState::Jumping(jumping_state) => jumping_state.into(),
    }
  }
}

impl From<FallingEndState> for RedHatBoyStateMachine {
  fn from(end_state: FallingEndState) -> Self {
    match end_state {
      FallingEndState::Falling(falling) => falling.into(),
      FallingEndState::KnockedOut(knocked_out) => knocked_out.into(),
    }
  }
}

#[derive(Copy, Clone)]
pub struct RedHatBoyContext {
    pub frame: u8,
    pub position: Point,
    pub velocity: Point,
}

impl RedHatBoyContext {
  pub fn update(mut self, frame_count: u8) -> Self {

    if self.frame < frame_count {
      self.frame += 1;
    } else {
      self.frame = 0;
    }

    self.velocity.y += GRAVITY;

    self.position.x += self.velocity.x;
    self.position.y += self.velocity.y;
    if self.position.y > FLOOR {
      self.position.y = FLOOR;
    }
    self
  }

  pub fn reset_frame(mut self) -> Self {
    self.frame = 0;
    self
  }

  pub fn run_right(mut self) -> Self {
    self.velocity.x += RUNNING_SPEED;
    self
  }

  fn set_vertical_velocity(mut self, y: i16) -> Self {
    self.velocity.y = y;
    self
  }

  fn stop(mut self) -> Self {
    self.velocity.x = 0;
    self
  }
}

#[derive(Copy, Clone)]
pub struct Idle;
#[derive(Copy, Clone)]
pub struct Running;
#[derive(Copy, Clone)]
pub struct Sliding;
#[derive(Copy, Clone)]
pub struct Jumping;
#[derive(Copy, Clone)]
pub struct Falling;
#[derive(Copy, Clone)]
pub struct KnockedOut;
