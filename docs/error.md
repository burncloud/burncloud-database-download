# error.rs - 错误处理模块

## 概述

`error.rs` 模块定义了 `burncloud-database-download` 库中使用的错误类型和错误处理机制。该模块使用 `thiserror` 库来提供结构化的错误处理。

## 主要组件

### DownloadDbError 枚举

这是库的主要错误类型，涵盖了所有可能的错误情况：

#### 错误变体

1. **Database**
   - **描述**: 来自底层数据库操作的错误
   - **类型**: `burncloud_database::DatabaseError`
   - **错误消息**: `"Database error: {0}"`
   - **用途**: 包装数据库连接和操作相关的错误

2. **Sqlx**
   - **描述**: 来自 SQLx 数据库驱动的错误
   - **类型**: `sqlx::Error`
   - **错误消息**: `"SQLx error: {0}"`
   - **用途**: 包装 SQL 查询执行错误

3. **TaskNotFound**
   - **描述**: 指定的下载任务未找到
   - **类型**: `String` (任务ID)
   - **错误消息**: `"Task not found: {0}"`
   - **用途**: 当查询不存在的任务时抛出

4. **Serialization**
   - **描述**: JSON 序列化/反序列化错误
   - **类型**: `serde_json::Error`
   - **错误消息**: `"Serialization error: {0}"`
   - **用途**: 处理状态信息的 JSON 转换错误

5. **InvalidStatus**
   - **描述**: 无效的下载状态
   - **类型**: `String` (状态描述)
   - **错误消息**: `"Invalid status: {0}"`
   - **用途**: 当遇到无法识别的状态值时抛出

6. **Other**
   - **描述**: 其他通用错误
   - **类型**: `String` (错误描述)
   - **错误消息**: `"{0}"`
   - **用途**: 包装其他未分类的错误

### 错误转换

该模块实现了自动错误转换（使用 `#[from]` 属性）：
- `burncloud_database::DatabaseError` → `DownloadDbError::Database`
- `sqlx::Error` → `DownloadDbError::Sqlx`
- `serde_json::Error` → `DownloadDbError::Serialization`

### 类型别名

```rust
pub type Result<T> = std::result::Result<T, DownloadDbError>;
```

提供了便捷的结果类型，用于整个库的错误处理。

## 使用示例

### 基本错误处理

```rust
use burncloud_database_download::{DownloadDbError, Result};

async fn example_function() -> Result<()> {
    // 如果操作失败，会自动转换为相应的错误类型
    // 例如：数据库错误、序列化错误等
    Ok(())
}
```

### 错误匹配

```rust
match result {
    Ok(task) => {
        // 处理成功情况
    }
    Err(DownloadDbError::TaskNotFound(id)) => {
        println!("任务未找到: {}", id);
    }
    Err(DownloadDbError::Database(db_err)) => {
        println!("数据库错误: {}", db_err);
    }
    Err(e) => {
        println!("其他错误: {}", e);
    }
}
```

### 创建自定义错误

```rust
use burncloud_database_download::DownloadDbError;

// 创建任务未找到错误
let error = DownloadDbError::TaskNotFound("task-123".to_string());

// 创建无效状态错误
let error = DownloadDbError::InvalidStatus("unknown".to_string());

// 创建通用错误
let error = DownloadDbError::Other("Something went wrong".to_string());
```

## 错误传播

该模块的错误类型支持自动错误传播，可以使用 `?` 操作符：

```rust
async fn save_task(repo: &DownloadRepository, task: &DownloadTask) -> Result<()> {
    // 以下操作的错误会自动转换为 DownloadDbError
    repo.initialize().await?;  // 数据库错误
    repo.save_task(task).await?;  // SQLx 错误或序列化错误
    Ok(())
}
```

## 文件位置

`src/error.rs:1-24`

## 相关文件

- `src/lib.rs` - 导出错误类型
- `src/repository.rs` - 使用这些错误类型
- `src/models.rs` - 可能产生序列化错误