# jlib-rs Workloads

<table>
<tr>
    <td rowspan="8"> 文件状态：<br/>
        [√] 草稿<br/>
        [√] 正在修改<br/>
        [ ] 正式发布 </td>
</tr>
<tr>
    <td>当前版本：</td>
    <td>0.0.1</td>
</tr>
<tr>
    <td>项目名称：</td>
    <td>jlib-rs</td>
</tr>
<tr>
    <td>作    者：</td>
    <td>zTgx</td>
</tr>
<tr>
    <td>Email：</td>
    <td>beautifularea@163.com</td>
</tr>
<tr>
    <td>创建日期：</td>
    <td>2019年8月7日</td>
</tr>
<tr>
    <td>最后更新：</td>
    <td></td>
</tr>
<tr>
    <td>版权说明：</td>
    <td>MIT</td>
</tr>
</table>

<table>
<tr>
    <td rowspan="7" style="width: 250px;">模块概述：<br/>
        [√] 调研js版本<br/>
</tr>
<tr>
    <td style="width: 100px;">工作量</td>
    <td>1周</td>
</tr>
<tr>
    <td>内容</td>
    <td>
    <li> 搭建jintum-lib开发环境，熟悉js版本接口调用 </li>
    <li> 学习js语言，熟悉jingtum-lib/jingtum-base-lib代码 </li>
    <li> 根据官方文档，测试js接口 </li>
    <li> 调研js版本代码组成结构 </li>
    <li> 分析主要模块功能 </li>
    <li> 了解请求类接口数据结构及实现原理 </li>
    <li> 了解交易类接口数据结构及实现原理 </li>
    <li> 了解以太坊solidity合约 </li>
    </td>
</tr>
</table>

<table>
<tr style="width: 250px;">
    <td rowspan="7" style="width: 250px;">模块概述：<br/>
        [√] 初始化工程<br/>
</tr>
<tr>
    <td style="width: 100px;">工作量</td>
    <td>3人天</td>
</tr>
<tr>
    <td>内容</td>
    <td>
    <li> 创建工程jlib-rs(jingtum-lib Rust版本) </li>
    <li> 调研确定第三方协议库如ws,secp256k1,serde等 </li>
    <li> 实验server_info接口实现 </li>
    </td>
</tr>
</table>

<table>
<tr style="width: 250px;">
    <td rowspan="7" style="width: 250px;">模块概述：<br/>
        [√] 确定请求command数据结构<br/>
</tr>
<tr>
    <td style="width: 100px;">工作量</td>
    <td>2人天</td>
</tr>
<tr>
    <td>内容</td>
    <td>
    <li> 添加config配置类，包含地址，类型，签名等配置信息 </li>
    <li> 抽象command类，根据不同接口的参数来实现 </li>
    <li> 声明CommandConversion trait,  来序列化api请求的命令参数， 由各个命令类来实现</li>
    </td>
</tr>
</table>

<table>
<tr>
    <td rowspan="7" style="width: 250px;">模块概述：<br/>
        [√] 实现接口（无本地签名）<br/>
</tr>
<tr>
    <td style="width: 100px;">工作量</td>
    <td>4周</td>
</tr>
<tr>
    <td>内容</td>
    <td>
    <li> 分析调试js版本request_server_info接口参数及Rust实现 </li>
    <li> 分析调试js版本ledger_closed接口参数及Rust实现 </li>
    <li> 分析调试js版本spec_ledger接口参数及Rust实现 </li>
    <li> 分析调试js版本account_info接口参数及Rust实现</li>
    <li> 分析调试js版本spec_tx接口参数及Rust实现 </li>
    <li> 分析调试js版本account_tums接口参数及Rust实现</li>
    <li> 分析调试js版本relations接口参数及Rust实现</li>
    <li> 分析调试js版本offer接口参数及Rust实现</li>
    <li> 分析调试js版本account_tx接口参数及Rust实现</li>
    <li> 分析调试js版本order_book接口参数及Rust实现</li>
    <li> 分析调试js版本broker.rs接口参数及Rust实现</li>
    <li> 分析调试js版本subscribe接口参数及Rust实现</li>
    <li> 分析调试js版本pay接口参数及Rust实现</li>
    <li> 分析调试js版本relation接口参数及Rust实现</li>
    <li> 分析调试js版本create_offer接口参数及Rust实现 </li>
    <li> 分析调试js版本cancel_offer接口参数及Rust实现 </li>
    <li> 完善command相关的请求/序列化/工具类 </li>
    <li> 重构Get/Post接口请求代码 </li>
    <li> 调试接口，完善API中数据结构，对于特定条件下服务下发的字段，使用Option<T>类型接收，比如：支付接口中的Memos字段 </li>
    <li> 对Request和Response数据结构添加serde，default修饰供序列化 </li>
    <li> 针对必要的数据类型进行手动序列化，如：TxJson
    <li> 更新README,gitignore等</li>
    <li> 导出API到lib.rs中，简化调用 </li>
    </td>
