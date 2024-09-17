# BiliCleaner 🎯
- `BiliCleaner` 是一个基于 [qaqFei/BiliCleaner](https://github.com/qaqFei/BiliCleaner) 的优秀 Fork（自认为）。
- 由于原仓库已不再积极维护，本仓库将在同步原仓库功能更新的同时会增添许多原仓库所不计划/不具有的功能。
- 此仓库未来可能会完全脱离原仓库。

---

## 处理异常
- **与 `config.json` 相关的异常**：
  - 如果问题无法解决, 可以删除 `config.json` 文件

## `config.json` 配置文件 📝
- `headers`: 📨 B站api的请求头
    - `User-Agent`: 🔍 浏览器标识
    - `Cookie`: 🍪 B站api的请求头中的 `Cookie`
- `bili_report_api`: 📡 是否调用B站api的举报接口
- `reply_limit`: 🔒 单条视频获取评论的最大数量 尽量不要大于100 可能会被风控
- `enable_gpt`: 🤖 是否启用GPT进行评论过滤
- `gpt_apibase`: 🔗 GPT的API地址
- `gpt_proxy`: 🔗 GPT的代理地址
- `gpt_apikey`: 🔑 GPT的API密钥
- `gpt_model`: 🧠 GPT的模型名称
- `enable_check_lv2avatarat`: 📷 启用检查评论是否包含头像 (前置: lv.2, 包含@)
- `enable_check_replyimage`: 📷 启用识别评论图像 

## 开发贡献 🤝
- **过滤规则**：
  - 过滤规则存储在 `./res/rules.yaml` 文件中（即将弃用）
  - 结构
    - `rules_exact` 为一个列表 type: `list[list[str] | str]`
       - `list[str]` 为一个字符串列表, 每个字符串代表一个关键词, 在前面添加`$-not `即可对结果取反, 如: `["http", "$-not https", "$-not bilibili", "$-not 163cn.tv"]`
       - `str` 正则表达式
    - `rules_elastic` 模糊匹配的规则 type: `list[str]`
  

---

## 声明 ⚠️
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

---

## License 📄
BiliCleaner 使用 [MIT License](LICENSE)