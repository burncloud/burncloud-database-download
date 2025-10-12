# burncloud-database-download 项目函数架构总览

## 项目架构概述

此项目是一个下载任务数据库持久化层，采用分层架构设计，提供类型安全的数据访问接口。

## 整体架构流程图

```mermaid
flowchart TB
    subgraph "业务层"
        BL[DownloadTask / DownloadProgress]
    end

    subgraph "数据访问层"
        RA[DownloadRepository]
    end

    subgraph "数据模型层"
        DM[DownloadTaskRecord / DownloadProgressRecord]
    end

    subgraph "数据库层"
        DB[(SQLite Database)]
        DT[download_tasks]
        DP[download_progress]
    end

    BL <--> RA
    RA <--> DM
    DM <--> DB
    DB --> DT
    DB --> DP
    DT -.->|外键关系| DP
```

## 模块依赖关系图

```mermaid
flowchart LR
    subgraph "lib.rs"
        L[模块导出和重新声明]
    end

    subgraph "error.rs"
        E[错误类型定义]
    end

    subgraph "models.rs"
        M[数据模型转换]
    end

    subgraph "schema.rs"
        S[数据库架构初始化]
    end

    subgraph "repository.rs"
        R[数据仓库实现]
    end

    L --> E
    L --> M
    L --> S
    L --> R
    R --> E
    R --> M
    R --> S
    M --> E
    S --> E
```

## 函数调用关系图

```mermaid
flowchart TD
    subgraph "Repository 公共接口"
        NEW[new]
        INIT[initialize]
        SAVE[save_task]
        GET[get_task]
        GETURL[get_task_by_url]
        LIST[list_tasks]
        LISTST[list_tasks_by_status]
        DEL[delete_task]
        SAVEP[save_progress]
        GETP[get_progress]
        DELP[delete_progress]
        COUNT[count_tasks]
        COUNTST[count_tasks_by_status]
        CLEAR[clear_all]
    end

    subgraph "内部依赖函数"
        SCHEMA[initialize_schema]
        FROMTASK[DownloadTaskRecord::from_task]
        TOTASK[DownloadTaskRecord::to_task]
        FROMPROG[DownloadProgressRecord::from_progress]
        TOPROG[DownloadProgressRecord::to_progress]
    end

    INIT --> SCHEMA
    SAVE --> GETURL
    SAVE --> FROMTASK
    GET --> TOTASK
    GETURL --> TOTASK
    LIST --> TOTASK
    LISTST --> TOTASK
    SAVEP --> FROMPROG
    GETP --> TOPROG
```

## 数据流转图

```mermaid
flowchart LR
    subgraph "输入数据"
        IT[DownloadTask]
        IP[DownloadProgress]
        IID[TaskId]
        IURL[URL String]
        IST[DownloadStatus]
    end

    subgraph "数据转换"
        TR[Type Conversion]
        SER[Serialization]
        DESER[Deserialization]
    end

    subgraph "数据库操作"
        INS[INSERT]
        UPD[UPDATE]
        SEL[SELECT]
        DEL[DELETE]
    end

    subgraph "输出数据"
        OT[DownloadTask]
        OP[DownloadProgress]
        OL[Task List]
        OC[Count Results]
    end

    IT --> TR
    IP --> TR
    IID --> TR
    IURL --> TR
    IST --> SER

    TR --> INS
    TR --> UPD
    TR --> SEL
    TR --> DEL

    SER --> INS
    SER --> UPD

    SEL --> DESER
    DESER --> OT
    DESER --> OP
    DESER --> OL
    SEL --> OC
```

## 错误处理流程图

```mermaid
flowchart TD
    subgraph "错误源"
        SE[Serialization Error]
        DE[Database Error]
        CE[Conversion Error]
        NF[Not Found Error]
    end

    subgraph "错误转换"
        MAP[Error Mapping]
    end

    subgraph "统一错误类型"
        DBE[DownloadDbError]
    end

    subgraph "错误处理策略"
        PROP[Error Propagation]
        LOG[Error Logging]
        RET[Error Return]
    end

    SE --> MAP
    DE --> MAP
    CE --> MAP
    NF --> MAP

    MAP --> DBE

    DBE --> PROP
    DBE --> LOG
    DBE --> RET
```

