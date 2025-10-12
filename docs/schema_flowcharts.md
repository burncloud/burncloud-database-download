# 数据库架构初始化流程图

## initialize_schema 函数流程图

```mermaid
flowchart TD
    A[开始: initialize_schema] --> B[执行 CREATE_DOWNLOAD_TASKS_TABLE]
    B --> C{任务表创建成功?}
    C -->|是| D[执行 CREATE_DOWNLOAD_PROGRESS_TABLE]
    C -->|否| E[返回数据库错误]
    D --> F{进度表创建成功?}
    F -->|是| G[执行 CREATE_INDEXES]
    F -->|否| E
    G --> H{索引创建成功?}
    H -->|是| I[返回 Ok(())]
    H -->|否| E
    E --> J[结束: Error]
    I --> K[结束: Success]

    style A fill:#e1f5fe
    style K fill:#c8e6c9
    style J fill:#ffcdd2
```

## 数据库表结构

### download_tasks 表设计

```mermaid
erDiagram
    download_tasks {
        TEXT id PK "任务ID (UUID字符串)"
        TEXT url "下载URL"
        TEXT target_path "目标文件路径"
        TEXT status "任务状态 (JSON序列化)"
        INTEGER created_at "创建时间 (Unix时间戳)"
        INTEGER updated_at "更新时间 (Unix时间戳)"
    }
```

### download_progress 表设计

```mermaid
erDiagram
    download_progress {
        TEXT task_id PK,FK "任务ID (外键)"
        INTEGER downloaded_bytes "已下载字节数"
        INTEGER total_bytes "总字节数 (可选)"
        INTEGER speed_bps "下载速度 (bytes/second)"
        INTEGER eta_seconds "预计剩余时间 (秒, 可选)"
        INTEGER updated_at "更新时间 (Unix时间戳)"
    }
```

### 表关系图

```mermaid
erDiagram
    download_tasks ||--o| download_progress : "一对一关系"

    download_tasks {
        TEXT id PK
        TEXT url
        TEXT target_path
        TEXT status
        INTEGER created_at
        INTEGER updated_at
    }

    download_progress {
        TEXT task_id PK,FK
        INTEGER downloaded_bytes
        INTEGER total_bytes
        INTEGER speed_bps
        INTEGER eta_seconds
        INTEGER updated_at
    }
```

## 索引策略

### 性能优化索引

```mermaid
flowchart LR
    A[download_tasks 表] --> B[idx_download_tasks_status]
    A --> C[idx_download_tasks_created_at]
    A --> D[idx_download_tasks_updated_at]
    A --> E[idx_download_tasks_url]

    B --> F[按状态查询优化]
    C --> G[按创建时间排序优化]
    D --> H[按更新时间查询优化]
    E --> I[URL 唯一性检查优化]
```

## 数据库约束

### 外键约束
- `download_progress.task_id` → `download_tasks.id`
- 级联删除: `ON DELETE CASCADE`

### 数据完整性
- 主键约束确保记录唯一性
- 非空约束确保关键字段存在
- 外键约束确保引用完整性

## 初始化流程关键点

1. **表创建顺序**: 必须先创建主表 (download_tasks)，再创建从表 (download_progress)
2. **错误处理**: 任何步骤失败都会立即返回错误，不会继续执行
3. **幂等性**: 使用 `IF NOT EXISTS` 确保重复执行不会出错
4. **性能考虑**: 在表创建后立即创建索引，优化后续查询性能