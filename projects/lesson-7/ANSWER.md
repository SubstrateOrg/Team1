# 作业

## 题目要求

基础作业

- [x] 手动实现 Kitty 和 LinkedItem 的 Encode 和 Decode
  - [x] fn encode_to<T: Output>(&self, dest: &mut T)
  - [x] fn decode<I: Input>(input: &mut I) -> core::result::Result<Self, codec::Error>

额外作业

- [x] 讨论 SCALE 编码和其他编码的相⽐的优缺点
  - [x] 其他编码例子:protobuf, JSON, cbor
- [x] 如果由你来选择，你会选择什么编码，为什么?

---

## 作业答案

### 工程目录

> 建议使用外部目录, 但leason7/runtime也更新为该commit的代码

工程目录为: `projects/kitties`

本地作业最终结果commit为: `b005da18d6de741280260e066cc7847a48d0d7dd`

### 额外作业

> 讨论 SCALE 编码和其他编码的相⽐的优缺点

- SCALE: 可压缩，编码后体积非常小，非常适合空间容量有限的环境。
- protobuf: 由protobuf标准格式进行结构设计，可自动生成任意语言的结构体代码。
- JSON: 易用性高，明文有可读性高。

> 如果由你来选择，你会选择什么编码，为什么?

根据产品的设计目的进行选择，根据需求的侧重点。
