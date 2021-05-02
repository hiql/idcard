# Chinese Identity Card Utilities

身份证号码解析校验工具

## 功能特点

- 15位升18位
- 号码有效性校验
- 解析号码相关信息（性别/年龄/出生年月日/省份/生肖等）
- 归属地信息查询
- 支持港澳台身份证号码校验
- 生成18位身份证号码

## API

### 号码信息

```rust

use idcard::Identity;

let id = Identity::new("632123820927051");

// 18位号码
id.number();
// 性别
id.gender(); 
// 计算年龄 
id.age(); 
// 出生年份
id.year(); 
// 出生月份
id.month();
// 出生日期 
id.date(); 
// 出生年月日（格式：yyyy-mm-dd)
id.birth_date(); 
// 天干地支
id.chinese_era(); 
// 生肖
id.chinese_zodiac(); 
// 星座
id.constellation(); 
// 省份
id.province(); 
// 号码归属地
id.region(); 
// 判断号码是否有效
id.is_valid();
// 判断号码是否为空
id.is_empty();
// 号码长度
id.len(); 

```

### 港澳台身份证

```rust

use idcard::{hk, mo, tw};

// 香港身份证校验
hk::validate("G123456(A)");

// 澳门身份证校验
mo::validate("1123456(0)");

// 台湾身份证校验
tw::validate("A123456789");

```

### 号码归属地

```rust

use idcard::region;

// 查询归属地名称
region::query("632123");

```

### 全局方法

```rust

// 15位升18位
idcard::upgrade("632123820927051");

// 15/18位号码校验
idcard::validate("632123820927051");

// 生成假身份证号码
idcard::new_fake("654325", 2018, 2, 28， Gender::Male);

// 随机生成假身份证号码
idcard::rand_fake();

// 根据参数随机生成假身份证号码
let mut opts = FakeOptions::default();
opts.set_region("3301");
opts.set_gender(Gender::Female);
opts.set_min_year(1990);
opts.set_max_year(2000);
idcard::rand_fake_with_opts(&opts);

```

## 资料来源

部分算法代码参考于网络

## License

MIT