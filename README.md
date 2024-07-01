# Drone Delivery Problem
高级算法大作业：无人机配送问题

## TL;DR
运行：
```shell
git clone https://github.com/CircuitMurderer/drone_delivery_solve.git
cd drone_delivery_solve
cargo run
```

## Problem Intro
无人机可以快速解决最后10公里的配送，本作业要求设计一个算法，实现一个特定区域内的无人机配送问题的路径规划。

假设在一区域中，共有j个配送中心，任意一个配送中心有用户所需要的商品，其数量无限，同时任一配送中心的无人机数量无限。该区域同时有k个卸货点（无人机只需要将货物放到相应的卸货点即可），假设每个卸货点会随机生成订单，一个订单只有一个商品，但这些订单有优先级别，分为三个优先级别（用户下订单时，会选择优先级别，优先级别高的付费高）：

- 一般：3小时内配送到即可；
- 较紧急：1.5小时内配送到；
- 紧急：0.5小时内配送到。

我们将时间离散化，也就是每隔t分钟，所有的卸货点会生成订单（0-m个订单）；同时每隔t分钟，系统要做出决策，包括：
- 哪些配送中心出动多少无人机完成哪些订单；
- 每个无人机的路径规划，即先完成哪个订单，再完成哪个订单，等等。最后需要返回原来的配送中心；

注意：系统做决策时，可以不对当前的某些订单进行配送，因为当前某些订单可能紧急程度不高，可以累积后和后面的订单一起配送。

目标：一段时间内（如一天），所有无人机的总配送路径最短。

约束条件：满足订单的优先级别要求。

假设条件：
- 无人机一次最多只能携带n个物品；
- 无人机一次飞行最远路程为20公里（无人机送完货后需要返回配送点）；
- 无人机的速度为60公里/小时；
- 配送中心的无人机数量无限；
- 任意一个配送中心都能满足用户的订货需求。

## Algorithm Design

### Assumption
为了简化问题，这里假设所有的配送中心和配货点都位于二维平面，且每两点之间都可达。每个配送中心和配货点都有一个坐标 $(x, y)$ ，两个点之间的距离就是欧氏距离：

$$dist = \sqrt{(x_1 - x_2)^2 + (y_1 - y_2)^2}$$

所以可以很轻松地构建出一个邻接矩阵 $A$ ，其中 $A_{ij}$ 表示表示点 $i$ 到点 $j$ 的距离。接着，我们来假设其他未给出的变量。设它们为以下值：
- 配送中心数量 $j = 3$ ；
- 卸货点数量 $k = 6$ ；
- 生成订单时间间隔 $t = 30(min)$ ；
- 每次生成的订单数量 $m = 3$ ；
- 无人机一次最多携带数量 $n = 3$ 。

这样就构成了一个完整的问题。我们将单位统一，则已知的其他变量为：
- 三个优先级 $Pri_1, Pri_2, Pri_3 = 30, 90, 180(min)$ ；
- 无人机一次飞行最远路程 $d = 20(km)$ ；
- 无人机一次最大飞行距离 $s = 1(km/min)$ ；
- 总计时间长度 $T = 180(min)$ 。

### Think
每一个无人机在配送完后都需要回到原来的配送中心，且优化目标为路径最短，这让我们想到了TSP（旅行商）问题，即求经过所有需要经过节点的最短回路。假如题目只有一个配送中心、且同时只能配出一架无人机，那其就是一个旅行商问题，我们可以将产出订单的配货点作为需要经过的节点，配送中心作为起点，用解TSP的思路去求解。

但很明显此题目不能直接化为旅行商问题，因为：
- 题目不仅有一个配送中心；
- 每个配送中心不止可以配出一架无人机；
- 每架无人机有最远距离和携带物品限制；
- 每个订单有优先级和时间限制。

但我们可以这样考虑：假如每架无人机按照此问题的最优解进行配送，那么其走过的轨迹上所有的点（配货点）和起点（配送中心）构成的图的TSP回路就是其最优路径。所以理论上来说，我们可以通过穷举所有组合，并一一检查合理的节点组合，单独地对每个组合求其TSP回路，并进行比较，获得精确的最优解。但这样做的复杂度非常高，光是穷举就需要 $O(n!)$ 的时间复杂度，更别提计算了。

