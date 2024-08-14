use std::io;

use find_your_lover::candidate::{self, Candidate};

fn main() {
    let mut candidates = Vec::new();

    loop {
        println!("请输入相亲对象的名字：");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("读取输入失败");
        let name = name.trim().to_string();

        println!("请输入{}的年龄：", name);
        let mut age_input = String::new();
        io::stdin().read_line(&mut age_input).expect("读取输入失败");
        let age: u8 = age_input.trim().parse().expect("请输入有效的年龄");

        let attributes = candidate::Attribute::input_attributes();

        let candidate = Candidate::new(name, age, attributes);

        let total_score = candidate.calculate_score();
        println!("{}的总评分为：{:.2}/100", candidate.name, total_score);
        candidates.push(candidate);

        println!("目前所有输入对象的评分为：");
        for candidate in &candidates {
            println!(
                "{} ({}岁): {:.2}/100",
                candidate.name,
                candidate.age,
                candidate.calculate_score()
            );
        }

        println!("是否继续输入下一个相亲对象？(y/n)");
        let mut continue_input = String::new();
        io::stdin()
            .read_line(&mut continue_input)
            .expect("读取输入失败");
        if continue_input.trim().to_lowercase() != "y" {
            break;
        }
    }
}
#[cfg(test)]
mod modtest {
    use candidate::Attribute;

    use super::*;

    // 测试函数，展示测试数据
    fn generate_test_candidate() -> Candidate {
        let attributes = Attribute {
            appearance: 80,
            personality: 85,
            financial_stability: 90,
            sense_of_humor: 70,
            common_interests: 75,
            family_background: 80,
            health: 85,
            lifestyle: 75,
            career: 90,
            education: 95,
        };

        Candidate::new("测试用户".to_string(), 30, attributes)
    }

    #[test]
    fn test_calculate_score() {
        let candidate = generate_test_candidate();
        let total_score = candidate.calculate_score();

        println!(
            "候选人: {}, 年龄: {}, 总评分: {:.2}/100",
            candidate.name, candidate.age, total_score
        );

        // 断言测试总评分是否为期望值
        assert!(total_score > 80.0 && total_score < 90.0);
    }
}
