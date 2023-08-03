use std::rc::Rc;

use anyhow::Result;
use ratatui::{
    prelude::{Backend, Rect},
    Frame,
};

pub mod button;
pub mod header;
pub mod mission;
pub mod shortcuts;
pub mod sign_in;
pub mod text_input;

pub trait DrawableComponent {
    ///
    fn draw<B: Backend>(
        &mut self,
        state: &mut AppState,
        frame: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()>;
}

pub trait InputComponent {
    fn unique_name(&self) -> String;
    fn set_value(&mut self, value: String);
    fn value(&self) -> String;
}

pub trait AppComponentTrait<B: Backend, Props> {
    fn render(&self, frame: &mut Frame<B>, rect: Rect, props: &Props) -> Result<()>;
}

pub enum AppComponent {
    Actions(mission::actions::ActionsComponent),
    Button(button::ButtonComponent),
    ContextFiles(mission::context_files::ContextFilesComponent),
    Header(header::HeaderComponent),
    MessageInput(mission::message_input::MessageInputComponent),
    ProjectInfo(mission::project_info::ProjectInfoComponent),
    Shortcuts(shortcuts::ShortcutsComponent),
    TextInput(text_input::TextInputComponent),
}

impl AppComponent {
    pub fn draw<B: Backend>(
        &mut self,
        state: &mut AppState,
        frame: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()> {
        match self {
            AppComponent::Actions(component) => component.draw(state, frame, rect),
            AppComponent::Button(component) => component.draw(state, frame, rect),
            AppComponent::ContextFiles(component) => component.draw(state, frame, rect),
            AppComponent::Header(component) => component.draw(state, frame, rect),
            AppComponent::MessageInput(component) => component.draw(state, frame, rect),
            AppComponent::ProjectInfo(component) => component.draw(state, frame, rect),
            AppComponent::Shortcuts(component) => component.draw(state, frame, rect),
            AppComponent::TextInput(component) => component.draw(state, frame, rect),
        }
    }
}

// ================================================================================================

pub struct ComponentA {}

pub struct ComponentAProps {}

impl<B: Backend, TestComponentProps> AppComponentTrait<B, TestComponentProps> for ComponentA {
    fn render(&self, frame: &mut Frame<B>, rect: Rect, state: &TestComponentProps) -> Result<()> {
        Ok(())
    }
}

use std::any::Any;

use crate::app::AppState;

pub struct ComponentB {}

impl ComponentB {
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ComponentBProps {}

impl<B: Backend, TestComponentProps> AppComponentTrait<B, TestComponentProps> for ComponentB {
    fn render(&self, frame: &mut Frame<B>, rect: Rect, state: &TestComponentProps) -> Result<()> {
        Ok(())
    }
}

// pub enum AnyComponent {
//     ComponentA(ComponentA),
//     ComponentB(ComponentB),
// }

trait View {
    fn get_components(&self) -> Vec<&dyn Any>;
    fn render(&self);
}

struct ViewA<Component: std::any::Any> {
    components: Vec<Component>,
}

fn get_views() {
    let compA: Rc<&dyn std::any::Any> = Rc::new(&ComponentA {});
    let compAProps = ComponentAProps {};
    let compB = Rc::new(ComponentB {}.as_any());
    let compBProps = ComponentBProps {};
    let mut a: ViewA<Rc<&dyn std::any::Any>> = ViewA {
        components: vec![Rc::clone(&compA), Rc::clone(&compB)],
    };
}

fn app() {}
