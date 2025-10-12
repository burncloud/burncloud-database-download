# repository.rs - 数据仓库模块

## 概述

`repository.rs` 模块实现了下载任务的数据仓库层，提供了对下载任务和进度数据的完整 CRUD（创建、读取、更新、删除）操作。该模块是数据持久化的核心实现。

## 主要组件

### DownloadRepository 结构体

下载任务数据库仓库的主要实现，封装了所有数据库操作。

#### 字段

- **db**: `Database` - 数据库连接管理器

#### 构造函数

##### `new(db: Database) -> Self`

**功能**: 创建新的仓库实例

**参数**:
- `db`: 数据库连接管理器

**返回**: 新的仓库实例

**使用示例**:
```rust
let db = Database::new("download.db").await?;
let repo = DownloadRepository::new(db);
```

## 核心方法

### 初始化方法

##### `initialize(&self) -> Result<()>`

**功能**: 初始化数据库schema

**位置**: `src/repository.rs:22-24`

**实现细节**:
- 调用 `schema::initialize_schema` 函数
- 创建必要的数据库表和索引

**使用场景**: 应用启动时调用，确保数据库结构正确

## 任务操作方法

### 保存任务

##### `save_task(&self, task: &DownloadTask) -> Result<()>`

**功能**: 保存或更新下载任务

**位置**: `src/repository.rs:27-51`

**参数**:
- `task`: 要保存的下载任务

**SQL操作**: 使用 `INSERT ... ON CONFLICT ... DO UPDATE` 实现 upsert 操作

**实现细节**:
- 将业务对象转换为数据库记录
- 支持任务更新（基于任务ID冲突处理）
- 自动更新时间戳

### 获取单个任务

##### `get_task(&self, task_id: &TaskId) -> Result<DownloadTask>`

**功能**: 根据ID获取特定下载任务

**位置**: `src/repository.rs:54-69`

**参数**:
- `task_id`: 要查询的任务ID

**返回**: 找到的下载任务

**错误处理**:
- `RowNotFound` → `TaskNotFound`
- 其他SQLx错误直接传播

**实现细节**:
- 执行精确的ID匹配查询
- 自动转换数据库记录为业务对象

### 获取任务列表

##### `list_tasks(&self) -> Result<Vec<DownloadTask>>`

**功能**: 获取所有下载任务

**位置**: `src/repository.rs:72-82`

**返回**: 所有任务的列表，按创建时间倒序排列

**实现细节**:
- 查询所有任务记录
- 按 `created_at DESC` 排序
- 批量转换为业务对象

### 按状态筛选任务

##### `list_tasks_by_status(&self, status: &DownloadStatus) -> Result<Vec<DownloadTask>>`

**功能**: 根据状态筛选下载任务

**位置**: `src/repository.rs:85-98`

**参数**:
- `status`: 要筛选的下载状态

**实现细节**:
- 将状态对象序列化为JSON进行匹配
- 按创建时间倒序返回结果

**使用示例**:
```rust
let running_tasks = repo.list_tasks_by_status(&DownloadStatus::Running).await?;
```

### 删除任务

##### `delete_task(&self, task_id: &TaskId) -> Result<()>`

**功能**: 删除指定的下载任务

**位置**: `src/repository.rs:101-110`

**参数**:
- `task_id`: 要删除的任务ID

**实现细节**:
- 直接从数据库删除记录
- 相关的进度记录会因外键约束被级联删除

## 进度操作方法

### 保存进度

##### `save_progress(&self, task_id: &TaskId, progress: &DownloadProgress) -> Result<()>`

**功能**: 保存或更新下载进度

**位置**: `src/repository.rs:113-137`

**参数**:
- `task_id`: 关联的任务ID
- `progress`: 下载进度信息

**SQL操作**: 使用 `INSERT ... ON CONFLICT ... DO UPDATE` 实现 upsert 操作

**实现细节**:
- 每个任务只保存最新的进度记录
- 自动更新时间戳

