# Security Service

Security Service 是一个基于 Rust + Axum 的用户认证与信息管理服务，使用 MySQL 存储用户数据，支持 JWT 认证、密码哈希、国际化消息和用户资料更新。

## 功能概览

- 用户注册
- 用户登录
- JWT 令牌生成与校验
- 用户名更新
- 密码更新
- 昵称更新
- 中英文国际化支持（zh-CN / en-US）

## 技术栈

- Rust 2024
- Axum
- Tokio
- SQLx
- MySQL 8.0
- JWT (jwt-simple)
- Argon2
- rust-i18n

## 项目结构

```text
SecurityService/
├── src/
│   ├── main.rs
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── users_auth_routes.rs
│   │   └── users_update_routes.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── users_auth_handlers.rs
│   │   └── users_update_handlers.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── users_auth_services.rs
│   │   └── users_update_services.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── login.rs
│   │   ├── register.rs
│   │   ├── update.rs
│   │   ├── response.rs
│   │   └── pool.rs
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── hash_util.rs
│   │   ├── jwt_util.rs
│   │   └── lang_extractor.rs
│   ├── errors/
│   └── ...
├── langs/
│   ├── en-US.yml
│   └── zh-CN.yml
├── Cargo.toml
├── Cargo.lock
├── LICENSE.txt
├── README.md
└── .gitignore
```

## 环境变量

在项目根目录创建 `.env` 文件，并配置下面的环境变量：

| 环境变量 | 类型 | 说明 |
|:---|:---|:---|
| `DATABASE_URL` | string | MySQL 数据库连接地址 |
| `JWT_SECRET` | string | JWT 签名密钥 |
| `JWT_EXPIRATION_HOURS` | int | JWT 过期时间（小时） |

示例：

```bash
DATABASE_URL=mysql://root:password@localhost:3306/security_service
JWT_SECRET=your_secret_key_here
JWT_EXPIRATION_HOURS=24
```

推荐生成密钥：

```bash
openssl rand -base64 32
```

## 数据库结构

当前代码使用的用户表如下：

```sql
CREATE TABLE users (
    uuid BINARY(16) PRIMARY KEY,
    username VARCHAR(256) UNIQUE NOT NULL,
    hash_pw VARCHAR(256) NOT NULL,
    nickname VARCHAR(64) NOT NULL
);
```

说明：
- `uuid` 用于唯一标识用户
- `username` 作为登录名，唯一约束
- `hash_pw` 为加密后的密码
- `nickname` 为用户昵称

## 安装与运行

### 1. 安装 Rust

如果你还没有安装 Rust，请先安装：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. 安装依赖

```bash
cargo build
```

### 3. 启动服务

```bash
cargo run
```

服务默认监听：

```bash
0.0.0.0:3000
```

## API 接口

注意：当前代码中路由使用的是 `pw-auth` 和 `update` 形式，路径中使用连字符。

### 1. 登录

- Method: `POST`
- URL: `/users/pw-auth/login`

请求体：

```json
{
  "username": "demo_user",
  "password": "demo_password"
}
```

成功响应示例：

```json
{
  "message": "登录成功",
  "data": {
    "token": "..."
  }
}
```

失败响应示例：

```json
{
  "message": "登录失败，请检查用户名或密码",
  "data": null
}
```

---

### 2. 注册

- Method: `POST`
- URL: `/users/pw-auth/register`

请求体：

```json
{
  "username": "demo_user",
  "password": "demo_password",
  "nickname": "DemoUser"
}
```

成功响应示例：

```json
{
  "message": "注册成功",
  "data": null
}
```

失败响应示例：

```json
{
  "message": "用户名已存在",
  "data": null
}
```

---

### 3. 更新用户名

- Method: `PUT`
- URL: `/users/update/username`

请求体：

```json
{
  "token": "your_jwt_token",
  "username": "new_username"
}
```

---

### 4. 更新密码

- Method: `PUT`
- URL: `/users/update/password`

请求体：

```json
{
  "token": "your_jwt_token",
  "password": "new_password"
}
```

---

### 5. 更新昵称

- Method: `PUT`
- URL: `/users/update/nickname`

请求体：

```json
{
  "token": "your_jwt_token",
  "nickname": "new_nickname"
}
```

---

## 认证说明

登录成功后，服务会返回一个 JWT Token，后续更新用户资料时需要在请求体中附带该 token。服务会先验证 token，并从 token 中解析出用户 UUID，然后执行对应的数据库更新操作。

## 国际化

项目已集成 `rust-i18n`，支持以下语言：

- `zh-CN`
- `en-US`

对应翻译文件位于：

```text
langs/
├── zh-CN.yml
└── en-US.yml
```

## 备注

- 当前项目是一个后端认证服务，主要面向 API 调用。
- 代码中路径命名为 `pw-auth` 与 `update`，如 `/users/pw-auth/login`。
- 用户密码在数据库中以哈希值存储，不直接保存明文。

## License

本项目使用 MIT/兼容许可证，具体请查看 `LICENSE.txt`。
