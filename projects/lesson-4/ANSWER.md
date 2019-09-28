# 作业

## 题目要求

- [x] 重构 create，使⽤用新的帮助函数
- [x] 完成 combine_dna
- [x] 设计加密猫模块 V3
  - [x] transfer kitty 转移猫
  - [x] 要求复杂度必须优于 O(n)
- [x] 设计如何在substrate中实现树形结构

---

## 作业答案

### 工程目录

> 建议使用外部目录, 但leason3/runtime也更新为该commit的代码

工程目录为: `projects/kitties`

本地作业最终结果commit为: `366b871d3b50ebd53f5ffde96062d5710ff6e47b`

### 题目答案

> 设计如何在substrate中实现树形结构

- 用一个map记录`node_id: NodeId -> NodeData`
- NodeData的struct中，记录`parent: Option<NodeId>`
- 用一个double map记录`(node_id: NodeId, child_idx: u32) -> NodeId
- 用一个map记录`node_id: NodeId -> u32`子节点总数量
