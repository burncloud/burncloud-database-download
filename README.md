# burncloud-database-download

下载任务数据库持久化层,负责将 `burncloud-download` 的下载任务数据存储到 SQLite 数据库中。

## 功能特性

- ✅ 下载任务持久化 (任务信息、状态、进度)
- ✅ 支持任务查询、更新、删除
- ✅ 按状态筛选任务
- ✅ 任务统计功能
- ✅ 完整的数据库 schema 管理
- ✅ 与 `burncloud-database-core` 无缝集成

## 快速开始

### 添加依赖

```toml
[dependencies]
burncloud-database-download = { path = "../burncloud-database-download" }
```

### 基本使用

```rust
use burncloud_database_download::{DownloadRepository, DownloadTask, DownloadProgress};
use burncloud_database_core::create_in_memory_database;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 创建数据库连接
    let db = create_in_memory_database().await?;

    // 2. 创建仓库并初始化 schema
    let repo = DownloadRepository::new(db);
    repo.initialize().await?;

    // 3. 创建并保存下载任务
    let task = DownloadTask::new(
        "https://example.com/file.zip".to_string(),
        PathBuf::from("/downloads/file.zip")
    );
    repo.save_task(&task).await?;

    // 4. 保存下载进度
    let progress = DownloadProgress {
        downloaded_bytes: 5120,
        total_bytes: Some(10240),
        speed_bps: 1024,
        eta_seconds: Some(5),
    };
    repo.save_progress(&task.id, &progress).await?;

    // 5. 查询任务和进度
    let retrieved_task = repo.get_task(&task.id).await?;
    let retrieved_progress = repo.get_progress(&task.id).await?;

    println!("Task: {} - {}", retrieved_task.url, retrieved_task.status);
    println!("Progress: {:.1}%",
        retrieved_progress.completion_percentage().unwrap_or(0.0)
    );

    Ok(())
}
```

## 数据库 Schema

### download_tasks 表

存储下载任务的基本信息:

| 字段 | 类型 | 说明 |
|------|------|------|
| id | TEXT | 任务ID (UUID) |
| url | TEXT | 下载URL |
| target_path | TEXT | 目标文件路径 |
| status | TEXT | 任务状态 (JSON) |
| created_at | INTEGER | 创建时间 (Unix timestamp) |
| updated_at | INTEGER | 更新时间 (Unix timestamp) |

### download_progress 表

存储下载进度信息:

| 字段 | 类型 | 说明 |
|------|------|------|
| task_id | TEXT | 任务ID (外键) |
| downloaded_bytes | INTEGER | 已下载字节数 |
| total_bytes | INTEGER | 总字节数 (可选) |
| speed_bps | INTEGER | 下载速度 (bytes/second) |
| eta_seconds | INTEGER | 预计剩余时间 (秒) |
| updated_at | INTEGER | 更新时间 (Unix timestamp) |

## API 文档

### DownloadRepository

主要方法:

- `initialize()` - 初始化数据库 schema
- `save_task(&task)` - 保存/更新任务
- `get_task(&task_id)` - 获取任务
- `list_tasks()` - 列出所有任务
- `list_tasks_by_status(&status)` - 按状态筛选任务
- `delete_task(&task_id)` - 删除任务
- `save_progress(&task_id, &progress)` - 保存进度
- `get_progress(&task_id)` - 获取进度
- `count_tasks()` - 统计任务数量
- `clear_all()` - 清空所有数据

## 运行示例

```bash
cd burncloud-database-download
cargo run --example basic_usage
```

## 运行测试

```bash
cd burncloud-database-download
cargo test --lib -- --test-threads=1
```

## 测试结果

✅ 所有 10 个单元测试通过:

- ✅ 任务记录序列化/反序列化
- ✅ 进度记录序列化/反序列化
- ✅ 任务保存和查询
- ✅ 任务列表和删除
- ✅ 进度保存和查询
- ✅ 任务统计
- ✅ Schema 初始化
- ✅ SQL 语法验证
- ✅ 清空数据

## 架构设计

```
burncloud-database-download
├── models.rs         # 数据模型和序列化
├── repository.rs     # 数据访问层
├── schema.rs         # 数据库 schema 定义
├── error.rs          # 错误类型
└── lib.rs            # 公共接口
```

## 依赖关系

- `burncloud-database-core` - 数据库核心功能
- `burncloud-download` - 下载任务类型定义
- `sqlx` - SQLite 数据库驱动
- `serde` / `serde_json` - 序列化
- `chrono` - 时间处理

## License

MIT