use ::{Element, View, QuasarApp, Renderable};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub enum EventType {
    Click,
    DoubleClick,
    MouseDown,
    MouseUp,
    MouseEnter,
    MouseLeave,
    MouseOver,
    MouseOut,
    Input,
    Submit,
    Blur,
    Focus,
}

impl EventType {
    pub fn name(&self) -> &'static str {
        match *self {
            EventType::Click => "click",
            EventType::DoubleClick => "dblclick",
            EventType::MouseDown => "mousedown",
            EventType::MouseUp => "mouseup",
            EventType::MouseEnter => "mouseenter",
            EventType::MouseLeave => "mouseleave",
            EventType::MouseOver => "mouseover",
            EventType::MouseOut => "mouseout",
            EventType::Input => "input",
            EventType::Submit => "submit",
            EventType::Blur => "blur",
            EventType::Focus => "focus",
        }
    }
}

pub struct Event<'a, 'b, 'c, R> {
    pub app: QuasarApp<'a>,
    pub target: Element<'b>,
    pub view: View<'c, R>,
}