## 性能优化策略

### 数据库优化

```mermaid
flowchart LR
    subgraph "查询优化"
        IDX[索引策略]
        BATCH[批量操作]
        CACHE[连接池]
    end

    subgraph "数据优化"
        NORM[数据标准化]
        COMP[数据压缩]
        PART[数据分区]
    end

    IDX --> FAST[快速查询]
    BATCH --> THROUGHPUT[高吞吐量]
    CACHE --> CONN[连接复用]

    NORM --> CONSIST[数据一致性]
    COMP --> SPACE[存储优化]
    PART --> SCALE[可扩展性]
```

### 内存优化

```mermaid
flowchart LR
    subgraph "类型优化"
        ZERO[Zero-Copy]
        STREAM[流式处理]
        LAZY[延迟加载]
    end

    subgraph "生命周期管理"
        BORROW[借用检查]
        MOVE[移动语义]
        CLONE[智能克隆]
    end

    ZERO --> PERF[性能提升]
    STREAM --> MEM[内存效率]
    LAZY --> LOAD[按需加载]

    BORROW --> SAFE[内存安全]
    MOVE --> EFFICIENT[高效传输]
    CLONE --> BALANCE[平衡性能]
```

## 测试策略

### 测试层次结构

```mermaid
flowchart TD
    subgraph "单元测试"
        UT1[模型转换测试]
        UT2[函数逻辑测试]
        UT3[错误处理测试]
    end

    subgraph "集成测试"
        IT1[数据库操作测试]
        IT2[端到端流程测试]
        IT3[并发安全测试]
    end

    subgraph "测试工具"
        MOCK[Mock Database]
        FIXTURE[测试数据装置]
        HELPER[测试辅助函数]
    end

    UT1 --> IT1
    UT2 --> IT2
    UT3 --> IT3

    MOCK --> IT1
    FIXTURE --> IT2
    HELPER --> IT3
```

## 部署和监控

### 部署架构

```mermaid
flowchart LR
    subgraph "开发环境"
        DEV[Development DB]
    end

    subgraph "测试环境"
        TEST[Test DB]
    end

    subgraph "生产环境"
        PROD[Production DB]
        BACKUP[Backup DB]
    end

    DEV --> TEST
    TEST --> PROD
    PROD --> BACKUP
```

### 监控指标

```mermaid
flowchart TD
    subgraph "性能指标"
        QPS[查询每秒]
        LAT[查询延迟]
        CPU[CPU 使用率]
        MEM[内存使用率]
    end

    subgraph "业务指标"
        TASK[任务数量]
        PROG[进度更新]
        ERR[错误率]
        SUCC[成功率]
    end

    subgraph "系统指标"
        DISK[磁盘使用]
        CONN[连接数]
        LOCK[锁等待]
        WAL[WAL 大小]
    end

    QPS --> ALERT[性能告警]
    LAT --> ALERT
    ERR --> ALERT
    DISK --> ALERT
```

## 项目扩展规划

### 功能扩展

```mermaid
flowchart LR
    subgraph "当前功能"
        BASIC[基础 CRUD]
        PROG[进度跟踪]
        STAT[统计功能]
    end

    subgraph "扩展功能"
        SEARCH[高级搜索]
        FILTER[复杂过滤]
        EXPORT[数据导出]
        IMPORT[数据导入]
    end

    subgraph "高级功能"
        REPLICA[数据复制]
        SHARDING[数据分片]
        CACHE[缓存层]
        QUEUE[任务队列]
    end

    BASIC --> SEARCH
    PROG --> FILTER
    STAT --> EXPORT
    STAT --> IMPORT

    SEARCH --> REPLICA
    FILTER --> SHARDING
    EXPORT --> CACHE
    IMPORT --> QUEUE
```

这个总览文档提供了项目的完整技术架构视图，包括了所有函数的关系、数据流转、错误处理、性能优化、测试策略和未来扩展规划。