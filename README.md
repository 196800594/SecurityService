# Security Service

**Check your Env**


| env                  |          type           |
|:---------------------|:-----------------------:|
| DATABASE_URL         |        MySQL Url        |
| JWT_SECRET           | openssl rand -base64 32 |
| JWT_EXPIRATION_HOURS |           int           |


**Table Struct** (*MySQL 8.0*)
```
CREATE TABLE users (
    uuid BINARY(16) PRIMARY KEY,
    username VARCHAR(256) UNIQUE NOT NULL,
    hash_pw VARCHAR(256) NOT NULL,
    nickname VARCHAR(64) NOT NULL
);
```
**APIs**
- [x] Login API
- ``url>> users/pw_auth/login``
- Request Body (*Json*)
```
{
    "username": "[USERNAME]",
    "password": "[PASSWORD]"
}
```
- Response Body (*Json*)
```
Status Code: 200

{
    "message": "登录成功",
    "data": {
        "token": "[TOKEN]
    }
}
```
```
Status Code: 401

{
"message": "登录失败，请检查用户名或密码",
"data": null
}
```

- [x] Register API
- ``url>> users/pw_auth/register``
- Request Body (*Json*)
```
{
    "username": "[USERNAME]",
    "password": "[PASSWORD]",
    "nickname": "[NICKNAME]
}
```
- Response Body (*Json*)
```
Status Code: 201

{
    "message": "注册成功",
    "data": null
}
```
```
Status Code: 409

{
"message": "用户名已存在",
"data": null
}
```
- [ ] Update API
- [ ] Drop API