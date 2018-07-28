pub type EventHandler<'a, T> = 'a + FnMut(&T) -> ();

#[derive(Deserialize)]
pub struct MouseEvent {
  pub alt_key: bool,
  pub client_x: u32,
  pub client_y: u32,
  pub ctrl_key: bool,
  pub layer_x: u32,
  pub layer_y: u32,
  pub meta_key: bool,
  pub movement_x: i32,
  pub movement_y: i32,
  pub offset_x: i32,
  pub offset_y: i32,
  pub page_x: u32,
  pub page_y: u32,
  pub screen_x: u32,
  pub screen_y: u32,
  pub shift_key: bool,
  pub time_stamp: f32,
  // type is a reserved word
  pub event_type: String,
  pub x: u32,
  pub y: u32,
}
pub type MouseEventHandler<'a> = EventHandler<'a, MouseEvent>;

#[derive(Deserialize)]
pub struct InputEvent {
  pub data: Option<String>,
  // N.B. not in firefox?
  // pub input_type: String, // todo use enum
  pub time_stamp: f32,
  pub event_type: String,
  pub which: u32,

  // *new field*
  pub value: Option<String>, // This is provided by our JS!
}
pub type InputEventHandler<'a> = EventHandler<'a, InputEvent>;

pub struct EventHandlers<'a> {
  pub on_click: Option<Box<MouseEventHandler<'a>>>,
  pub on_mouse_over: Option<Box<MouseEventHandler<'a>>>,
  pub on_mouse_out: Option<Box<MouseEventHandler<'a>>>,
  pub on_input: Option<Box<InputEventHandler<'a>>>,
}

impl<'a> EventHandlers<'a> {
  pub fn new() -> EventHandlers<'a> {
    EventHandlers {
      on_click: None,
      on_mouse_over: None,
      on_mouse_out: None,
      on_input: None,
    }
  }
}
