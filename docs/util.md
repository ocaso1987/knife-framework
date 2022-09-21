# knife-util工具包

## 包含以下内容:

### any工具
用于处理指针等特殊操作，内部实现大量使用unsafe操作，在使用时需注意
* **AnyValue:** 支持存入数据并以对象方式取出，也可以多次使用其指针，可以用于代替Box<dyn Any>类型，优点是可以取出原始类型并忽略生命周期限制
* **AnyRef:** 支持绑定指针并使用其指针，可以用于代替&，优点是可以忽略生命周期限制
* **AnyFuture:** 用来代替dyn Future<Output = T> + Send + 'static的工具
* **AnyHandler:** 用来代替dyn FnOnce(E) -> AnyFuture<'static, R> + Send + 'static的工具

### context工具
为对象提供上下文存取值的快捷操作
* **ContextExt:** 统一提供对上下文对象存入取出基本类型数据
* **AnyContextExt:** 统一提供对上下文对象存入任意类型数据

### error工具
统一封装异常
* **AppError：** 统一封装异常，以支持全局异常错误码处理。

### page工具
分页功能
* **get_offset:** 将页面分页参数转换为数据库偏移参数
* **PageRequest:** 统一分页请求
* **PageResult:** 统一分页响应

### template工具
模板工具
* **render_simple_template:** 根据字符内容渲染handlerbars模板
* **render_sql_template:** 根据字符内容渲染SQL格式的handlerbars模板
* **render_template:** 根据字符内容渲染handlerbars模板，支持采用占位符替换某参数，并返回占位符指定参数组成的集合
* **render_template_recursion:** 根据模板递归调用子模板、计算类型及上下文渲染模板

## type工具
为常见数据类型提供增强功能
* **DoubleCastExt:** 为浮点型数据提供数值转换功能
* **IntegerCastExt:** 为整型数据提供数值转换功能
* **StringExt:** 提供字段基本转换及正则匹配操作
* **VecExt:** 对vec对象进行数据处理及转换操作

### value工具
提供内容对象Value类型，并提供与serde_json::Value、serde_yaml::Value、toml::Value及Bson格式的相互转换
* **Value** 内置类型，可以与多种模式进行转换
* **ConvertExt:** 转换为Bson格式
* **MergeExt:** 实现数据对象的合并操作
* **PointerExt:** 通过/a/b/c格式快速定位到子级内容