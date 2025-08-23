use crate::{
    errors::posts_errors::PostError,
    models::{
        auth_models::Claims,
        posts_models::{CreatePost, GetAllPosts, PostsPath, UpdatePost},
    },
    repositories::posts_repository::PostsRepository,
};
use actix_web::{
    HttpMessage, HttpRequest, HttpResponse, Result, delete, get, post, put,
    web::{Data, Json, Path, ServiceConfig, scope},
};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::PgPool;
use validator::Validate;

/// Extracts user ID from the request's JWT.
fn extract_user_id(req: &HttpRequest) -> Result<i32, PostError> {
    req.extensions()
        .get::<Claims>()
        .map(|claims| claims.sub)
        .ok_or(PostError::NotFound)
}

#[post("")]
pub async fn create_post(
    req: HttpRequest,
    post_data: Json<CreatePost>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    let user_id = extract_user_id(&req)?;
    post_data.validate().map_err(PostError::Validation)?;

    let post =
        PostsRepository::create(&pool, post_data.into_inner(), user_id).await?;
    Ok(HttpResponse::Ok().json(post))
}

#[get("/all")]
pub async fn get_all_posts(
    pool: Data<PgPool>,
    post_data: Json<GetAllPosts>,
) -> Result<HttpResponse, PostError> {
    post_data.validate().map_err(PostError::Validation)?;
    let posts = PostsRepository::get_all(&pool, post_data.user_id).await?;
    log::info!("Found {} posts for user {}", posts.len(), post_data.user_id);
    Ok(HttpResponse::Ok().json(posts))
}

#[get("/{id}")]
pub async fn get_post(
    path: Path<i32>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    // let _user_id = extract_user_id(&req)?; // Chech authentication, but not used here
    let post_id = path.into_inner();
    let post = PostsRepository::find_by_id(&pool, post_id).await?;
    Ok(HttpResponse::Ok().json(post))
}

#[put("/{post_id}")]
pub async fn update_post(
    req: HttpRequest,
    path: Path<PostsPath>,
    post_data: Json<UpdatePost>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    println!("Updating post with ID: {}", path.post_id);
    let user_id = extract_user_id(&req)?;
    post_data.validate().map_err(PostError::Validation)?;
    path.validate().map_err(PostError::Validation)?;

    let post_id = path.post_id;
    println!("Updating post with ID: {}", post_id);
    let post = PostsRepository::find_by_id(&pool, post_id).await?;
    if post.user_id != user_id {
        return Err(PostError::Unauthorized(
            "You can only update your own posts".to_string(),
        ));
    }

    let updated_post =
        PostsRepository::update(&pool, post_id, post_data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated_post))
}

#[delete("/{id}")]
pub async fn delete_post(
    req: HttpRequest,
    path: Path<i32>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    let user_id = extract_user_id(&req)?;
    let post_id = path.into_inner();

    let post = PostsRepository::find_by_id(&pool, post_id).await?;
    if post.user_id != user_id {
        return Err(PostError::Unauthorized(
            "You can only delete your own posts".to_string(),
        ));
    }

    PostsRepository::delete(&pool, post_id).await?;
    Ok(HttpResponse::Ok().json(()))
}

pub fn posts_routes(cfg: &mut ServiceConfig) {
    let auth = HttpAuthentication::bearer(
        crate::middlewares::auth_middleware::auth_middleware_validator,
    );

    cfg.service(
        scope("/posts").service(get_all_posts).service(get_post).service(
            scope("")
                .wrap(auth)
                .service(create_post)
                .service(update_post)
                .service(delete_post),
        ),
    );
}
