# Chinese Identity Card Utilities

中国身份证号码解析校验工具

## 特点

- 号码15位升18位
- 号码有效性校验
- 解析号码相关信息
- 查询号码归属地信息
- 支持港澳台身份证号码校验
- 生成18位身份证号码

## 用法

Cargo.toml中添加依赖：

```toml
[dependencies]
idcard = "0.2"

```

## 例子

### 解析号码信息

```rust
use idcard::Identity;

let id = Identity::new("632123820927051");

// 18位号码
id.number();
// 性别
id.gender();
// 当前年龄 
id.age(); 
// 计算相对年龄 
id.age_in_year(2020); 
// 出生年份
id.year(); 
// 出生月份
id.month();
// 出生日
id.day(); 
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
// 归属地代码
id.region_code(); 
// 判断号码是否有效
id.is_valid();
// 判断号码是否为空
id.is_empty();
// 号码长度
id.len(); 

// 返回JSON格式字符串
id.to_json_string(true);

```

JSON字符串格式:

```json
{
    "number": "511702198002221308",
    "gender": "Female",
    "birthDate": "1980-02-22",
    "year": 1980,
    "month": 2,
    "day": 22,
    "age": 41,
    "province": "四川",
    "region": "四川省达州市通川区",
    "regionCode": "511702",
    "chineseEra": "庚申",
    "chineseZodiac": "猴",
    "constellation": "双鱼座",
    "isValid": true
}

{
    "number": "51170280022213X",
    "isValid": false
}

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
tw::gender("A123456789");
tw::region("A123456789");

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
    .gender(Gender::Female);
fake::rand_with_opts(&opts);

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

## License

MIT
