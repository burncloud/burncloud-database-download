# schema.rs - 数据库模式定义模块

## 概述

`schema.rs` 模块定义了下载任务数据库的SQL模式，包括表结构、索引和初始化函数。该模块负责创建和维护数据库的结构定义。

## 主要组件

### 数据库表定义

#### download_tasks 表

##### `CREATE_DOWNLOAD_TASKS_TABLE`

**功能**: 创建下载任务主表的SQL语句

**位置**: `src/schema.rs:4-13`

**表结构**:

| 字段名 | 类型 | 约束 | 描述 |
|--------|------|------|------|
| id | TEXT | PRIMARY KEY NOT NULL | 任务唯一标识符 (UUID字符串) |
| url | TEXT | NOT NULL | 下载URL地址 |
| target_path | TEXT | NOT NULL | 目标文件保存路径 |
| status | TEXT | NOT NULL | 任务状态 (JSON序列化) |
| created_at | INTEGER | NOT NULL | 创建时间 (Unix时间戳) |
| updated_at | INTEGER | NOT NULL | 最后更新时间 (Unix时间戳) |

**设计说明**:
- 使用TEXT类型存储UUID以提高可读性
- status字段存储JSON以支持复杂状态信息
- 时间字段使用INTEGER存储Unix时间戳

#### download_progress 表

##### `CREATE_DOWNLOAD_PROGRESS_TABLE`

**功能**: 创建下载进度表的SQL语句

**位置**: `src/schema.rs:16-26`

**表结构**:

| 字段名 | 类型 | 约束 | 描述 |
|--------|------|------|------|
| task_id | TEXT | PRIMARY KEY NOT NULL | 关联的任务ID |
| downloaded_bytes | INTEGER | NOT NULL DEFAULT 0 | 已下载字节数 |
| total_bytes | INTEGER | (可选) | 文件总字节数 |
| speed_bps | INTEGER | NOT NULL DEFAULT 0 | 下载速度 (字节/秒) |
| eta_seconds | INTEGER | (可选) | 预计剩余时间 (秒) |
| updated_at | INTEGER | NOT NULL | 最后更新时间 |

**外键约束**:
```sql
FOREIGN KEY (task_id) REFERENCES download_tasks(id) ON DELETE CASCADE
```

**设计说明**:
- task_id作为主键，每个任务只保存一条最新进度
- 使用外键约束确保数据一致性
- CASCADE删除确保任务删除时自动清理进度记录
- 可选字段允许NULL值以支持未知大小的下载

### 性能优化索引

##### `CREATE_INDEXES`

**功能**: 创建查询性能优化索引的SQL语句

**位置**: `src/schema.rs:29-33`

**索引列表**:

1. **`idx_download_tasks_status`**
   - **字段**: `status`
   - **用途**: 优化按状态筛选任务的查询
   - **对应方法**: `list_tasks_by_status()`, `count_tasks_by_status()`

2. **`idx_download_tasks_created_at`**
   - **字段**: `created_at`
   - **用途**: 优化按创建时间排序的查询
   - **对应方法**: `list_tasks()` (ORDER BY created_at DESC)

3. **`idx_download_tasks_updated_at`**
   - **字段**: `updated_at`
   - **用途**: 优化按更新时间查询和排序
   - **对应方法**: 未来可能的按更新时间筛选功能

**性能影响**:
- 显著提升状态筛选查询速度
- 加速时间排序操作
- 轻微增加写入开销

## 核心函数

### 初始化函数

##### `initialize_schema(db: &burncloud_database::Database) -> crate::Result<()>`

**功能**: 初始化数据库的完整模式

**位置**: `src/schema.rs:36-41`

**参数**:
- `db`: 数据库连接管理器引用

**返回**: 操作结果，可能包含数据库错误

**执行顺序**:
1. 创建 `download_tasks` 表
2. 创建 `download_progress` 表
3. 创建性能优化索引

**错误处理**:
- 任何步骤失败都会返回错误
- 使用 `?` 操作符进行错误传播

**幂等性**:
- 所有SQL语句都使用 `IF NOT EXISTS` 子句
- 可以安全地重复执行
- 不会破坏现有数据

