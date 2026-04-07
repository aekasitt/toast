/* ~~/src/main.rs */

// standard crates
use std::env::current_dir;
use std::sync::Arc;

// third-party crates
use async_trait::async_trait;
use axum::extract::{FromRequestParts, Json, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{Router, delete, get};
use http::request::Parts;
use serde::{Deserialize, Serialize};
use serde_json::json;
use toasty::{Db, Model};
use toasty_driver_sqlite::Sqlite;
use tokio::net::TcpListener;
use uuid::Uuid;

// Models
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

// Error handling
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

// Service schema and struct
#[async_trait]
trait TodoServiceSchema: Send + Sync {
  async fn create(&self, database: &mut Db, input: CreateTodo) -> Result<Todo, TodoError>;
  async fn complete(&self, database: &mut Db, slug: Uuid) -> Result<Todo, TodoError>;
  async fn delete(&self, database: &mut Db, slug: Uuid) -> Result<(), TodoError>;
  async fn get(&self, database: &mut Db, slug: Uuid) -> Result<Todo, TodoError>;
  async fn list(&self, database: &mut Db) -> Result<Vec<Todo>, TodoError>;
}
struct TodoService;
#[async_trait]
impl TodoServiceSchema for TodoService {
  async fn create(&self, database: &mut Db, input: CreateTodo) -> Result<Todo, TodoError> {
    let todo = toasty::create!(Todo {
      completed: false,
      title: input.title
    })
    .exec(database)
    .await?;
    Ok(todo)
  }
  async fn complete(&self, database: &mut Db, slug: Uuid) -> Result<Todo, TodoError> {
    let mut todo = Todo::get_by_slug(database, &slug).await?;
    Todo::update(&mut todo)
      .completed(true)
      .exec(database)
      .await?;
    Ok(todo)
  }
  async fn delete(&self, database: &mut Db, slug: Uuid) -> Result<(), TodoError> {
    let todo = Todo::get_by_slug(database, &slug).await?;
    todo.delete().exec(database).await?;
    Ok(())
  }
  async fn get(&self, database: &mut Db, slug: Uuid) -> Result<Todo, TodoError> {
    let todo = Todo::get_by_slug(database, &slug).await?;
    Ok(todo)
  }
  async fn list(&self, database: &mut Db) -> Result<Vec<Todo>, TodoError> {
    Ok(Todo::all().exec(database).await?)
  }
}

// Endpoints
async fn complete_todo(
  State(context): State<Context>,
  Database(mut database): Database,
  Path(slug): Path<Uuid>,
) -> Result<Json<Todo>, TodoError> {
  context
    .todosvc
    .complete(&mut database, slug)
    .await
    .map(Json)
}
async fn delete_todo(
  State(context): State<Context>,
  Database(mut database): Database,
  Path(slug): Path<Uuid>,
) -> Result<(StatusCode, Json<()>), TodoError> {
  context.todosvc.delete(&mut database, slug).await?;
  Ok((StatusCode::NO_CONTENT, Json(())))
}
async fn get_todo(
  State(context): State<Context>,
  Database(mut database): Database,
  Path(slug): Path<Uuid>,
) -> Result<Json<Todo>, TodoError> {
  context.todosvc.get(&mut database, slug).await.map(Json)
}
async fn create_todo(
  State(context): State<Context>,
  Database(mut database): Database,
  Json(input): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), TodoError> {
  let todo = context.todosvc.create(&mut database, input).await?;
  Ok((StatusCode::CREATED, Json(todo)))
}
async fn list_todos(
  State(context): State<Context>,
  Database(mut database): Database,
) -> Result<Json<Vec<Todo>>, TodoError> {
  context.todosvc.list(&mut database).await.map(Json)
}

// Context & dependency injection
pub struct Database(pub Db);
impl FromRequestParts<Context> for Database {
  type Rejection = TodoError;
  fn from_request_parts(
    _: &mut Parts,
    context: &Context,
  ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
    async { Ok(Database(context.database.clone())) }
  }
}
#[derive(Clone)]
struct Context {
  database: Db,
  todosvc: Arc<dyn TodoServiceSchema>,
}

// Lifespan
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let database_path = current_dir()?.join("todos.db");
  let database_url = format!("sqlite://{}", database_path.display());
  let driver = Sqlite::new(database_url)?;
  let database = Db::builder()
    .models(toasty::models!(Todo))
    .build(driver)
    .await?;
  match database.push_schema().await {
    Ok(_) => println!("✅ Migrated models into database tables."),
    Err(_) => println!("⚠️ Model tables already existed."),
  };
  let todosvc: Arc<dyn TodoServiceSchema> = Arc::new(TodoService {});
  let context = Context { database, todosvc };
  let app = Router::new()
    .route("/todos", get(list_todos).post(create_todo))
    .route(
      "/todos/{slug}",
      delete(delete_todo).get(get_todo).put(complete_todo),
    )
    .with_state(context);
  let listener = TcpListener::bind("127.0.0.1:3000").await?;
  println!("✅ Listening on http://localhost:3000");
  axum::serve(listener, app).await?;
  Ok(())
}
