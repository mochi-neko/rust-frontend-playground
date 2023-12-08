use firebase_rust::auth::get_user_data::{
    GetUserDataRequestBodyPayload, GetUserDataResponsePayload,
};

use crate::generated::dotenv;

pub(crate) struct GetUserDataInfo {
    pub(crate) id_token: String,
}

pub(crate) async fn get_user_data(
    info: &GetUserDataInfo
) -> anyhow::Result<GetUserDataResponsePayload> {
    firebase_rust::auth::get_user_data::get_user_data(
        &dotenv::FIREBASE_API_KEY.to_string(),
        GetUserDataRequestBodyPayload::new(info.id_token.clone()),
    )
    .await
    .map_err(|error| {
        log::error!("Get user data failed: {:?}", error);
        anyhow::Error::new(error)
    })
}
