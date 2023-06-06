# readme

## 核心功能是什么

核心功能是能将一个表当作一个graph-node来显示，主要显示表名

## todo

- [ ] 实时编辑太复杂了，可以先用更加简单的方式，就是在软件启动的时候将所有的schema信息都查询出来，然后再通过界面可视化的编辑sql，导出sql
- [ ] 确认主要创建table的graph node的方式，目前觉得最合理的是有两个途径：
    - 一个是左侧显示所有schema以及table的信息（树状结构显示，可展开折叠），然后可以拖动其中的table到画布上来来创建对应的graph
      node
    - 另一个是直接有添加不同类型graph node的按钮，选择添加table graph node时，默认在

## 最基础的功能

1. 输入url连接到指定数据库
2. 连接上数据库后在左侧以树状结构显示所有的schema,table,column等信息
3. 右键table可以创建table对应的graph node
4. graph node显示column,可以展开和折叠
    - 展开时在column的连接点处按下可以创建连接线,可以拖动并与其他graph node的连接点连接
    - 展开时显示所有column
    - 折叠时只显示有连接线的column