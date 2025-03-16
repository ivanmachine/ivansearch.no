use qdrant_client::{prelude::*, qdrant::VectorParams};
use std::env;

pub async fn init_qdrant() -> QdrantClient {
    let qdrant_url = env::var("QDRANT_URL").expect("QDRANT_URL not set");
    QdrantClient::from_url(&qdrant_url).build().unwrap()
}

pub async fn create_collection(client: &QdrantClient) {
    client
        .create_collection(&CreateCollection {
            collection_name: "pages".into(),
            vectors_config: Some(VectorsConfig::Params(VectorParams {
                size: 384,
                distance: Distance::Cosine.into(),
                ..Default::default()
            })),
            ..Default::default()
        })
        .await
        .expect("Failed to create collection");
}
