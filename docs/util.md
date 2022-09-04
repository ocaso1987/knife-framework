# knife-util工具包

## 包含以下内容:

### any工具
* **AnyValue:** 支持存入数据并以对象方式取出，也可以多次使用其指针，可以用于代替Box<dyn Any>类型，优点是可以取出原始类型并忽略生命周期限制。
* **AnyRef:** 支持绑定指针并使用其指针，可以用于代替&，优点是可以忽略生命周期限制。

### value工具
提供内容对象Value类型，并提供与serde_json::Value、serde_yaml::Value、toml::Value及Bson格式的相互转换
* **Value** 内置类型，可以与多种模式进行转换
* **ValuePointerExt:** 通过/a/b/c格式快速定位到子级内容
* **ValueMergeExt:** 实现数据对象的合并操作
* **ValueConvertExt:** 转换为Bson格式

### error工具
统一异常封装，以支持全局异常错误码处理。

### context工具
* **ContextExt:** 统一提供对上下文对象存入取出基本类型数据
* **AnyContextExt:** 统一提供对上下文对象存入任意类型数据

### number工具
* **IntegerCastTrait** 提供整型间数值的相互转换
* **DoubleCastTrait** 提供浮点型间数值的相互转换

### future工具
* **FutureHandler:** 用来代替dyn FnOnce(E) -> FutureObj<'static, R> + Send + 'static的工具
* **FutureObj:** 用来代替dyn Future<Output = T> + Send + 'static的工具

### string工具
* **StringExt:** 提供字段基本转换及正则匹配操作

### template工具
* **render_simple_template:** 根据字符内容渲染handlerbars模板
* **render_template:** 根据字符内容渲染handlerbars模板，支持采用占位符替换某参数，并返回占位符指定参数组成的集合
* **render_template_recursion:** 根据模板递归调用子模板、计算类型及上下文渲染模板

### vec工具
* **VecExt:** 对vec对象进行数据处理及转换操作
