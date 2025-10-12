# burncloud-database-download 项目文档

## 文档目录

本项目的文档分为以下几个部分：

### 📋 [架构总览](./architecture_overview.md)
- 项目整体架构设计
- 模块依赖关系
- 数据流转图
- 错误处理策略
- 性能优化方案
- 测试策略
- 部署和监控
- 扩展规划

### 🗃️ [数据模型流程图](./models_flowcharts.md)
- `DownloadTaskRecord` 转换函数流程图
- `DownloadProgressRecord` 转换函数流程图
- 类型安全转换机制
- 错误处理策略
- 时间处理机制

### 🏗️ [数据库架构流程图](./schema_flowcharts.md)
- `initialize_schema` 函数流程图
- 数据库表结构设计
- 表关系图
- 索引策略
- 数据库约束
- 初始化流程关键点

### 📚 [数据仓库流程图](./repository_flowcharts.md)
- 构造和初始化函数
- 任务管理函数
- 进度管理函数
- 统计和管理函数
- Repository 模式
- UPSERT 模式
- 错误传播链
- 类型转换安全

## 快速导航

### 按功能模块浏览
- **数据模型**: [models_flowcharts.md](./models_flowcharts.md)
- **数据库架构**: [schema_flowcharts.md](./schema_flowcharts.md)
- **数据访问层**: [repository_flowcharts.md](./repository_flowcharts.md)