我们来换个思路：采用近似算法和缩减法。假设系统在某一个时间点要处理一些订单，则我们可以分别以每个配送中心为出发点、所有的持有订单的配货点作为其他节点作为一个图，来解求一条近似最优的TSP回路。这里考虑：
- 对于配送中心来说，这几条回路就是一次性配送所有订单的较优解；
- 但对于持有订单的配货点而言，它的最优解是直接从最近的配送中心出发，到达配货点再返回。

所以：
- 如果贸然取配送中心的几个TSP回路，则必然有更好的解，因为不是所有的配货点都必须等待一次性配送；
- 如果直接取配货点的最短回路，则不能保证其全局最优性，因为没有考虑到配送点之间的距离。

那么我们的算法怎么做？很简单：
1. 首先求得每个配送中心到所有有订单的配货点的TSP回路；
2. 选择一个未配送订单，然后选择此订单持有者（配货点）的最近配送中心对应的回路；
3. 遍历此回路的每个节点，如果节点有更优解（直接到配送中心）或不满足要求，删除此节点；
4. 将上述路径中的节点所持有的订单标记为已配送；
5. 回到2，重复，直至所有的订单都已得到配送。

具体算法设计请见下一小节。因为我们选的是距离配货点最近的配送中心，即在其回路中可以较快遍历到此配货点，可以保证解较优；同时，在遍历回路时，若某节点有更优解，则舍弃路径中的此节点，使其在别的更优回路或是自己对应的最优回路被选择，确保得到更好的解。结束后，算法会输出一系列的路径，其数量小于等于待处理的订单数量。

关于调度，因为即使是最优的订单都有30分钟的等待时间，而无人机受限于速度和最大距离，最多只能飞20分钟。为了确保每个订单都能送达，我们设每个配货点到配送中心的距离都不大于10km以保证无人机可送达。我们的假定是每30分钟生成一次订单，所以一定可以在下一次生成订单前配送完毕所有优先级最高的订单。那么对于调度的设计就简单了，只需在每个时间片仅处理优先级最高的订单，然后在下一个时间片将上一个时间片的其他优先级的订单优先级集体提高一级即可。最后一个时间片会将

### Design
首先是通过最小生成树来近似求解最优回路，使用Prim算法：
```
Algo 1: get loops use MST
-------------------------
$senders <- distribution centres 
$receivers <- distribution points with orders
$loops = empty
for $sender in $senders do
    $graph <- build graph by $sender and $receivers
    $mst <- Prim($graph) to generate MST, with root $sender
    $loop <- prior-order traversal $mst
    $loop.add($sender)
    $loops.add($loop)
endfor
return $loops
```
以上就通过近似算法求得了每个配送中心对应的最小回路。设每个图的总节点数为n、配送中心数量为m，因为使用Prim算法获得最小生成树的时间复杂度是 $O(n^2)$ ，所以此算法的整体时间复杂度为 $O(mn^2)$ 。

接下来是根据限制条件缩减回路的算法：
```
Algo 2: reduce and limit loops
------------------------------
$drone <- drone configs
$loops <- loops solved by Algo 1
$orders <- orders to handle
$paths <- empty
for $order in $orders do
    continue if $order is handled
    $sender <- nearest sender of $order.owner
    $path <- $loops[$sender]
    for $x in $path do
        continue if $x is $sender
        if $x.order.priority is not satisfied then
            delete $x
        endif
        if distance($x.previous to $x to $sender) > 2 * (distance($x to $x's nearest sender)) then
            delete $x
        endif 
        if distance($x.previous to $x to $sender) > $drone's max fly distance then
            delete $x
        endif
        if $drone.carry == $drone's max carry then
            delete $x
        endif
        set $x.order to handled
    endfor
    $paths.add($path)
endfor
return $paths
```
以上即可通过约束条件来缩减回路，最后得到一系列较优的解。其中，设系统每批次生成k个订单，每个图总节点数为n，则总的时间复杂度为 $O(kn)$ 。

最后是调度算法：
```
Algo 3: schedule
----------------
$list <- list including high, mid and low priority orders
for $time in time slices do
    $list <- generate some random orders inner
    if $time is last time slice do
        set all orders' priority in $list to high-pri
    endif
    handle orders in $list's high-pri list
    $list's mid-pri and low-pri priority increase one level
endfor
```

