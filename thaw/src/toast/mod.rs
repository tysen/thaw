mod toast_body;
mod toast_footer;
mod toast_title;
mod toaster;
mod toaster_provider;

pub use toast_body::*;
pub use toast_footer::*;
pub use toast_title::*;
pub use toaster_provider::*;

use leptos::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender, TryIter};
use wasm_bindgen::UnwrapThrowExt;

#[derive(Clone, Copy)]
pub struct ToasterInjection {
    sender: StoredValue<Sender<ToasterMessage>>,
    trigger: StoredValue<ArcTrigger>,
}

enum ToasterMessage {
    Dispatch(Children, ToastOptions),
    Dismiss(uuid::Uuid),
    DismissAll,
}

impl ToasterInjection {
    #[track_caller]
    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn channel() -> (Self, ToasterReceiver) {
        let (sender, receiver) = channel::<ToasterMessage>();
        let trigger = ArcTrigger::new();

        (
            Self {
                sender: StoredValue::new(sender),
                trigger: StoredValue::new(trigger.clone()),
            },
            ToasterReceiver::new(receiver, trigger),
        )
    }

    pub fn dismiss_toast(&self, toast_id: uuid::Uuid) {
        self.sender.with_value(|sender| {
            sender
                .send(ToasterMessage::Dismiss(toast_id))
                .unwrap_throw()
        });
        self.trigger.with_value(|trigger| trigger.notify());
    }

    pub fn dismiss_all(&self) {
        self.sender
            .with_value(|sender| sender.send(ToasterMessage::DismissAll).unwrap_throw());
        self.trigger.with_value(|trigger| trigger.notify());
    }

    pub fn dispatch_toast<C, IV>(&self, children: C, options: ToastOptions)
    where
        C: FnOnce() -> IV + Send + 'static,
        IV: IntoView + 'static,
    {
        self.sender.with_value(|sender| {
            sender
                .send(ToasterMessage::Dispatch(
                    Box::new(move || children().into_any()),
                    options,
                ))
                .unwrap_throw()
        });
        self.trigger.with_value(|trigger| trigger.notify());
    }
}

pub struct ToasterReceiver {
    receiver: Receiver<ToasterMessage>,
    trigger: ArcTrigger,
}

impl ToasterReceiver {
    fn new(receiver: Receiver<ToasterMessage>, trigger: ArcTrigger) -> Self {
        Self { receiver, trigger }
    }

    fn try_recv(&self) -> TryIter<'_, ToasterMessage> {
        self.trigger.track();
        self.receiver.try_iter()
    }
}

use std::time::Duration;
use thaw_utils::{class_list, ArcOneCallback};

#[component]
pub fn Toast(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! { <div class=class_list!["thaw-toast", class]>{children()}</div> }
}

#[derive(Default, Clone, Copy)]
pub enum ToastPosition {
    Top,
    TopStart,
    TopEnd,
    Bottom,
    BottomStart,
    #[default]
    BottomEnd,
}

impl ToastPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::TopStart => "top-start",
            Self::TopEnd => "top-dnc",
            Self::Bottom => "bottom",
            Self::BottomStart => "bottom-start",
            Self::BottomEnd => "bottom-end",
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum ToastIntent {
    Success,
    #[default]
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToastStatus {
    Mounted,
    Unmounted,
}

#[derive(Clone)]
pub struct ToastOptions {
    pub(crate) id: uuid::Uuid,
    pub(crate) position: Option<ToastPosition>,
    pub(crate) timeout: Option<Duration>,
    pub(crate) intent: Option<ToastIntent>,
    pub(crate) on_status_change: Option<ArcOneCallback<ToastStatus>>,
}

impl Default for ToastOptions {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            position: None,
            timeout: None,
            intent: None,
            on_status_change: None,
        }
    }
}

impl ToastOptions {
    /// The id that will be assigned to this toast.
    pub fn with_id(mut self, id: uuid::Uuid) -> Self {
        self.id = id;
        self
    }

    /// The position the toast should render.
    pub fn with_position(mut self, position: ToastPosition) -> Self {
        self.position = Some(position);
        self
    }

    /// Auto dismiss timeout in milliseconds.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Intent.
    pub fn with_intent(mut self, intent: ToastIntent) -> Self {
        self.intent = Some(intent);
        self
    }

    /// Status change callback.
    pub fn with_on_status_change(
        mut self,
        on_status_change: impl Fn(ToastStatus) + Send + Sync + 'static,
    ) -> Self {
        self.on_status_change = Some(on_status_change.into());
        self
    }
}
