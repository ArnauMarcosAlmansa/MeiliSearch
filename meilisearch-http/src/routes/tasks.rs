use actix_web::{web, HttpRequest, HttpResponse};
use meilisearch_error::ResponseError;
use meilisearch_lib::tasks::task::TaskId;
use meilisearch_lib::MeiliSearch;
use serde_json::json;

use crate::analytics::Analytics;
use crate::extractors::authentication::{policies::*, GuardedData};
use crate::task::{TaskListView, TaskView};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(get_tasks)))
        .service(web::resource("/{task_id}").route(web::get().to(get_task)));
}

async fn get_tasks(
    meilisearch: GuardedData<ActionPolicy<{ actions::TASKS_GET }>, MeiliSearch>,
    req: HttpRequest,
    analytics: web::Data<dyn Analytics>,
) -> Result<HttpResponse, ResponseError> {
    analytics.publish(
        "Tasks Seen".to_string(),
        json!({ "per_task_uid": false }),
        Some(&req),
    );

    let tasks: TaskListView = meilisearch
        .list_tasks(None, None, None)
        .await?
        .into_iter()
        .map(TaskView::from)
        .collect::<Vec<_>>()
        .into();

    Ok(HttpResponse::Ok().json(tasks))
}

async fn get_task(
    meilisearch: GuardedData<ActionPolicy<{ actions::TASKS_GET }>, MeiliSearch>,
    task_id: web::Path<TaskId>,
    req: HttpRequest,
    analytics: web::Data<dyn Analytics>,
) -> Result<HttpResponse, ResponseError> {
    analytics.publish(
        "Tasks Seen".to_string(),
        json!({ "per_task_uid": true }),
        Some(&req),
    );

    let task: TaskView = meilisearch
        .get_task(task_id.into_inner(), None)
        .await?
        .into();

    Ok(HttpResponse::Ok().json(task))
}
