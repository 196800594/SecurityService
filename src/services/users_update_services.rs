use sqlx::{MySqlPool};
use crate::models::{Error, Success, UpdateRequest, UpdateResponse};
use crate::utils::{hash_pw, token_verify};

#[derive(Clone)]
enum DataType {
    Username,
    Password,
    Nickname,
}
impl DataType {
    fn into(self) -> String {
        match self {
            DataType::Username => "username".to_string(),
            DataType::Password => "hash_pw".to_string(),
            DataType::Nickname => "nickname".to_string(),
        }
    }
}

//数据类型和格式校验
fn data_type_verify(data_type: &DataType, request: &UpdateRequest) -> Option<String> {
    let request = request.clone();
    match data_type {
        DataType::Username => { if request.username.is_some() & request.password.is_none() &request.nickname.is_none() {
            if param_is_not_empty(&request.username) { request.username } else { None }
        } else { None } }
        DataType::Password => {
            if request.password.is_some() & request.username.is_none() & request.nickname.is_none() {
                if param_is_not_empty(&request.password) {
                    //密码需要额外将其转为哈希
                    if let Some(password) = request.password { Some(hash_pw(password)) } else { None } } else { None } } else { None } }
        DataType::Nickname => {
            if request.nickname.is_some() & request.username.is_none() & request.password.is_none() {
                if param_is_not_empty(&request.nickname) { request.nickname } else { None }
            } else { None } }
    }
}
//判断字符串是否为空串
fn param_is_not_empty(value: &Option<String>) -> bool {
    let value = value.as_ref().unwrap();
    if !value.is_empty(){ true } else { false }
}

//单 column 数据更新
async fn single_data_update(data_type: &DataType, pool: &MySqlPool, request: &UpdateRequest) -> (Result<Success, Error>, Option<UpdateResponse>) {
    //验证参数格式是否正确
    let data_type = data_type.clone();
    if let Some(value) = data_type_verify(&data_type, &request) {
        //验证 token的合法性
        let token_result = token_verify(request.token.clone());
        match token_result {
            Ok(uuid_bytes) => {
                let data_type_string = data_type.clone().into();
                let query = format!("UPDATE users SET {} = ? WHERE uuid = ?", data_type_string);
                let row = sqlx::query(sqlx::AssertSqlSafe(query))
                    .bind(value)
                    .bind(uuid_bytes)
                    .execute(pool)
                    .await;
                match row {
                    Ok(row) => {
                        if row.rows_affected() > 0 {
                            match data_type {
                                DataType::Username => {(Ok(Success::UpdateUsernameSuccess), Some(UpdateResponse{})) }
                                DataType::Password => {(Ok(Success::UpdatePasswordSuccess), Some(UpdateResponse{})) }
                                DataType::Nickname => {(Ok(Success::UpdateNicknameSuccess), Some(UpdateResponse{})) }
                            }
                        }
                        else { (Err(Error::InternalError), None) }
                    }
                    Err(_) => { (Err(Error::InternalError), None) }
                }
            }
            Err(_) => { (Err(Error::TokenExpired), None) }
        }
    } else { (Err(Error::InvalidParameter), None) }
}

pub async fn users_update_username(pool: &MySqlPool, request: UpdateRequest) -> (Result<Success, Error>, Option<UpdateResponse>) {
    single_data_update(&DataType::Username, &pool, &request).await
}

pub async fn users_update_password(pool: &MySqlPool, request: UpdateRequest) -> (Result<Success, Error>, Option<UpdateResponse>) {
    single_data_update(&DataType::Password, &pool, &request).await
}

pub async fn users_update_nickname(pool: &MySqlPool, request: UpdateRequest) -> (Result<Success, Error>, Option<UpdateResponse>) {
    single_data_update(&DataType::Nickname, &pool, &request).await
}




