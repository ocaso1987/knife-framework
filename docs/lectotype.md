# 技术选型

框架技术选型多数情况下在knife-util工具包中就已包含进去，以实现对原有工具的扩展需要，技术选型包含以下内容及其选择原因。

## 通用工具包

* **async-trait**
    
    * 支持在trait中定义异步方法

* **lay_static**

    * 定义全局变量

* **ctor**
    * 在文件加载到mod后自动执行一个方法，用于自动注册全局Componet对象

* **anyhow**
    * 统一封装异常信息

* **chrono**
    * 时间处理

## 字符串、数值与集合

* **regex**
    * 正则处理

* **globset**
    * 提供*及?的通配符处理

* **handlebars**
    * 提供模板生成字符串
    * 提供模板生成SQL，以进行SQL处理

## 配置文件、对象序列化

* **serde**
    * 默认序列化框架

* **serde_json**
    * JSON处理

* **serde_yaml**
    * Yaml处理

* **toml**
    * Toml配置支持

## 数据库操作

* **rbatis**
    * 数据库操作

## Server服务

* **tokio**
    * 异步框架集成

* **hyper**
    * Web框架集成

* **tonic**
    * Grpc集成