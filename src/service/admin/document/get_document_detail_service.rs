use crate::model::dto::Document;
use crate::repository::document::DocumentRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_document_detail(
    repo: &dyn DocumentRepo,
    id: i64,
) -> Result<Document, anyhow::Error> {
    let entity = repo
        .find_one(id)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                &e.to_string(),
            ))
        })?;

    let tags_vec: Vec<String> = if entity.tags.is_empty() {
        vec![]
    } else {
        entity
            .tags
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    };

    Ok(Document {
        id: entity.id,
        title: entity.title,
        content: entity.content,
        tags: tags_vec,
        show: entity.show.unwrap_or(false),
        created_at: entity.created_at,
        updated_at: entity.updated_at,
    })
}
