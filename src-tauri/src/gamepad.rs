use crate::InputEvent;
use gilrs::{Axis, Button, Event as GamepadEvent, EventType as GamepadEventType, Gilrs};
use std::time::{Duration, Instant};
use tokio::sync::mpsc::Sender;

pub(crate) async fn handle_gamepad_input(sender: Sender<InputEvent>) {
	let mut gilrs = Gilrs::new().unwrap();
	let mut debounce: Option<Instant> = None;
	let mut latest_axis: Option<(gilrs::Axis, f32)> = None;

	'event_loop: loop {
		if let Some(GamepadEvent { id: _, event, .. }) = gilrs.next_event() {
			tracing::debug!("gamepad input event: {:?}", event);
			match event {
				GamepadEventType::AxisChanged(x, amp, ..) => {
					if let Some(inner) = debounce {
						if Instant::now() - inner < Duration::from_millis(200) {
							continue 'event_loop;
						} else {
							debounce = None;
						}
					}

					if let Some(inner) = latest_axis {
						if inner.0 == x {
							// NOTE: releasing the stick should not generate additional events. this
							// fixes that.
							if (inner.1 < 0.0 && amp > inner.1) || (inner.1 > 0.0 && amp < inner.1)
							{
								continue 'event_loop;
							}
						} else {
							latest_axis = None;
						}
					}

					let event = match x {
						Axis::LeftStickY => {
							if amp > 0.5 {
								Some(InputEvent::Up)
							} else if amp < -0.5 {
								Some(InputEvent::Down)
							} else {
								None
							}
						}
						Axis::LeftStickX => {
							if amp > 0.5 {
								Some(InputEvent::Right)
							} else if amp < -0.5 {
								Some(InputEvent::Left)
							} else {
								None
							}
						}
						_ => None,
					};

					if let Some(event) = event {
						latest_axis = Some((x, amp));
						debounce = Some(Instant::now());
						let _ = sender.send(event).await;
					}
				}
				GamepadEventType::ButtonPressed(x, ..) => {
					let event = match x {
						Button::DPadDown => Some(InputEvent::Down),
						Button::DPadUp => Some(InputEvent::Up),
						Button::DPadLeft => Some(InputEvent::Left),
						Button::DPadRight => Some(InputEvent::Right),
						Button::Start => Some(InputEvent::Menu),
						Button::South => Some(InputEvent::Ok),
						Button::East => Some(InputEvent::Cancel),
						Button::LeftTrigger => Some(InputEvent::PageUp),
						Button::RightTrigger => Some(InputEvent::PageDown),
						_ => None,
					};

					if let Some(event) = event {
						let _ = sender.send(event).await;
					}
				}
				_ => {}
			}
		} else {
			tokio::time::sleep(Duration::from_millis(200)).await;
		}
	}
}
