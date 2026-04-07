/* ~~/src/main.rs */

// standard crates
use std::env::current_dir;
use std::sync::Arc;

// third-party crates
use async_trait::async_trait;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{Router, delete, get};
use serde::{Deserialize, Serialize};
use serde_json::json;
use toasty::{Db, Model};
use toasty_driver_sqlite::Sqlite;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone, Model, Serialize)]
struct Todo {
  completed: bool,
  #[key]
  #[auto(uuid(v4))]
  slug: Uuid,
  title: String,
}

#[derive(Deserialize)]
struct CreateTodo {
  title: String,
}

enum TodoError {
  Internal,
  NotFound(toasty::Error),
}
impl From<toasty::Error> for TodoError {
  fn from(e: toasty::Error) -> Self {
    if e.is_record_not_found() {
      TodoError::NotFound(e)
    } else {
      TodoError::Internal
    }
  }
}
impl IntoResponse for TodoError {
  fn into_response(self) -> Response {
    match self {
      TodoError::NotFound(error) => (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": format!("{error}") })),
      )
        .into_response(),
      TodoError::Internal => (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": "Oops, we did it again" })),
      )
        .into_response(),
    }
  }
}

#[async_trait]
trait TodoService: Send + Sync {
  async fn create(&self, input: CreateTodo) -> Result<Todo, TodoError>;
  async fn complete(&self, slug: Uuid) -> Result<Todo, TodoError>;
  async fn delete(&self, slug: Uuid) -> Result<(), TodoError>;
  async fn get(&self, slug: Uuid) -> Result<Todo, TodoError>;
  async fn list(&self) -> Result<Vec<Todo>, TodoError>;
}

struct SqliteTodoService {
  db: RwLock<Db>,
}
impl SqliteTodoService {
  async fn new(database_url: &str) -> Result<Self, toasty::Error> {
    let driver = Sqlite::new(database_url)?;
    let db = Db::builder()
      .models(toasty::models!(Todo))
      .build(driver)
      .await?;
    match db.push_schema().await {
      Ok(_) => println!("✅ Migrated models into database tables."),
      Err(_) => println!("⚠️ Model tables already existed."),
    };
    Ok(Self {
      db: RwLock::new(db),
    })
  }
}

#[async_trait]
impl TodoService for SqliteTodoService {
  async fn create(&self, input: CreateTodo) -> Result<Todo, TodoError> {
    let mut db = self.db.write().await;
    let todo = toasty::create!(Todo {
      completed: false,
      title: input.title
    })
    .exec(&mut *db)
    .await?;
    Ok(todo)
  }
  async fn complete(&self, slug: Uuid) -> Result<Todo, TodoError> {
    let mut db = self.db.write().await;
    let mut todo = Todo::get_by_slug(&mut *db, &slug).await?;
    Todo::update(&mut todo)
      .completed(true)
      .exec(&mut *db)
      .await?;
    Ok(todo)
  }
  async fn delete(&self, slug: Uuid) -> Result<(), TodoError> {
    let mut db = self.db.write().await;
    let todo = Todo::get_by_slug(&mut *db, &slug).await?;
    todo.delete().exec(&mut *db).await?;
    Ok(())
  }
  async fn get(&self, slug: Uuid) -> Result<Todo, TodoError> {
    let mut db = self.db.write().await;
    let todo = Todo::get_by_slug(&mut *db, &slug).await?;
    Ok(todo)
  }
  async fn list(&self) -> Result<Vec<Todo>, TodoError> {
    let mut db = self.db.write().await;
    Ok(Todo::all().exec(&mut *db).await?)
  }
}

async fn complete_todo(
  State(service): State<Arc<dyn TodoService>>,
  Path(slug): Path<Uuid>,
) -> Result<Json<Todo>, TodoError> {
  service.complete(slug).await.map(Json)
}
async fn delete_todo(
  State(service): State<Arc<dyn TodoService>>,
  Path(slug): Path<Uuid>,
) -> Result<(StatusCode, Json<()>), TodoError> {
  let deleted = service.delete(slug).await?;
  Ok((StatusCode::NO_CONTENT, Json(deleted)))
}
async fn get_todo(
  State(service): State<Arc<dyn TodoService>>,
  Path(slug): Path<Uuid>,
) -> Result<Json<Todo>, TodoError> {
  service.get(slug).await.map(Json)
}
async fn create_todo(
  State(service): State<Arc<dyn TodoService>>,
  Json(input): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), TodoError> {
  let todo = service.create(input).await?;
  Ok((StatusCode::CREATED, Json(todo)))
}
async fn list_todos(
  State(service): State<Arc<dyn TodoService>>,
) -> Result<Json<Vec<Todo>>, TodoError> {
  service.list().await.map(Json)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let db_path = current_dir()?.join("todos.db");
  let db_url = format!("sqlite://{}", db_path.display());
  let service = Arc::new(SqliteTodoService::new(&db_url).await?);
  let app = Router::new()
    .route("/todos", get(list_todos).post(create_todo))
    .route(
      "/todos/{slug}",
      delete(delete_todo).get(get_todo).put(complete_todo),
    )
    .with_state(service);
  let listener = TcpListener::bind("127.0.0.1:3000").await?;
  println!("✅ Listening on http://localhost:3000");
  axum::serve(listener, app).await?;
  Ok(())
}
