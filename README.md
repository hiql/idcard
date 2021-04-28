# Identity Card Utilities

身份证号码解析校验工具

## 功能特点

- 15位升18位
- 号码有效性校验
- 解析号码相关信息（性别/年龄/出生年月日/省份/生肖等）
- 归属地信息查询
- 支持港澳台身份证号码校验

## API

### 号码信息解析

```rust

use idcard::Identity;

let id = Identity::new("632123820927051");

// 18位号码
id.number();

// 性别
id.gender(); 

// 当前年龄 
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

// 号码是否有效
id.is_valid();

// 号码是否为空
id.is_empty();

// 号码长度
id.len(); 

```

### 港澳台身份证

```rust

// 香港身份证校验
idcard::hk::validate("G123456(A)");

// 澳门身份证校验
idcard::mo::validate("1123456(0)");

// 台湾身份证校验
idcard::tw::validate("A123456789");

```

### 号码归属地查询

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

// 生成一个有效的虚假身份证号码
idcard::new_fake("654325", 2018, 2, 28， Gender::Male);

```

## 资料来源

部分算法代码参考于网络

## License

MIT
