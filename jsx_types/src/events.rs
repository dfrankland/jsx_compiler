pub type EventHandler<'a, T> = 'a + FnMut(&T) -> ();
use web_sys::{
  ClipboardEvent,
  KeyboardEvent,
  UiEvent,
  FocusEvent,
  MouseEvent,
  PointerEvent,
  TouchEvent,
  ScrollAreaEvent,
  InputEvent,
  AnimationEvent,
  TransitionEvent,
};

pub type UiEventHandler<'a> = EventHandler<'a, UiEvent>;
pub type ClipboardEventHandler<'a> = EventHandler<'a, ClipboardEvent>;
pub type KeyboardEventHandler<'a> = EventHandler<'a, KeyboardEvent>;
pub type InputEventHandler<'a> = EventHandler<'a, InputEvent>;
pub type FocusEventHandler<'a> = EventHandler<'a, FocusEvent>;
pub type MouseEventHandler<'a> = EventHandler<'a, MouseEvent>;
pub type PointerEventHandler<'a> = EventHandler<'a, PointerEvent>;
pub type TouchEventHandler<'a> = EventHandler<'a, TouchEvent>;
pub type ScrollAreaEventHandler<'a> = EventHandler<'a, ScrollAreaEvent>;
pub type AnimationEventHandler<'a> = EventHandler<'a, AnimationEvent>;
pub type TransitionEventHandler<'a> = EventHandler<'a, TransitionEvent>;

#[derive(Default)]
pub struct EventHandlers<'a> {
  // --Clipboard
  pub on_copy: Option<Box<ClipboardEventHandler<'a>>>,
  pub on_cut: Option<Box<ClipboardEventHandler<'a>>>,
  pub on_paste: Option<Box<ClipboardEventHandler<'a>>>,
  // --Composition
  // onCompositionEnd
  // onCompositionStart
  // onCompositionUpdate
  // --Keyboard
  pub on_key_down: Option<Box<KeyboardEventHandler<'a>>>,
  pub on_key_press: Option<Box<KeyboardEventHandler<'a>>>,
  pub on_key_up: Option<Box<KeyboardEventHandler<'a>>>,
  // --Focus
  pub on_focus: Option<Box<FocusEventHandler<'a>>>,
  pub on_blur: Option<Box<FocusEventHandler<'a>>>,
  // --Form
  pub on_change: Option<Box<InputEventHandler<'a>>>,
  pub on_input: Option<Box<InputEventHandler<'a>>>,
  pub on_invalid: Option<Box<InputEventHandler<'a>>>,
  pub on_submit: Option<Box<InputEventHandler<'a>>>,
  // --Mouse
  pub on_click: Option<Box<MouseEventHandler<'a>>>,
  pub on_context_menu: Option<Box<MouseEventHandler<'a>>>,
  pub on_dbl_click: Option<Box<MouseEventHandler<'a>>>,

  pub on_drag: Option<Box<MouseEventHandler<'a>>>,
  pub on_drag_end: Option<Box<MouseEventHandler<'a>>>,
  pub on_drag_enter: Option<Box<MouseEventHandler<'a>>>,
  pub on_drag_exit: Option<Box<MouseEventHandler<'a>>>,
  pub on_drag_leave: Option<Box<MouseEventHandler<'a>>>,
  pub on_drag_over: Option<Box<MouseEventHandler<'a>>>,
  pub on_drag_start: Option<Box<MouseEventHandler<'a>>>,
  pub on_drop: Option<Box<MouseEventHandler<'a>>>,

  pub on_mouse_down: Option<Box<MouseEventHandler<'a>>>,
  pub on_mouse_enter: Option<Box<MouseEventHandler<'a>>>,
  pub on_mouse_leave: Option<Box<MouseEventHandler<'a>>>,
  pub on_mouse_move: Option<Box<MouseEventHandler<'a>>>,
  pub on_mouse_over: Option<Box<MouseEventHandler<'a>>>,
  pub on_mouse_out: Option<Box<MouseEventHandler<'a>>>,
  pub on_mouse_up: Option<Box<MouseEventHandler<'a>>>,
  // --Pointer
  pub on_pointer_down: Option<Box<PointerEventHandler<'a>>>,
  pub on_pointer_move: Option<Box<PointerEventHandler<'a>>>,
  pub on_pointer_up: Option<Box<PointerEventHandler<'a>>>,
  pub on_pointer_cancel: Option<Box<PointerEventHandler<'a>>>,
  pub on_got_pointer_capture: Option<Box<PointerEventHandler<'a>>>,
  pub on_lost_pointer_capture: Option<Box<PointerEventHandler<'a>>>,
  pub on_pointer_enter: Option<Box<PointerEventHandler<'a>>>,
  pub on_pointer_leave: Option<Box<PointerEventHandler<'a>>>,
  pub on_pointer_over: Option<Box<PointerEventHandler<'a>>>,
  pub on_pointer_out: Option<Box<PointerEventHandler<'a>>>,
  // --Selection
  pub on_select: Option<Box<UiEventHandler<'a>>>,
  // --Touch
  pub on_touch_cancel: Option<Box<TouchEventHandler<'a>>>,
  pub on_touch_end: Option<Box<TouchEventHandler<'a>>>,
  pub on_touch_move: Option<Box<TouchEventHandler<'a>>>,
  pub on_touch_start: Option<Box<TouchEventHandler<'a>>>,
  pub on_scroll: Option<Box<ScrollAreaEventHandler<'a>>>,
  // --Wheel
  // onWheel
  // --Media
  // onAbort
  // onCanPlay
  // onCanPlayThrough
  // onDurationChange
  // onEmptied
  // onEncrypted
  // onEnded
  // onError
  // onLoadedData
  // onLoadedMetadata
  // onLoadStart
  // onPause
  // onPlay
  // onPlaying
  // onProgress
  // onRateChange
  // onSeeked
  // onSeeking
  // onStalled
  // onSuspend
  // onTimeUpdate
  // onVolumeChange
  // onWaiting
  // --Image
  pub on_load: Option<Box<UiEventHandler<'a>>>,
  pub on_error: Option<Box<UiEventHandler<'a>>>,
  // --Animation
  pub on_animation_start: Option<Box<AnimationEventHandler<'a>>>,
  pub on_animation_end: Option<Box<AnimationEventHandler<'a>>>,
  pub on_animation_iteration: Option<Box<AnimationEventHandler<'a>>>,
  // --Transition
  pub on_transition_end: Option<Box<TransitionEventHandler<'a>>>,
  // --Other
  pub on_toggle: Option<Box<UiEventHandler<'a>>>,
}
