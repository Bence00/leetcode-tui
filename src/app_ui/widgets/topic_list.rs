use crate::{
    app_ui::{
        channel::{ChannelRequestSender, Response, TaskRequest, TaskResponse},
        components::{help_text::HelpText, list::StatefulList},
    },
    entities::TopicTagModel,
    errors::AppResult,
};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};

use super::{
    notification::{
        Notification,
        WidgetName::{self, QuestionList},
    },
    Callout, CommonState, CrosstermStderr, Widget,
};

#[derive(Debug)]
pub struct TopicTagListWidget {
    common_state: CommonState,
    pub topics: StatefulList<TopicTagModel>,
}

impl TopicTagListWidget {
    pub fn new(id: WidgetName, task_sender: ChannelRequestSender) -> Self {
        Self {
            common_state: CommonState::new(id, task_sender),
            topics: Default::default(),
        }
    }
}

impl TopicTagListWidget {
    fn get_item(ttm: &TopicTagModel) -> ListItem {
        ListItem::new(Text::styled(
            ttm.name
                .as_ref()
                .map_or("Not a Valid Tag".to_string(), |name| name.to_owned()),
            Style::default(),
        ))
    }

    fn update_questions(&mut self) -> AppResult<Option<Notification>> {
        if let Some(sel) = self.topics.get_selected_item() {
            let questions = vec![sel.as_ref().clone()];
            let notif = Notification::Questions(QuestionList, questions);
            return Ok(Some(notif));
        }
        Ok(None)
    }
}

impl Widget for TopicTagListWidget {
    fn set_active(&mut self) -> AppResult<Option<Notification>> {
        self.common_state.active = true;
        Ok(Some(Notification::HelpText(
            WidgetName::HelpLine,
            vec![
                HelpText::new(
                    "Switch Pane".to_string(),
                    vec![KeyCode::Left, KeyCode::Right],
                ),
                HelpText::new("Scroll Up".to_string(), vec![KeyCode::Up]),
                HelpText::new("Scroll Down".to_string(), vec![KeyCode::Down]),
            ],
        )))
    }
    fn render(&mut self, rect: Rect, frame: &mut CrosstermStderr) {
        let lines = self
            .topics
            .items
            .iter()
            .map(|tt| Self::get_item(tt))
            .collect::<Vec<_>>();

        let mut border_style = Style::default();

        if self.is_active() {
            border_style = border_style.fg(Color::Cyan);
        }

        let hstyle: Style = Callout::Info.into();
        let items = List::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Topics")
                    .border_style(border_style),
            )
            .highlight_style(hstyle.add_modifier(Modifier::BOLD));
        frame.render_stateful_widget(items, rect, &mut self.topics.state);
    }

    fn handler(&mut self, event: KeyEvent) -> AppResult<Option<Notification>> {
        match event.code {
            crossterm::event::KeyCode::Up => {
                self.topics.previous();
                return Ok(self.update_questions()?);
            }
            crossterm::event::KeyCode::Down => {
                self.topics.next();
                return Ok(self.update_questions()?);
            }
            _ => {}
        };
        Ok(None)
    }

    fn process_task_response(&mut self, response: TaskResponse) -> AppResult<Option<Notification>> {
        if let TaskResponse::AllTopicTags(Response {
            content,
            widget_name: _,
        }) = response
        {
            self.topics.add_item(TopicTagModel {
                name: Some("All".to_owned()),
                id: "all".to_owned(),
                slug: Some("all".to_owned()),
            });
            for tt in content {
                self.topics.add_item(tt)
            }
        }
        self.update_questions()
    }

    fn setup(&mut self) -> AppResult<Option<Notification>> {
        self.get_task_sender().send(TaskRequest::GetAllTopicTags {
            widget_name: self.get_widget_name(),
        })?;
        Ok(None)
    }

    fn set_response(&mut self) {}

    fn process_notification(
        &mut self,
        _notification: &Notification,
    ) -> AppResult<Option<Notification>> {
        Ok(None)
    }

    fn get_common_state(&self) -> &CommonState {
        &self.common_state
    }

    fn get_common_state_mut(&mut self) -> &mut CommonState {
        &mut self.common_state
    }
}
