# Aria2 下载数据库设计文档

## 概述

本项目为 BurnCloud Aria2 下载器设计一个精简而功能完整的数据库系统，用于管理下载任务、配置和统计信息。

基于 `../burncloud-database` 架构，使用 SQLite 作为后端数据库。

## 设计原则

1. **极度精简**: 最少的表结构和字段
2. **功能完整**: 满足 aria2 下载管理的核心需求
3. **高性能**: 优化的索引和查询结构
4. **易扩展**: 为未来功能预留扩展空间

## 数据库表结构

### 1. 下载任务表 (downloads)

存储 aria2 下载任务的核心信息。

```sql
CREATE TABLE downloads (
    -- 基础字段
    gid TEXT PRIMARY KEY,                     -- aria2 全局唯一标识符
    status TEXT NOT NULL DEFAULT 'waiting',  -- 任务状态: active/waiting/paused/error/complete/removed

    -- 下载信息
    uris TEXT NOT NULL,                       -- 下载链接(JSON数组格式)
    total_length INTEGER DEFAULT 0,          -- 总大小(字节)
    completed_length INTEGER DEFAULT 0,      -- 已完成大小(字节)
    download_speed INTEGER DEFAULT 0,        -- 下载速度(字节/秒)

    -- 文件信息
    download_dir TEXT,                        -- 下载目录
    filename TEXT,                            -- 文件名

    -- 配置选项
    connections INTEGER DEFAULT 16,          -- 连接数
    split INTEGER DEFAULT 5,                 -- 分片数

    -- 时间戳
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### 2. 配置表 (configs)

存储 aria2 配置信息。

```sql
CREATE TABLE configs (
    -- 基础字段
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,               -- 配置名称

    -- aria2 配置
    port INTEGER NOT NULL DEFAULT 6800,     -- RPC 端口
    secret TEXT,                             -- RPC 密钥
    download_dir TEXT NOT NULL,              -- 默认下载目录
    max_concurrent INTEGER DEFAULT 5,       -- 最大并发下载数
    max_connections INTEGER DEFAULT 16,     -- 每任务最大连接数
    split_size TEXT DEFAULT '1M',           -- 分片大小

    -- 路径配置
    aria2_path TEXT NOT NULL,               -- aria2 可执行文件路径

    -- 状态
    is_active BOOLEAN DEFAULT 0,            -- 是否为当前激活配置

    -- 时间戳
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### 3. 统计表 (stats)

记录下载统计信息。

```sql
CREATE TABLE stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    -- 全局统计
    total_download_speed INTEGER DEFAULT 0,  -- 全局下载速度
    active_downloads INTEGER DEFAULT 0,      -- 活跃下载数
    waiting_downloads INTEGER DEFAULT 0,     -- 等待下载数
    completed_downloads INTEGER DEFAULT 0,   -- 已完成下载数

    -- 记录时间
    recorded_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

## 索引设计

```sql
-- 下载任务查询优化
CREATE INDEX idx_downloads_status ON downloads(status);
CREATE INDEX idx_downloads_created_at ON downloads(created_at);
CREATE INDEX idx_downloads_updated_at ON downloads(updated_at);

-- 配置查询优化
CREATE INDEX idx_configs_name ON configs(name);
CREATE INDEX idx_configs_active ON configs(is_active);

-- 统计查询优化
CREATE INDEX idx_stats_recorded_at ON stats(recorded_at);
```

## 数据类型说明

### URIs 字段格式
```json
["http://example.com/file.zip", "ftp://mirror.com/file.zip"]
```

### 状态枚举
- `active`: 正在下载
- `waiting`: 等待中
- `paused`: 已暂停
- `error`: 下载错误
- `complete`: 下载完成
- `removed`: 已删除

## 核心操作接口

### 下载管理
```rust
// 添加下载任务
async fn add_download(gid: &str, uris: Vec<String>, options: DownloadOptions) -> Result<()>

// 更新下载进度
async fn update_progress(gid: &str, completed: i64, speed: i64) -> Result<()>

// 更新下载状态
async fn update_status(gid: &str, status: &str) -> Result<()>

// 查询下载任务
async fn get_download(gid: &str) -> Result<Option<Download>>

// 列出下载任务(按状态筛选)
async fn list_downloads(status: Option<&str>) -> Result<Vec<Download>>

// 删除下载任务
async fn delete_download(gid: &str) -> Result<()>
```

### 配置管理
```rust
// 保存配置
async fn save_config(config: &Aria2Config) -> Result<()>

// 获取激活配置
async fn get_active_config() -> Result<Option<Aria2Config>>

// 设置激活配置
async fn set_active_config(name: &str) -> Result<()>

// 删除配置
async fn delete_config(name: &str) -> Result<()>
```

### 统计信息
```rust
// 记录统计
async fn record_stats(stats: &GlobalStats) -> Result<()>

// 获取最新统计
async fn get_latest_stats() -> Result<Option<GlobalStats>>

// 清理旧统计(保留指定天数)
async fn cleanup_old_stats(days: i32) -> Result<()>
```

## 数据结构定义

```rust
#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Download {
    pub gid: String,
    pub status: String,
    pub uris: String,  // JSON 格式
    pub total_length: i64,
    pub completed_length: i64,
    pub download_speed: i64,
    pub download_dir: Option<String>,
    pub filename: Option<String>,
    pub connections: i32,
    pub split: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Aria2Config {
    pub id: Option<i32>,
    pub name: String,
    pub port: i32,
    pub secret: Option<String>,
    pub download_dir: String,
    pub max_concurrent: i32,
    pub max_connections: i32,
    pub split_size: String,
    pub aria2_path: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct GlobalStats {
    pub id: Option<i32>,
    pub total_download_speed: i64,
    pub active_downloads: i32,
    pub waiting_downloads: i32,
    pub completed_downloads: i32,
    pub recorded_at: String,
}
```

## 特色功能

1. **状态追踪**: 完整的下载状态生命周期管理
2. **多配置支持**: 支持多套 aria2 配置，可快速切换
3. **统计分析**: 记录下载统计，支持历史数据分析
4. **JSON 存储**: 复杂数据(如 URI 列表)使用 JSON 格式存储
5. **自动更新时间戳**: 使用触发器自动更新 updated_at 字段

## 部署说明

1. 数据库文件位置: `~/.burncloud/aria2_downloads.db`
2. 自动创建表结构和索引
3. 支持数据库迁移和版本管理
4. 兼容 Windows 和 Linux 平台

## 性能优化

1. **连接池**: 复用 burncloud-database 的连接池机制
2. **批量操作**: 支持批量插入和更新
3. **索引优化**: 针对常用查询建立索引
4. **数据清理**: 定期清理过期统计数据
5. **JSON 查询**: 利用 SQLite 的 JSON 函数优化查询

这个设计在保持极度精简的同时，提供了 aria2 下载管理所需的全部核心功能。