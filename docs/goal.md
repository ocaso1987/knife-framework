# 开发目标

本框架旨在构建一个类似于SpringBoot的微服务框架，并支持对企业级配置注册等资源进行统一访问与管理，主要包括：

* 提供一个可用的Web集成框架，作为所有项目开发的基石
* 无缝接入技术平台
* 服务管控与治理
* 日志的全链路追踪

## 基本原则

为实现目标，本框架遵循配置约定及最小实现的开发原则，不追求实现各种不同的开发场景，而是在完成最小颗粒的基本功能前提下，逐步构建（技术平台）产品线，因此在技术选型上相同的功能模块可能只提供较少的开发模式，譬如数据库选型，暂时只考虑使用PostgresSQL等。

