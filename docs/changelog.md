# 变更记录

## v1.0.6
* 按照clippy建议对代码进行更正
* 本着最小可用原则，临时去掉对bson、toml格式的支持
* 对Web层提供Value参数支持
* 增加AsyncInto、AsyncFrom特征，以支持在转换逻辑中采用异步实现
* AnyValue、AnyRef、AppError工具类改为支持Copy,在复制后指向的对象不变
* 将AnyValue及AnyRef中的as_ref及as_mut方法改为to_ref及to_mut，以避开AsRef、AsMut等常用名称
* 增加MergeValue派生宏，用于POJO对象合并Value(JSON)对象
* 增加对Date、Time、DateTime处理工具类的支持
* 增加field_spec属性宏，功能待扩展，现阶段可生成注释文档
* AppError增加backtrace功能

## v1.0.5
* 对util包进行了结构调整，影响颇大
* 去除anyhow，anyhow很好用也推荐用，但本框架致力于采用统一简约的完成开发目标，本次重构error模块，可以对大多数异常进行统一处理，anyhow的封装在功能上有重叠之处，在排版上还有待优化；
* handlebars_sql在实际接入应用服务中可以提供完善的增删改查功能；
* 增加EnumName宏使得枚举支持显示或遍历其名称

## v1.0.4

* 因为sqlx的泛型机制过于复杂，重新采用rbatis替代原有sqlx；
* 支持采用handlebars来实现SQL查询；