</tr>
</table>

<table>
<tr>
    <td rowspan="7" style="width: 250px;">模块概述：<br/>
        [√] examples<br/>
</tr>
<tr>
    <td style="width: 100px;">工作量</td>
    <td>1周</td>
</tr>
<tr>
    <td>内容</td>
    <td>
    <li> 针对所有接口，添加examples </li>
    <li> 持续更新维护examples,包括接口数据结构调整，优化 </li>
    </td>
</tr>
</table>

<table>
<tr>
    <td rowspan="7" style="width: 250px;">模块概述：<br/>
        [√] 钱包模块<br/>
</tr>
<tr>
    <td style="width: 100px;">工作量</td>
    <td>2周</td>
</tr>
<tr>
    <td>内容</td>
    <td>
    <li> 调研secp256k1及rust-crypto算法库 </li>
    <li> 完成钱包类的创建 </li>
    <li> 通过WalletType来定制钱包的类型 </li>
    <li> seed 的生成 </li>
    <li> address的生成 </li>
    <li> 基于Secp256k1库，keypair的生成 </li>
    <li> curve模块中对Ripemd160，secp256k1，sha256等相关加密算法的封装 </li>
    <li> 导出generate_wallet方法到lib.rs，最大化的方便钱包的创建
    </td>
</tr>
</table>

<table>
<tr>
    <td rowspan="7" style="width: 250px;">模块概述：<br/>
        [√] serialize模块<br/>
</tr>
<tr>
    <td style="width: 100px;">工作量</td>
    <td>4周</td>
</tr>
<tr>
    <td>内容</td>
    <td>
    <li> 研究Js版本中相关序列化库的实现原理 </li>
    <li> 在Rust的实现版本中优化了Js版本中的重复序列化问题。在生成txn_signature，及生成tx_blob的两个过程中，对基础fields如"Flags", "Fee", "TransactionType", "Account", "SigningPubKey", "Sequence"的重复计算问题 </li>
    <li> 实现所有字段的序列化，针对不同字段，设置不同的序列化trait 及lifetime </li>
    <li> 解耦header和body的序列化 </li>
    <li> 调整数据序列化方式，实现序列的模块化 </li>
    <li> 代码调整优化，解决warnings，删除debug信息 </li>
    <li> 调试js版本中的序列化，对比测试Rust实现 </li>
    <li> 实现string_or_struct方法，重点处理Amount在针对native/non-native的字符串或者对象的序列化问题，对Amount相关联的数据结构单独的实现序列化操作。 </li>
    <li> Amount其他操作如multi1000000, Amount对象和RAmount对象的映射 </li>
    <li> Amount 类引入BigInt大数处理 </li>
    </td>
</tr>
</table>

<table>
<tr>
    <td rowspan="7" style="width: 250px;">模块概述：<br/>
        [√] 本地签名模块<br/>
</tr>
<tr>
    <td style="width: 100px;">工作量</td>
    <td>4周</td>
</tr>
<tr>
    <td>内容</td>
    <td>
    <li> 实现payment支付功能api的本地签名 </li>
    <li> 实现relation设置关系api的本地签名 </li>
    <li> 实现create_offer挂单api的本地签名 </li>
    <li> 实现cancel_offer取消挂单api的本地签名 </li>
    <li> 实现set_brokerage手续费api的本地签名 </li>
    <li> 重构代码结构，由sign_tx提供统一的接口及数据结构 </li>
    <li> 区别于js版本，创建单独的交易签名模块，更大限度的降低代码的耦合度，易于使用 </li>
    <li> 参考js版本，调试交易的本地签名blob功能 </li>
    <li> 配合serialize模块，集成测试 </li>
    </td>
