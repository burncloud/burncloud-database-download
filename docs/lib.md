# lib.rs - 库入口文件

## 概述

`lib.rs` 是 `burncloud-database-download` 库的主入口文件，负责将下载任务数据存储到 SQLite 数据库中。这个模块提供了下载任务的持久化层实现。

## 主要功能

该文件主要负责：
- 定义库的模块结构
- 导出公共接口
- 重新导出核心依赖

## 模块结构

### 导出的模块

- `error` - 错误处理模块
- `models` - 数据模型定义
- `repository` - 数据仓库实现
- `schema` - 数据库模式定义

### 公共类型导出

#### 错误类型
- `DownloadDbError` - 下载数据库操作的错误类型
- `Result` - 带有 `DownloadDbError` 的结果类型

#### 数据模型
- `DownloadTaskRecord` - 下载任务的数据库记录
- `DownloadProgressRecord` - 下载进度的数据库记录

#### 仓库
- `DownloadRepository` - 下载任务数据库仓库

### 重新导出的依赖

#### 数据库核心
- `Database` - 数据库连接管理器 (来自 `burncloud_database`)
- `DatabaseConnection` - 数据库连接 (来自 `burncloud_database`)
- `DbCoreResult` - 数据库核心结果类型 (来自 `burncloud_database`)

#### 下载类型
- `TaskId` - 任务唯一标识符 (来自 `burncloud_download_types`)
- `DownloadStatus` - 下载状态枚举 (来自 `burncloud_download_types`)
- `DownloadProgress` - 下载进度结构 (来自 `burncloud_download_types`)
- `DownloadTask` - 下载任务结构 (来自 `burncloud_download_types`)

## 使用示例

```rust
use burncloud_database_download::{
    DownloadRepository,
    Database,
    DownloadTask,
    Result,
};

async fn example() -> Result<()> {
    // 创建数据库连接
    let db = Database::new("download.db").await?;

    // 创建仓库实例
    let repo = DownloadRepository::new(db);

    // 初始化数据库模式
    repo.initialize().await?;

    // 使用仓库进行操作...
    Ok(())
}
```

## 文件位置

`src/lib.rs`

## 相关文件

- `src/error.rs` - 错误处理定义
- `src/models.rs` - 数据模型实现
- `src/repository.rs` - 数据仓库实现
- `src/schema.rs` - 数据库模式定义