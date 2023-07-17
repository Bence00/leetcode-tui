use ratatui::widgets::ListState;

use crate::entities::question::Model as QuestionModel;
use crate::entities::topic_tag::Model as TopicTagModel;
use std::{
    collections::{HashMap, HashSet},
    error,
};

use super::list::StatefulList;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub type SS = (TopicTagModel, Vec<QuestionModel>);

pub type TTReciever = crossbeam::channel::Receiver<SS>;
pub type TTSender = crossbeam::channel::Sender<SS>;

#[derive(Debug)]
pub enum Widget<'a> {
    QuestionList(&'a mut StatefulList<QuestionModel>),
    TopicTagList(&'a mut StatefulList<TopicTagModel>),
}

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,

    pub widgets: &'a mut Vec<Widget<'a>>,

    pub questions_list: &'a HashMap<String, Vec<QuestionModel>>,

    pub widget_switcher: i32,
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new(
        wid: &'a mut Vec<Widget<'a>>,
        questions_list: &'a HashMap<String, Vec<QuestionModel>>,
    ) -> Self {
        let mut app = Self {
            running: true,
            questions_list,
            widgets: wid,
            widget_switcher: 0,
        };
        app.update_list();
        app
    }

    pub fn next_widget(&mut self) {
        let a = self.widget_switcher + 1;
        let b = self.widgets.len() as i32;
        self.widget_switcher = ((a % b) + b) % b;
    }

    pub fn prev_widget(&mut self) {
        let a = self.widget_switcher - 1;
        let b = self.widgets.len() as i32;
        self.widget_switcher = ((a % b) + b) % b;
    }

    pub fn get_current_widget(&self) -> &Widget {
        &self.widgets[self.widget_switcher as usize]
    }

    pub fn update_list(&mut self) {
        let mut name: Option<String> = None;

        match &self.widgets[self.widget_switcher as usize] {
            super::app::Widget::TopicTagList(ttl) => {
                if let Some(selected_widget) = ttl.get_selected_item() {
                    if let Some(n) = &selected_widget.name {
                        name = Some(n.clone());
                    }
                }
            }
            _ => {}
        }

        for w in self.widgets.iter_mut() {
            if let Widget::QuestionList(ql) = w {
                if let Some(name) = &name {
                    let mut items;
                    if name.as_str() == "All" {
                        let set = self
                            .questions_list
                            .values()
                            .flat_map(|q| q.clone())
                            .collect::<HashSet<_>>();
                        items = set.into_iter().map(|c| c.clone()).collect::<Vec<_>>();
                    } else {
                        items = self.questions_list.get(name).unwrap().clone();
                    }
                    items.sort();
                    ql.items = items;
                    ql.state = ListState::default();
                }
            }
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