</tr>
</table>

<table>
<tr>
    <td rowspan="7" style="width: 250px;">模块概述：<br/>
        [√] API容错处理<br/>
</tr>
<tr>
    <td style="width: 100px;">工作量</td>
    <td>1周</td>
</tr>
<tr>
    <td>内容</td>
    <td>
    <li> 添加API模块中所有接口的错误处理, 更新所有接口返回类型为Result类型, 调用方通过is_ok, is_err方法来判断接口返回的数据结构 </li>
    <li> 接口参数的输入安全容错等，主要是address/secret的有效性判定，重构build_keypair_str方法 </li>
    <li> 搭建测试环境，模拟测试接口
    </td>
</tr>
</table>

<table>
<tr>
    <td rowspan="7" style="width: 250px;">模块概述：<br/>
        [√] 创建工具库<br/>
</tr>
<tr>
    <td style="width: 100px;">工作量</td>
    <td>1周</td>
</tr>
<tr>
    <td>内容</td>
    <td>
    <li> 创建独立的base58库basex-rs </li>
    <li> 创建数据类型转换库cast-rs </li>
    <li> 其他公用方法的util库实现 </li>
    <li> 更新jlib-rs工程配置，替换代码 </li>
    <li> 针对新库的更新进行针对性测试 </li>
    </td>
</tr>
</table>

<table>
<tr>
    <td rowspan="7" style="width: 250px;">模块概述：<br/>
        [√] solidity合约<br/>
</tr>
<tr>
    <td style="width: 100px;">工作量</td>
    <td>1周</td>
</tr>
<tr>
    <td>内容</td>
    <td>
    <li> 实现合约（无参数）的部署接口deploy，调用接口call </li>
    <li> 调试实现带（有参数）的部署和调用接口 </li>
    <li> 调试合约实现的数据结构 </li>
    <li> 优化合约参数的输入，直接传递原始值，无需调用方先转换为Hex </li>
    <li> 测试无参数和有参数的合约代码 </li>
    </td>
</tr>
</table>

<table>
<tr>
    <td rowspan="7" style="width: 250px;">模块概述：<br/>
        [√] 文档整理<br/>
</tr>
<tr>
    <td style="width: 100px;">工作量</td>
    <td>1周</td>
</tr>
<tr>
    <td>内容</td>
    <td>
    <li> github上创建wiki </li>
    <li> 文档化API接口的参数说明，数据格式，调用方式 </li>
    <li> 文档化jlib-rs库的环境搭建方法，使用jlib-rs实现一个最简单的接口 </li>
    <li> 更新example,README等 </li>
    <li> 添加LICENSE, .travis.yml等 </li>
    </td>
</tr>
</table>

<table>
<tr>
    <td rowspan="7" style="width: 250px;">模块概述：<br/>
        [√] 工程结构重构<br/>
</tr>
<tr>
    <td style="width: 100px;">工作量</td>
    <td>1周</td>
</tr>
<tr>
    <td>内容</td>
    <td>
    <li> 模块化代码结构,解耦序列化及交易签名实现 </li>
    <li> 抽象独立的库 </li>
    <li> 格式化代码，消除warnigns, 删除unused代码和文件等 </li>
    <li> 优化实现，使得工程更Rust化 </li>
    </td>
</tr>
</table>

<table>
<tr>
    <td rowspan="7" style="width: 250px;">模块概述：<br/>
        [√] 持续更新<br/>
</tr>
<tr>
    <td style="width: 100px;">工作量</td>
    <td>2周</td>
</tr>
<tr>
    <td>内容</td>
    <td>
    <li> 5.22设置挂单佣金 buildBrokerageTx 接口的实现 </li>
    <li> 保持对官方js版的持续跟踪更新... </li>
    <li> 持续优化代码... </li>
    </td>
</tr>
</table>

<table>
<tr>
    <td rowspan="7" style="width: 250px;">模块概述：<br/>
        [√] 待完善<br/>
</tr>
<tr>
    <td style="width: 100px;">工作量</td>
    <td>2周</td>
</tr>
<tr>
    <td>内容</td>
    <td>
    <li> process_tx方法实现
    </td>
</tr>
</table>
