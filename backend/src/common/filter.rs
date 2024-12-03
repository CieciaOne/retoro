use chrono::{DateTime, Utc};
use log::debug;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Filter {
    pub after: Option<DateTime<Utc>>,
    pub before: Option<DateTime<Utc>>,
    pub thread: Option<Uuid>,
    pub author: Option<Uuid>,
    pub limit: Option<i64>,
}
impl Filter {
    /// To use this function pass a sql query e.q. "SELECT name FROM users"
    /// then it will process all the filters and add them as WHERE clause,
    /// or LIMIT in case that is present
    pub fn prepare(&self, mut query: String) -> String {
        // Add WHERE clause if filter is applied
        if self.after.is_some()
            || self.before.is_some()
            || self.thread.is_some()
            || self.author.is_some()
        {
            query.push_str(" WHERE ");

            let mut conditions = Vec::new();

            if let Some(start) = self.after {
                conditions.push(format!("created_at >= '{}'", start));
            }
            if let Some(end) = self.before {
                conditions.push(format!("created_at <= '{}'", end));
            }
            if let Some(thread) = self.thread {
                conditions.push(format!("thread_id = '{}'", thread));
            }
            if let Some(user) = self.author {
                conditions.push(format!("author_id = '{}'", user));
            }

            for (i, cond) in conditions.iter().enumerate() {
                if i > 0 {
                    query.push_str(" AND ");
                }
                query.push_str(cond);
            }
        }
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        query.push_str(";");
        debug!("{}", query);
        query
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_prepare_query_no_filters() {
        let filter = Filter {
            after: None,
            before: None,
            thread: None,
            author: None,
            limit: None,
        };
        assert_eq!(
            filter.prepare("SELECT * FROM posts".to_string()),
            "SELECT * FROM posts;"
        );
    }

    #[test]
    fn test_prepare_query_with_start_timestamp() {
        let start_timestamp = DateTime::from_timestamp(0, 0).unwrap();
        let filter = Filter {
            after: Some(start_timestamp),
            before: None,
            thread: None,
            author: None,
            limit: None,
        };
        assert_eq!(
            filter.prepare("SELECT * FROM posts".to_string()),
            format!(
                "SELECT * FROM posts WHERE created_at >= '{}';",
                start_timestamp.to_string()
            )
        );
    }

    #[test]
    fn test_prepare_query_with_end_timestamp() {
        let end_timestamp = DateTime::from_timestamp(0, 0).unwrap();
        let filter = Filter {
            after: None,
            before: Some(end_timestamp),
            thread: None,
            author: None,
            limit: None,
        };
        assert_eq!(
            filter.prepare("SELECT * FROM posts".to_string()),
            format!(
                "SELECT * FROM posts WHERE created_at <= '{}';",
                end_timestamp.to_string()
            )
        );
    }

    #[test]
    fn test_prepare_query_with_thread() {
        let thread = Uuid::new_v4();
        let filter = Filter {
            after: None,
            before: None,
            thread: Some(thread),
            author: None,
            limit: None,
        };
        assert_eq!(
            filter.prepare("SELECT * FROM posts".to_string()),
            format!("SELECT * FROM posts WHERE thread_id = {};", thread)
        );
    }

    #[test]
    fn test_prepare_query_with_user() {
        let user = Uuid::new_v4();
        let filter = Filter {
            after: None,
            before: None,
            thread: None,
            author: Some(user),
            limit: None,
        };
        assert_eq!(
            filter.prepare("SELECT * FROM posts".to_string()),
            format!("SELECT * FROM posts WHERE author_id = '{}';", user)
        );
    }

    #[test]
    fn test_prepare_query_with_limit() {
        let filter = Filter {
            after: None,
            before: None,
            thread: None,
            author: None,
            limit: Some(10),
        };
        assert_eq!(
            filter.prepare("SELECT * FROM threads".to_string()),
            "SELECT * FROM threads LIMIT 10;"
        );
    }

    #[test]
    fn test_prepare_query_with_multiple_filters() {
        let start_timestamp = chrono::DateTime::from_timestamp(0, 0).unwrap();
        let end_timestamp = chrono::DateTime::from_timestamp(100, 0).unwrap();
        let thread = Uuid::new_v4();
        let user = Uuid::new_v4();
        let limit = 10;
        let filter = Filter {
            after: Some(start_timestamp),
            before: Some(end_timestamp),
            thread: Some(thread),
            author: Some(user),
            limit: Some(limit),
        };
        assert_eq!(
            filter.prepare("SELECT * FROM posts".to_string()),
            format!("SELECT * FROM posts WHERE created_at >= '{start_timestamp}' AND created_at <= '{end_timestamp}' AND thread_id = '{thread}' AND author_id = '{user}' LIMIT {limit};")
        );
    }
}
