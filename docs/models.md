# models.rs - 数据模型模块

## 概述

`models.rs` 模块定义了用于数据库持久化的数据结构。该模块包含了下载任务和下载进度的数据库记录模型，以及它们与业务对象之间的转换方法。

## 主要组件

### DownloadTaskRecord 结构体

下载任务的数据库记录模型，用于在 SQLite 数据库中存储下载任务信息。

#### 字段说明

- **id**: `String` - 任务ID (UUID字符串)
- **url**: `String` - 下载URL
- **target_path**: `String` - 目标文件路径
- **status**: `String` - 任务状态 (JSON序列化)
- **created_at**: `i64` - 创建时间 (Unix timestamp)
- **updated_at**: `i64` - 更新时间 (Unix timestamp)

#### 派生特性

- `Debug`, `Clone` - 基本调试和克隆支持
- `Serialize`, `Deserialize` - JSON 序列化支持
- `FromRow` - SQLx 行映射支持

#### 方法

##### `from_task(task: &DownloadTask) -> serde_json::Result<Self>`

**功能**: 从业务对象 `DownloadTask` 创建数据库记录

**参数**:
- `task`: 要转换的下载任务对象

**返回**:
- `serde_json::Result<Self>` - 转换结果，可能失败于状态序列化

**实现细节**:
- 自动设置当前时间为创建和更新时间
- 将任务ID转换为字符串
- 将文件路径转换为字符串
- 将状态序列化为JSON字符串

**使用示例**:
```rust
// 注意：DownloadTask::new 在碰到重复的下载链接时，会直接返回已存在的 task-id
let task = DownloadTask::new("https://example.com/file.zip", "/downloads/file.zip");
let record = DownloadTaskRecord::from_task(&task)?;
```

##### `to_task(&self) -> Result<DownloadTask, String>`

**功能**: 将数据库记录转换为业务对象 `DownloadTask`

**返回**:
- `Result<DownloadTask, String>` - 转换结果，可能失败于反序列化

**实现细节**:
- 反序列化状态JSON
- 解析任务ID
- 转换Unix时间戳为SystemTime
- 构建完整的DownloadTask对象

**错误处理**:
- 状态解析失败："Failed to parse status: {error}"
- 任务ID无效："Invalid task ID: {error}"

### DownloadProgressRecord 结构体

下载进度的数据库记录模型，用于跟踪下载任务的实时进度信息。

#### 字段说明

- **task_id**: `String` - 任务ID (UUID字符串)
- **downloaded_bytes**: `i64` - 已下载字节数
- **total_bytes**: `Option<i64>` - 总字节数 (可选)
- **speed_bps**: `i64` - 下载速度 (bytes/second)
- **eta_seconds**: `Option<i64>` - 预计剩余时间 (秒, 可选)
- **updated_at**: `i64` - 更新时间 (Unix timestamp)

#### 派生特性

- `Debug`, `Clone` - 基本调试和克隆支持
- `Serialize`, `Deserialize` - JSON 序列化支持
- `FromRow` - SQLx 行映射支持

#### 方法

##### `from_progress(task_id: &TaskId, progress: &DownloadProgress) -> Self`

**功能**: 从任务ID和进度对象创建数据库记录

**参数**:
- `task_id`: 关联的任务ID
- `progress`: 下载进度对象

**返回**:
- `Self` - 新的进度记录

**实现细节**:
- 自动设置当前时间为更新时间
- 将u64数值转换为i64以适应数据库
- 处理可选字段的None值

**使用示例**:
```rust
let task_id = TaskId::new();
let progress = DownloadProgress {
    downloaded_bytes: 1024,
    total_bytes: Some(10240),
    speed_bps: 512,
    eta_seconds: Some(18),
};
let record = DownloadProgressRecord::from_progress(&task_id, &progress);
```

##### `to_progress(&self) -> DownloadProgress`

**功能**: 将数据库记录转换为业务进度对象

**返回**:
- `DownloadProgress` - 进度对象

**实现细节**:
- 将i64数值转换回u64
- 保持可选字段的结构
- 简单的类型转换，不会失败

## 测试模块

模块包含全面的单元测试以确保转换的正确性：

### `test_download_task_record_roundtrip()`

**功能**: 测试下载任务记录的往返转换

**测试内容**:
- 创建DownloadTask对象
- 转换为DownloadTaskRecord
- 再转换回DownloadTask
- 验证关键字段的一致性

### `test_download_progress_record_roundtrip()`

**功能**: 测试下载进度记录的往返转换

**测试内容**:
- 创建DownloadProgress对象
- 转换为DownloadProgressRecord
- 再转换回DownloadProgress
- 验证所有字段的一致性

## 数据转换说明

### 时间处理

- **存储**: Unix时间戳 (i64)
- **业务对象**: SystemTime
- **转换**: 使用 `UNIX_EPOCH + Duration::from_secs()`

### 数值类型转换

- **数据库**: i64 (SQLite整数类型)
- **业务对象**: u64 (更符合字节数语义)
- **转换**: 直接 as 类型转换

### 路径处理

- **存储**: String
- **业务对象**: PathBuf
- **转换**:
  - 存储时: `path.to_string_lossy().to_string()`
  - 读取时: `path_string.into()`

### 状态序列化

- **存储**: JSON字符串
- **业务对象**: DownloadStatus枚举
- **转换**: serde_json序列化/反序列化

## 使用注意事项

1. **时间精度**: Unix时间戳精度为秒，不支持亚秒级精度
2. **数值范围**: 注意u64到i64的转换，超大值可能溢出
3. **路径编码**: 使用`to_string_lossy`可能在非UTF-8路径上丢失信息
4. **状态完整性**: 状态序列化依赖serde_json，版本兼容性需要注意

## 文件位置

`src/models.rs:1-142`

## 相关文件

- `src/repository.rs` - 使用这些模型进行数据库操作
- `src/error.rs` - 处理转换过程中的错误
- `src/schema.rs` - 定义对应的数据库表结构