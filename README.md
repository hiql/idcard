# Chinese Identity Card Utilities

中国身份证号码解析校验工具

🎉 **归属地信息已更新至 2022 年(民政部官方数据)**

## 特点

- 号码 15 位升 18 位
- 号码有效性校验
- 解析号码相关信息
- 查询号码归属地信息
- 支持港澳台身份证号码校验
- 生成 18 位身份证号码

## 用法

Cargo.toml 中添加依赖：

```toml
[dependencies]
idcard = "0.3"
```

## 例子

### 解析号码信息

```rust
use idcard::Identity;

let id = Identity::new("632123820927051");

id.number(); // 18位号码
id.gender(); // 性别
id.age(); // 当前年龄
id.age_in_year(2020); // 计算相对年龄
id.year(); // 出生年份
id.month(); // 出生月份
id.day(); // 出生日
id.birth_date(); // 出生年月日（格式：yyyy-mm-dd)
id.chinese_era(); // 天干地支
id.chinese_zodiac(); // 生肖
id.constellation(); // 星座
id.province(); // 省份
id.region(); // 号码归属地
id.region_code(); // 归属地代码
id.is_valid(); // 判断号码是否有效
id.is_empty(); // 判断号码是否为空
id.len(); // 号码长度
```

### 港澳台身份证

```rust
use idcard::{hk, mo, tw};

// 香港身份证
hk::validate("G123456(A)");

// 澳门身份证
mo::validate("1123456(0)");

// 台湾身份证
tw::validate("A123456789");
```

### 查询号码归属地

```rust
use idcard::region;

region::query("632123");
```

### 生成身份证号码

```rust
use idcard::{fake, Gender};

// 生成身份证号码
fake::new("654325", 2018, 2, 28， Gender::Male);

// 随机生成身份证号码
fake::rand();

// 根据参数随机生成身份证号码
let opts = fake::FakeOptions::new()
    .region("3301")
    .min_year(1990)
    .max_year(2000)
    .female();
fake::rand_with(&opts);
```

### 其它方法

```rust
// 15位号码升18位
idcard::upgrade("632123820927051");

// 15/18位号码校验
idcard::validate("632123820927051");

// 返回年份对应的生肖
idcard::chinese_zodiac(2021);

// 返回年份对应的天干地支
idcard::chinese_era(2021);

// 返回月日对应的星座
idcard::constellation(2, 29);
```

## 资料来源

部分算法代码参考于网络

License MIT