### 获取进度

##### `get_progress(&self, task_id: &TaskId) -> Result<DownloadProgress>`

**功能**: 获取任务的下载进度

**位置**: `src/repository.rs:141-156`

**参数**:
- `task_id`: 要查询进度的任务ID

**返回**: 任务的下载进度

**错误处理**:
- `RowNotFound` → `TaskNotFound`

### 删除进度

##### `delete_progress(&self, task_id: &TaskId) -> Result<()>`

**功能**: 删除任务的进度记录

**位置**: `src/repository.rs:159-168`

**参数**:
- `task_id`: 要删除进度的任务ID

## 统计方法

### 任务计数

##### `count_tasks(&self) -> Result<i64>`

**功能**: 获取任务总数

**位置**: `src/repository.rs:171-177`

**返回**: 数据库中任务的总数量

### 按状态统计

##### `count_tasks_by_status(&self) -> Result<Vec<(String, i64)>>`

**功能**: 获取按状态分组的任务数量统计

**位置**: `src/repository.rs:180-188`

**返回**: 状态字符串和对应数量的元组列表

**实现细节**:
- 使用 `GROUP BY status` 进行分组统计
- 返回状态的JSON字符串形式

**使用示例**:
```rust
let stats = repo.count_tasks_by_status().await?;
for (status, count) in stats {
    println!("状态 {}: {} 个任务", status, count);
}
```

## 管理方法

### 清空数据

##### `clear_all(&self) -> Result<()>`

**功能**: 清空所有下载任务和进度数据

**位置**: `src/repository.rs:191-201`

**实现细节**:
- 先删除进度记录，再删除任务记录
- 避免外键约束错误

**警告**: 这是危险操作，会删除所有数据

## 测试模块

模块包含全面的测试覆盖所有主要功能：

### 辅助函数

##### `setup_repo() -> DownloadRepository`

**功能**: 创建测试用的内存数据库仓库

**实现**:
- 创建内存数据库
- 初始化schema
- 返回可用的仓库实例

### 测试用例

1. **`test_save_and_get_task()`**: 测试任务保存和检索
2. **`test_list_tasks()`**: 测试任务列表功能
3. **`test_delete_task()`**: 测试任务删除功能
4. **`test_save_and_get_progress()`**: 测试进度保存和检索
5. **`test_count_tasks()`**: 测试任务计数功能
6. **`test_clear_all()`**: 测试清空所有数据功能

## 事务处理

虽然当前实现没有显式的事务管理，但所有单个操作都是原子的：
- 单个SQL语句具有原子性
- UPSERT操作保证数据一致性
- 外键约束确保引用完整性

## 性能考虑

1. **索引使用**: 依赖schema模块创建的索引提升查询性能
2. **批量操作**: `list_tasks`等方法一次性获取所有数据
3. **连接复用**: 通过Database连接管理器复用连接

## 使用模式

### 基本使用流程

```rust
// 1. 创建仓库
let repo = DownloadRepository::new(database);

// 2. 初始化
repo.initialize().await?;

// 3. 保存任务
let task = DownloadTask::new(url, path);
repo.save_task(&task).await?;

// 4. 更新进度
let progress = DownloadProgress { /* ... */ };
repo.save_progress(&task.id, &progress).await?;

// 5. 查询任务
let retrieved_task = repo.get_task(&task.id).await?;
```

### 错误处理模式

所有方法都返回 `Result<T>`，支持使用 `?` 操作符进行错误传播：

```rust
async fn download_workflow() -> Result<()> {
    let repo = setup_repository()?;
    repo.initialize().await?;

    let task = create_task()?;
    repo.save_task(&task).await?;

    // ... 其他操作
    Ok(())
}
```

## 文件位置

`src/repository.rs:1-329`

## 相关文件

- `src/models.rs` - 提供数据模型和转换方法
- `src/error.rs` - 定义错误类型
- `src/schema.rs` - 提供数据库初始化功能