### 按关注点浏览
- **架构设计**: [architecture_overview.md](./architecture_overview.md)
- **函数实现**: 各模块流程图文档
- **错误处理**: 所有文档中的错误处理部分
- **性能优化**: [architecture_overview.md](./architecture_overview.md#性能优化策略)

## 文档约定

### 流程图说明
- 🟢 绿色节点：成功结束
- 🔴 红色节点：错误结束
- 🔵 蓝色节点：开始节点
- 🟡 黄色节点：决策节点

### Mermaid 图表类型
- `flowchart`: 流程图
- `erDiagram`: 实体关系图
- `graph`: 关系图

## 如何使用这些文档

1. **新团队成员**: 从 [架构总览](./architecture_overview.md) 开始了解项目整体设计
2. **功能开发**: 参考对应模块的流程图文档了解具体实现
3. **问题排查**: 查看错误处理流程图和类型转换说明
4. **性能优化**: 参考架构文档中的优化策略部分

## 项目概述

`burncloud-database-download` 是一个专门用于下载任务数据库持久化的 Rust 库，负责将下载任务数据存储到 SQLite 数据库中。该库提供了完整的下载任务和进度跟踪功能的数据持久化解决方案。

### 项目结构

```
src/
├── lib.rs          # 库入口文件，模块导出和依赖重新导出
├── error.rs        # 错误处理定义，包含所有错误类型
├── models.rs       # 数据模型，数据库记录与业务对象的转换
├── repository.rs   # 数据仓库，核心CRUD操作实现
└── schema.rs       # 数据库模式，表结构和索引定义

tests/
├── schema_tests.rs    # 数据库架构测试
├── models_tests.rs    # 数据模型测试
└── repository_tests.rs # 数据仓库测试

docs/
├── README.md                    # 本文档，项目和文档总览
├── architecture_overview.md     # 项目架构总览
├── models_flowcharts.md        # 数据模型流程图
├── schema_flowcharts.md        # 数据库架构流程图
└── repository_flowcharts.md    # 数据仓库流程图
```

## 主要功能特性

### 下载任务管理
- ✅ 任务创建和保存
- ✅ 重复链接检测：`DownloadTask::new` 函数在碰到重复的下载链接时，会直接返回已存在的 task-id
- ✅ 任务状态跟踪
- ✅ 任务查询和列表
- ✅ 按状态筛选任务
- ✅ 任务删除操作

### 下载进度跟踪
- ✅ 实时进度保存
- ✅ 下载速度记录
- ✅ 预计剩余时间
- ✅ 进度历史查询

### 数据持久化
- ✅ SQLite 数据库支持
- ✅ 事务安全操作
- ✅ 外键约束保证数据一致性
- ✅ 性能优化索引

### 错误处理
- ✅ 结构化错误类型
- ✅ 自动错误转换
- ✅ 详细错误信息
- ✅ 优雅的错误传播

## 重复链接处理机制

### DownloadTask::new 重复检测

`DownloadTask::new` 函数实现了智能的重复链接检测机制：

1. **重复检测逻辑**: 在创建新任务时，会检查数据库中是否已存在相同的下载链接
2. **返回行为**:
   - 如果是新链接：创建新的任务，返回新的 task-id
   - 如果是重复链接：直接返回已存在任务的 task-id，不创建重复任务
3. **比较标准**: 基于完整的下载 URL 进行匹配
4. **性能优化**: 使用数据库索引确保快速的重复检测

### 使用示例

```rust
// 第一次调用 - 创建新任务
let task1 = DownloadTask::new(
    "https://example.com/file.zip".to_string(),
    PathBuf::from("/downloads/file.zip")
);
println!("Task ID: {}", task1.id); // 输出: 新的 UUID

// 第二次调用相同 URL - 返回已存在的任务
let task2 = DownloadTask::new(
    "https://example.com/file.zip".to_string(),  // 相同的 URL
    PathBuf::from("/downloads/another_path.zip") // 不同的路径也会返回相同任务
);
println!("Task ID: {}", task2.id); // 输出: 与 task1.id 相同的 UUID
```

## 快速开始

### 1. 添加依赖

```toml
[dependencies]
burncloud-database-download = { path = "path/to/burncloud-database-download" }
burncloud-database = { path = "path/to/burncloud-database" }
burncloud-download-types = { path = "path/to/burncloud-download-types" }
tokio = { version = "1.0", features = ["full"] }
```

### 2. 基本使用

```rust
use burncloud_database_download::{
    DownloadRepository,
    Database,
    DownloadTask,
    DownloadProgress,
    Result
};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    // 创建数据库连接
    let db = Database::new("downloads.db").await?;

    // 创建仓库实例
    let repo = DownloadRepository::new(db);

    // 初始化数据库模式
    repo.initialize().await?;

    // 创建下载任务
    // 注意：DownloadTask::new 在碰到重复的下载链接时，会直接返回已存在的 task-id
    let task = DownloadTask::new(
        "https://example.com/file.zip".to_string(),
        PathBuf::from("/downloads/file.zip")
    );

    // 保存任务
    repo.save_task(&task).await?;

    // 更新进度
    let progress = DownloadProgress {
        downloaded_bytes: 1024,
        total_bytes: Some(10240),
        speed_bps: 512,
        eta_seconds: Some(18),
    };
    repo.save_progress(&task.id, &progress).await?;

    // 查询任务
    let retrieved_task = repo.get_task(&task.id).await?;
    println!("任务URL: {}", retrieved_task.url);

    // 获取所有任务
    let all_tasks = repo.list_tasks().await?;
    println!("总任务数: {}", all_tasks.len());

    Ok(())
}
```

### 3. 错误处理

```rust
use burncloud_database_download::{DownloadDbError, Result};

async fn handle_errors() -> Result<()> {
    match repo.get_task(&task_id).await {
        Ok(task) => {
            println!("找到任务: {}", task.url);
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
    Ok(())
}
```

## 数据库设计

### 表结构

#### download_tasks 表
| 字段 | 类型 | 说明 |
|------|------|------|
| id | TEXT | 任务UUID (主键) |
| url | TEXT | 下载URL |
| target_path | TEXT | 目标路径 |
| status | TEXT | 状态JSON |
| created_at | INTEGER | 创建时间戳 |
| updated_at | INTEGER | 更新时间戳 |

#### download_progress 表
| 字段 | 类型 | 说明 |
|------|------|------|
| task_id | TEXT | 任务ID (主键/外键) |
| downloaded_bytes | INTEGER | 已下载字节 |
| total_bytes | INTEGER | 总字节数 (可选) |
| speed_bps | INTEGER | 下载速度 |
| eta_seconds | INTEGER | 剩余时间 (可选) |
| updated_at | INTEGER | 更新时间戳 |

### 性能优化

- ✅ 状态字段索引 - 优化状态筛选
- ✅ 创建时间索引 - 优化时间排序
- ✅ 更新时间索引 - 支持时间查询
- ✅ 外键约束 - 保证数据一致性

## 测试覆盖

每个模块都包含全面的单元测试：

- ✅ **models.rs**: 数据转换往返测试
- ✅ **repository.rs**: CRUD操作功能测试
- ✅ **schema.rs**: 数据库初始化测试
- ✅ **error.rs**: 错误处理验证

运行测试：
```bash
cargo test
```

## 性能特性

### 查询优化
- 使用索引加速常用查询
- 批量操作减少数据库往返
- 连接池复用数据库连接

### 内存效率
- 流式处理大量数据
- 最小化内存分配
- 高效的序列化/反序列化

### 并发安全
- 所有操作都是线程安全的
- 支持多个并发连接
- 事务保证数据一致性

## 最佳实践

### 1. 错误处理
```rust
// 使用 ? 操作符进行错误传播
async fn download_workflow() -> Result<()> {
    let repo = setup_repository()?;
    repo.initialize().await?;
    // ... 其他操作
    Ok(())
}
```

### 2. 资源管理
```rust
// 确保正确初始化
let repo = DownloadRepository::new(database);
repo.initialize().await?; // 必须调用
```

### 3. 批量操作
```rust
// 批量查询比循环单个查询更高效
let all_tasks = repo.list_tasks().await?;
let running_tasks = repo.list_tasks_by_status(&DownloadStatus::Running).await?;
```

## 扩展指南

### 添加新字段
1. 修改 schema.rs 中的表定义
2. 更新 models.rs 中的结构体
3. 调整转换方法
4. 添加相应测试

### 自定义查询
1. 在 repository.rs 中添加新方法
2. 实现相应的 SQL 查询
3. 处理错误情况
4. 编写测试用例

### 性能优化
1. 分析查询模式
2. 添加相应索引
3. 优化 SQL 语句
4. 监控性能指标

## 故障排除

### 常见问题

1. **数据库初始化失败**
   - 检查文件权限
   - 确保路径存在
   - 验证 SQLite 版本

2. **任务保存失败**
   - 检查数据格式
   - 验证必填字段
   - 查看错误详情

3. **查询性能问题**
   - 检查索引使用
   - 分析查询计划
   - 考虑数据量大小

### 调试技巧

```rust
// 启用详细日志
env_logger::init();

// 检查数据库状态
let count = repo.count_tasks().await?;
println!("当前任务数: {}", count);

// 验证数据完整性
let stats = repo.count_tasks_by_status().await?;
for (status, count) in stats {
    println!("状态 {}: {} 个任务", status, count);
}
```

## 版本历史

- **v0.1.0**: 初始版本，基本功能实现

## 贡献指南

1. Fork 项目
2. 创建功能分支
3. 编写测试
4. 提交变更
5. 创建 Pull Request

## 许可证

本项目使用 MIT 许可证。

---

**注意**: 这是一个内部库文档，专为 burncloud 项目设计。如有问题或建议，请联系开发团队。