**使用示例**:
```rust
let db = Database::new("download.db").await?;
initialize_schema(&db).await?;
```

## 数据类型映射

### Rust ↔ SQLite 类型映射

| Rust类型 | SQLite类型 | 用途 |
|----------|------------|------|
| String | TEXT | ID、URL、路径、状态 |
| i64 | INTEGER | 时间戳、字节数、速度 |
| Option\<i64\> | INTEGER (可选) | 可选的数值字段 |

### 特殊处理

1. **UUID → TEXT**
   - Rust: `TaskId` (基于UUID)
   - SQLite: TEXT字符串
   - 转换: `task_id.to_string()`

2. **路径 → TEXT**
   - Rust: `PathBuf`
   - SQLite: TEXT字符串
   - 转换: `path.to_string_lossy().to_string()`

3. **状态 → TEXT**
   - Rust: `DownloadStatus` 枚举
   - SQLite: JSON字符串
   - 转换: `serde_json::to_string()`

4. **时间 → INTEGER**
   - Rust: `SystemTime`
   - SQLite: Unix时间戳
   - 转换: `timestamp.timestamp()`

## 测试模块

### 测试用例

#### `test_sql_syntax()`

**功能**: 基本的SQL语法验证测试

**位置**: `src/schema.rs:48-53`

**验证内容**:
- 检查SQL语句包含预期的关键字
- 确保语句格式正确
- 语法静态验证

#### `test_schema_initialization()`

**功能**: 实际数据库初始化测试

**位置**: `src/schema.rs:55-62`

**测试流程**:
1. 创建内存数据库
2. 执行schema初始化
3. 验证操作成功

**测试意义**:
- 确保SQL语句在实际数据库中可执行
- 验证表创建和索引创建的正确性
- 检测潜在的SQL语法错误

## 数据库设计原则

### 规范化设计

1. **第一范式 (1NF)**: 所有字段都是原子值
2. **第二范式 (2NF)**: 非主键字段完全依赖于主键
3. **第三范式 (3NF)**: 非主键字段不传递依赖于主键

### 完整性约束

1. **实体完整性**: 主键约束确保记录唯一性
2. **参照完整性**: 外键约束确保关联数据一致性
3. **域完整性**: NOT NULL约束确保必要字段存在

### 性能考虑

1. **索引策略**: 为常用查询字段创建索引
2. **数据类型**: 选择合适的数据类型以节省空间
3. **约束优化**: 合理使用约束以提升查询性能

## 维护和扩展

### 添加新字段

如需添加新字段，建议的步骤：

1. **修改表定义**:
```sql
ALTER TABLE download_tasks ADD COLUMN new_field TEXT;
```

2. **创建迁移脚本**:
```rust
pub const ADD_NEW_FIELD: &str = "ALTER TABLE download_tasks ADD COLUMN new_field TEXT;";
```

3. **更新初始化函数**:
```rust
pub async fn migrate_to_v2(db: &Database) -> Result<()> {
    db.execute_query(ADD_NEW_FIELD).await?;
    Ok(())
}
```

### 添加新索引

```rust
pub const CREATE_NEW_INDEX: &str = "CREATE INDEX IF NOT EXISTS idx_new_field ON download_tasks(new_field);";
```

### 版本控制

建议实施数据库版本控制：

```rust
pub const SCHEMA_VERSION: i32 = 1;

pub const CREATE_VERSION_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER PRIMARY KEY
);
"#;
```

## 故障排除

### 常见问题

1. **外键约束错误**
   - 原因: 试图插入不存在的task_id
   - 解决: 确保先创建任务再创建进度

2. **索引创建失败**
   - 原因: 表不存在或权限不足
   - 解决: 检查表创建是否成功

3. **类型不匹配**
   - 原因: Rust类型与SQLite类型不兼容
   - 解决: 检查models.rs中的类型转换

## 文件位置

`src/schema.rs:1-63`

## 相关文件

- `src/repository.rs` - 使用这些表结构进行数据操作
- `src/models.rs` - 定义与表结构对应的Rust结构体
- `src/error.rs` - 处理schema初始化过程中的错误