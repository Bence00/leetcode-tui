use crate::emit;

use super::utils::Paginate;
use leetcode_db::{Db, DbTopic};

pub struct Topic {
    pub visible: bool,
    pub paginate: Paginate<DbTopic>,
}

impl Topic {
    pub async fn new(db: &Db) -> Self {
        let topics = DbTopic::fetch_all(db).await.unwrap();
        let s = Self {
            paginate: Paginate::new(topics),
            visible: true,
        };
        s.notify_change();
        s
    }

    pub fn next(&mut self) -> bool {
        let has_topic_changed = self.paginate.next();
        if has_topic_changed {
            self.notify_change();
        }
        has_topic_changed
    }

    fn notify_change(&self) {
        if let Some(hovered) = self.hovered() {
            emit!(Topic(hovered.clone()));
        }
    }

    pub fn prev(&mut self) -> bool {
        let has_topic_changed = self.paginate.prev();
        if has_topic_changed {
            self.notify_change()
        };
        has_topic_changed
    }

    pub fn window(&self) -> &[DbTopic] {
        self.paginate.window()
    }

    pub fn set_visible(&mut self) {
        self.visible = true;
    }
}

impl Topic {
    pub fn hovered(&self) -> Option<&DbTopic> {
        self.paginate.hovered()
    }
}
